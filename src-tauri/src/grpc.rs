use crate::config;
use proto::aurora_launcher_service_client::AuroraLauncherServiceClient;
use proto::AuthRequest;
use proto::ProfileRequest;
use proto::UpdateRequest;
pub mod proto {
    tonic::include_proto!("aurora_launcher.rpc");
}

#[tauri::command]
pub async fn auth(login: String, password: String) -> Result<proto::AuthResponse, String> {
    let mut client = AuroraLauncherServiceClient::connect(config::IP_GRPC)
        .await
        .map_err(|err| err.to_string())?;

    let request = tonic::Request::new(AuthRequest { login, password });

    let response = client.auth(request).await.map_err(|err| err.to_string())?;

    Ok(response.into_inner())
}

#[tauri::command]
pub async fn get_servers() -> Result<proto::ServersResponse, String> {
    let mut client = AuroraLauncherServiceClient::connect(config::IP_GRPC)
        .await
        .map_err(|err| err.to_string())?;

    let response = client
        .get_servers(())
        .await
        .map_err(|err| err.to_string())?;

    Ok(response.into_inner())
}

pub async fn get_profile(uuid: String) -> Result<proto::ProfileResponse, String> {
    let mut client = AuroraLauncherServiceClient::connect(config::IP_GRPC)
        .await
        .map_err(|err| err.to_string())?;

    let request = tonic::Request::new(ProfileRequest { uuid });

    let response = client
        .get_profile(request)
        .await
        .map_err(|err| err.to_string())?;

    Ok(response.into_inner())
}

pub async fn get_updates(dir: String) -> Result<proto::UpdateResponse, String> {
    let mut client = AuroraLauncherServiceClient::connect(config::IP_GRPC)
        .await
        .map_err(|err| err.to_string())?;

    let request = tonic::Request::new(UpdateRequest { dir });

    let response = client
        .get_updates(request)
        .await
        .map_err(|err| err.to_string())?;

    Ok(response.into_inner())
}

pub async fn get_token() -> Result<proto::VerifyResponse, String> {
    let mut client = AuroraLauncherServiceClient::connect(config::IP_GRPC)
        .await
        .map_err(|err| err.to_string())?;

    let response = client.get_token(()).await.map_err(|err| err.to_string())?;

    Ok(response.into_inner())
}
