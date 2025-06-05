use crate::grpc;
use crate::updater;
use crate::StorageData;
use std::sync::Mutex;
use tauri::State;

#[tauri::command]
pub async fn start_game(
    state: State<'_, Mutex<StorageData>>,
    server: grpc::proto::Server,
) -> Result<(), ()> {
    let profile = grpc::get_profile(server.profile_uuid).await.unwrap();
    log::info!("Assets downloading...");
    updater::download_assets(state.clone(), profile.asset_index).await;
    log::info!("Libraries downloading...");
    let download_lib = updater::download_libraries(state.clone(), profile.libraries).await;
    log::info!("Game files downloading...");
    updater::download_game_files(state.clone(), profile.client_dir).await;
    log::info!("Java downloading...");
    updater::download_java(state.clone(), profile.java_version).await;
    log::info!("Auth injector downloading...");
    updater::download_auth_injector(state.clone()).await;
    Ok(())
}


fn start_process(state: State<'_, Mutex<StorageData>>) {

}