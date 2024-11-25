use mc_query::status;

#[derive(serde::Serialize)]
pub struct PingResponse {
    online: u32,
    max: u32,
}

#[tauri::command]
pub async fn ping(host: String, port: u16) -> Result<PingResponse, ()> {
    let res = status(&host, port).await.unwrap();
    
    Ok(PingResponse {
        online: res.players.online,
        max: res.players.max,
    })
}