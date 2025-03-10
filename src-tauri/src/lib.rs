mod config;
mod discord;
mod grpc;
mod ping;

use declarative_discord_rich_presence::DeclarativeDiscordIpcClient;
use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
};
use tauri::{App, Manager};
use tauri_plugin_prevent_default::WindowsOptions;
use tauri_plugin_store::StoreExt;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(prevent_default())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_system_info::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_single_instance::init(|app, args, cwd| {
            let _ = args;
            let _ = cwd;
            let _ = app
                .get_webview_window("main")
                .expect("no main window")
                .set_focus();
        }))
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            ping::ping,
            grpc::auth,
            grpc::get_servers,
            grpc::get_profile,
            discord::set_activity
        ])
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let window = app.get_webview_window("main").unwrap();
                window.open_devtools();
            }
            
            start_discord_ipc(app);
            default_store(app);
            spawn_tray_icon(app);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn prevent_default() -> tauri::plugin::TauriPlugin<tauri::Wry> {
    tauri_plugin_prevent_default::Builder::new()
        .platform(WindowsOptions {
            general_autofill: false,
            password_autosave: false,
        })
        .build()
}

fn default_store(app: &mut App) {
    let store = app.store("config.json").unwrap();
    if store.is_empty() {
        store.set("dir", "TEST");
        store.set("autoConnect", false);
        store.set("fullScreen", false);
        store.set("useMemory", 1024);
        store.set("startDebug", false);
    }
}

fn start_discord_ipc(app: &mut App) {
    let discord_ipc_client = DeclarativeDiscordIpcClient::new(config::DISCORD.app_id);
    discord_ipc_client.enable();
    app.manage(discord_ipc_client);
}

fn spawn_tray_icon(app: &mut App) {
    let open_i = MenuItem::with_id(app, "open", "Показать окно", true, None::<&str>).unwrap();
    let quit_i = MenuItem::with_id(app, "quit", "Закрыть", true, None::<&str>).unwrap();
    let menu = Menu::with_items(app, &[&open_i, &quit_i]).unwrap();
    let _tray = TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "quit" => {
                app.exit(0);
            }
            "open" => {
                let _ = app
                    .get_webview_window("main")
                    .expect("no main window")
                    .set_focus();
            }
            _ => {
                println!("menu item {:?} not handled", event.id);
            }
        })
        .build(app)
        .unwrap();
}
