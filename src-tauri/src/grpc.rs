use proto::aurora_launcher_service_client::AuroraLauncherServiceClient;
use proto::AuthRequest;
use proto::ProfileRequest;
use proto::UpdateRequest;
use crate::config;
pub mod proto {
    tonic::include_proto!("aurora_launcher.rpc");
}

#[tauri::command]
pub async fn auth(login: String, password: String) -> Result<proto::AuthResponse, String> {
    let mut client = AuroraLauncherServiceClient::connect(config::IP).await.unwrap();

    let request = tonic::Request::new(AuthRequest { login, password });

    let response = client.auth(request).await.unwrap();

    Ok(response.into_inner())
}

#[tauri::command]
pub async fn get_profile(uuid: String) -> Result<proto::ProfileResponse, String> {
    let mut client = AuroraLauncherServiceClient::connect(config::IP).await.unwrap();

    let request = tonic::Request::new(ProfileRequest { uuid });

    let response = client.get_profile(request).await.unwrap();

    Ok(response.into_inner())
}

#[tauri::command]
pub async fn get_servers() -> Result<proto::ServersResponse, String> {
    let mut client = AuroraLauncherServiceClient::connect(config::IP).await.unwrap();

    let response = client.get_servers(()).await.unwrap();

    Ok(response.into_inner())
}

#[tauri::command]
pub async fn get_update(dir: String) -> Result<proto::UpdateResponse, String> {
    let mut client = AuroraLauncherServiceClient::connect(config::IP).await.unwrap();

    let request = tonic::Request::new(UpdateRequest { dir });

    let response = client.get_updates(request).await.unwrap();

    Ok(response.into_inner())
}

pub async fn get_token() -> Result<proto::VerifyResponse, String> {
    let mut client = AuroraLauncherServiceClient::connect(config::IP).await.unwrap();

    let response = client.get_token(()).await.unwrap();

    Ok(response.into_inner())
}