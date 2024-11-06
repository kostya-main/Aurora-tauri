use proto::aurora_launcher_service_client::AuroraLauncherServiceClient;
use proto::AuthRequest;

pub mod proto {
    tonic::include_proto!("aurora_launcher.rpc");
}

#[tauri::command]
pub async fn auth(login: String, password: String) -> Result<proto::AuthResponse , String> {
    let mut client = AuroraLauncherServiceClient::connect("http://127.0.0.1:1371").await.unwrap();

    let request = tonic::Request::new(AuthRequest {
        login,
        password,
    });

    let response = client.auth(request).await.unwrap();

    Ok(response.into_inner())
}