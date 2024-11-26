mod grpc;
mod ping;
mod discord;

use declarative_discord_rich_presence::DeclarativeDiscordIpcClient;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_single_instance::init(|app, args, cwd| {
            let _ = args;
            let _ = cwd;
            let _ = app
                .get_webview_window("main")
                .expect("no main window")
                .set_focus();
        }))
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![ping::ping, grpc::auth, grpc::get_servers, grpc::get_profile, discord::set_activity])
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let window = app.get_webview_window("main").unwrap();
                window.open_devtools();
            }
            let discord_ipc_client = DeclarativeDiscordIpcClient::new("1214685301793103902");

            discord_ipc_client.enable();
            app.manage(discord_ipc_client);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
