use crate::StorageData;
use crate::grpc;
use crate::updater;
use std::sync::Mutex;
use tauri::State;

#[tauri::command]
pub async fn start_game(
    state: State<'_, Mutex<StorageData>>,
    server: grpc::proto::Server,
) -> Result<(), ()> {
    let profile = grpc::get_profile(server.profile_uuid).await.unwrap();
    println!("Assets downloading...");
    updater::download_assets(state.clone(), profile.asset_index).await;
    //println!("Libraries downloading...");
    //updater::download_libraries(state.clone(), profile.libraries).await;
    println!("Game files downloading...");
    updater::download_game_files(state.clone(), profile.client_dir).await;
    println!("Java downloading...");
    updater::download_java(state, profile.java_version).await;
    Ok(())
}
