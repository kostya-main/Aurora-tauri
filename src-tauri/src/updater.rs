use crate::config;
use crate::grpc;
use crate::StorageData;
use reqwest::Url;
use serde::Deserialize;
use serde_json::Value;
use std::env::consts::ARCH;
use std::env::consts::OS;
use std::fs::create_dir_all;
use std::fs::write;
use std::io::Cursor;
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
    client_dir: String,
) {
    let game_hash = grpc::get_updates(client_dir).await.unwrap();
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

pub async fn download_java(state: State<'_, Mutex<StorageData>>, java_version: i32) {
    let url = format!("https://api.azul.com/metadata/v1/zulu/packages/?java_version={}&os={}&arch={}&archive_type=zip&java_package_type=jre&javafx_bundled=false&latest=true&release_status=ga&page=1&page_size=1", java_version, OS, arch(ARCH));
    let client = reqwest::Client::new();
    let java_dir;
    {
        let state = state.lock().unwrap();
        java_dir = state.java_dir.join(java_version.to_string());
    }
    let resp = client
        .get(url)
        .send()
        .await
        .unwrap()
        .json::<Vec<JavaData>>()
        .await
        .unwrap();
    let resp = client
        .get(resp[0].download_url.clone())
        .send()
        .await
        .unwrap()
        .bytes()
        .await
        .unwrap();

    let mut archive = zip::ZipArchive::new(Cursor::new(resp)).unwrap();
    let export = archive.extract(java_dir);
    println!("{:?}", export);
}

fn arch(arch: &str) -> std::string::String {
    match arch {
        "x86" => "i686".to_string(),
        "x86_64" => "x64".to_string(),
        "aarch64" => "aarch64".to_string(),
        "arm" => "aarch64".to_string(),
        _ => todo!(),
    }
}

fn prefix_read(start_char: char) -> String {
    match start_char {
        '\\' => "\\".to_string(),
        '/' => "/".to_string(),
        _ => todo!(),
    }
}

#[derive(Deserialize)]
struct JavaData {
    download_url: Url,
}