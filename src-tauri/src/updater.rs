use crate::config;
use crate::StorageData;
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
        let test = &object.1["hash"].as_str().unwrap()[0..2];
        let object_url = format!(
            "{}/files/assets/objects/{}/{}",
            config::IP_WEB,
            &test,
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

        create_dir_all(assets_dir.clone().join("objects").join(test)).unwrap();
        write(
            assets_dir
                .clone()
                .join("objects")
                .join(test)
                .join(object.1["hash"].as_str().unwrap()),
            resp,
        )
        .unwrap();
    }
}
