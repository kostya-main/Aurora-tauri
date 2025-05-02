use crate::config;
use crate::grpc;
use crate::grpc::proto::ProfileLibrary;
use crate::matcher;
use crate::StorageData;
use reqwest::Url;
use serde::Deserialize;
use serde_json::Value;
use sha1::{Digest, Sha1};
use std::env::consts::ARCH;
use std::env::consts::OS;
use std::fmt::Write;
use std::fs::create_dir_all;
use std::fs::write;
use std::fs::File;
use std::io;
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
    create_dir_all(&assets_dir.join("indexes")).unwrap();
    write(
        &assets_dir
            .join("indexes")
            .join(format!("{}.json", assets_index)),
        resp.as_bytes(),
    )
    .unwrap();
    let deserialized: Value = serde_json::from_str(&resp).unwrap();

    for object in deserialized["objects"].as_object().unwrap() {
        let dir_hash = &object.1["hash"].as_str().unwrap()[0..2];
        let mut object_url = Url::parse(config::IP_WEB).unwrap();
        object_url = object_url
            .join(format!("{}/files/assets/objects/{}/", config::IP_WEB, &dir_hash,).as_str())
            .unwrap();
        object_url = object_url
            .join(&object.1["hash"].as_str().unwrap())
            .unwrap();
        let object_dir = assets_dir
            .clone()
            .join("objects")
            .join(dir_hash)
            .join(object.1["hash"].as_str().unwrap());
        if !object_dir.exists() {
            let resp = client
                .get(object_url)
                .send()
                .await
                .unwrap()
                .bytes()
                .await
                .unwrap();
            create_dir_all(&object_dir.parent().unwrap()).unwrap();
            write(&object_dir, resp).unwrap();
        }
    }
}

pub async fn download_libraries(
    state: State<'_, Mutex<StorageData>>,
    libraries: Vec<ProfileLibrary>,
) -> Vec<ProfileLibrary> {
    let lib: Vec<ProfileLibrary> = libraries
        .into_iter()
        .filter(|lib| matcher::match_lib(lib.rules.clone()))
        .collect();
    let lib_url = format!("{}/files/libraries/", config::IP_WEB);
    let client = reqwest::Client::new();
    let libraries_dir;
    {
        let state = state.lock().unwrap();
        libraries_dir = state.libraries_dir.to_owned();
    }
    println!("{:?}", libraries_dir);
    for file in &lib {
        let url = Url::parse(&(lib_url.as_str().to_owned() + file.path.as_str())).unwrap();
        let dir = libraries_dir.join(&file.path);
        if !dir.exists() {
            let resp = client.get(url).send().await.unwrap().bytes().await.unwrap();
            create_dir_all(&dir.parent().unwrap()).unwrap();
            write(&dir, resp).unwrap();
        }
    }
    lib
}

pub async fn download_game_files(state: State<'_, Mutex<StorageData>>, client_dir: String) {
    let game_hash = grpc::get_updates(client_dir).await.unwrap();
    let game_url = format!("{}/files/clients", config::IP_WEB);
    let client = reqwest::Client::new();
    let clients_dir;
    {
        let state = state.lock().unwrap();
        clients_dir = state.clients_dir.to_owned();
    }
    for file in game_hash.hashed_file {
        let url = Url::parse(&(game_url.as_str().to_owned() + file.path.as_str())).unwrap();
        let dir = clients_dir.join(&file.path[1..file.path.len()]);
        if !dir.exists() {
            let resp = client.get(url).send().await.unwrap().bytes().await.unwrap();
            create_dir_all(&dir.parent().unwrap()).unwrap();
            write(&dir, resp).unwrap();
        } else {
            let mut open = File::open(&dir).unwrap();
            let mut hasher = Sha1::new();
            io::copy(&mut open, &mut hasher).unwrap();
            let hash = bytes_to_hex(&hasher.finalize());
            if hash != file.sha1 {
                let resp = client.get(url).send().await.unwrap().bytes().await.unwrap();
                create_dir_all(&dir.parent().unwrap()).unwrap();
                write(&dir, resp).unwrap();
            }
        }
    }
}

pub async fn download_java(state: State<'_, Mutex<StorageData>>, java_version: i32) {
    let url = format!(
        "https://api.azul.com/metadata/v1/zulu/packages/?java_version={}&os={}&arch={}&archive_type=zip&java_package_type=jre&javafx_bundled=false&latest=true&release_status=ga&page=1&page_size=1",
        java_version,
        OS,
        arch(ARCH)
    );
    let client = reqwest::Client::new();
    let java_dir;
    {
        let state = state.lock().unwrap();
        java_dir = state.java_dir.join(java_version.to_string());
    }
    if !java_dir.exists() {
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

        println!("Unpacking Java");
        let mut archive = zip::ZipArchive::new(Cursor::new(resp)).unwrap();
        let export = archive.extract(java_dir);
        println!("{:?}", export);
    }
    println!("Java installed!");
}

fn arch(arch: &str) -> String {
    match arch {
        "x86" => "i686".to_string(),
        "x86_64" => "x64".to_string(),
        "aarch64" => "aarch64".to_string(),
        "arm" => "aarch64".to_string(),
        _ => todo!(),
    }
}

fn bytes_to_hex(bytes: &[u8]) -> String {
    let mut hex_string = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        write!(hex_string, "{:02x}", byte).unwrap();
    }
    hex_string
}

#[derive(Deserialize)]
struct JavaData {
    download_url: Url,
}
