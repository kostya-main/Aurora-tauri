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
    println!("start game");
    updater::download_assets(state, profile.asset_index).await;
    Ok(())
}
