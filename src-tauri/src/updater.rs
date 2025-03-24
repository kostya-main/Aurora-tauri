use crate::config;
use crate::grpc;
use crate::StorageData;
use reqwest::Url;
use serde_json::Value;
use std::fs::create_dir_all;
use std::fs::write;
use std::sync::Mutex;
use tauri::State;

pub async fn download_assets(state: State<'_, Mutex<StorageData>>, assets_index: String) {
    let asset_url = format!(
        "{}/files/assets/indexes/{}.json",
        config::IP_WEB,
        assets_index
    );
    let client = reqwest::Client::new();
    let resp = client
        .get(asset_url)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let assets_dir;
    {
        let state = state.lock().unwrap();
        assets_dir = state.assets_dir.to_owned();
    }
    create_dir_all(assets_dir.clone().join("indexes")).unwrap();
    write(
        assets_dir
            .clone()
            .join("indexes")
            .join(format!("{}.json", assets_index)),
        resp.as_bytes(),
    )
    .unwrap();
    let deserialized: Value = serde_json::from_str(&resp).unwrap();

    for object in deserialized["objects"].as_object().unwrap() {
        let dir_hash = &object.1["hash"].as_str().unwrap()[0..2];
        let object_url = format!(
            "{}/files/assets/objects/{}/{}",
            config::IP_WEB,
            &dir_hash,
            &object.1["hash"]
        );
        let resp = client
            .get(object_url)
            .send()
            .await
            .unwrap()
            .bytes()
            .await
            .unwrap();

        create_dir_all(assets_dir.clone().join("objects").join(dir_hash)).unwrap();
        write(
            assets_dir
                .clone()
                .join("objects")
                .join(dir_hash)
                .join(object.1["hash"].as_str().unwrap()),
            resp,
        )
        .unwrap();
    }
}

pub async fn download_game_files(
    state: State<'_, Mutex<StorageData>>,
    profile: grpc::proto::ProfileResponse,
) {
    let game_hash = grpc::get_updates(profile.client_dir.clone()).await.unwrap();
    let game_url = format!("{}/files/clients", config::IP_WEB);
    let client = reqwest::Client::new();
    let mut clients_dir;
    {
        let state = state.lock().unwrap();
        clients_dir = state.clients_dir.to_owned();
    }
    for file in game_hash.hashed_file {
        let url = Url::parse(&(game_url.as_str().to_owned() + file.path.as_str())).unwrap();
        let resp = client.get(url).send().await.unwrap().bytes().await.unwrap();
        file.path
            .split(&prefix_read(file.path.chars().nth(0).unwrap()))
            .collect::<Vec<&str>>()
            .iter()
            .for_each(|f| {
                clients_dir.push(f);
            });
        create_dir_all(clients_dir.clone().parent().unwrap()).unwrap();
        write(&clients_dir, resp).unwrap();
    }
}

fn prefix_read(start_char: char) -> String {
    match start_char {
        '\\' => {
            return "\\".to_string();
        }
        '/' => {
            return "/".to_string();
        }
        _ => todo!(),
    }
}
