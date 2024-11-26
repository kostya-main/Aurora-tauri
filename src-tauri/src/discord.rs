use declarative_discord_rich_presence::{activity::{Activity, Assets, Button, Timestamps}, DeclarativeDiscordIpcClient};
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::State;

#[tauri::command]
pub async fn set_activity(discord_ipc_client: State<'_, DeclarativeDiscordIpcClient>, status: String) -> Result<(), ()> {
    let epoch_secs: i64 = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .expect("Time went backwards")
    .as_secs().try_into().unwrap();


    if status == "default" {
        discord_ipc_client.set_activity(Activity::new()
            .state("Тестирую лаунчер")
            .details("Чувак, ты думал здесь что-то будет?")
            .buttons(Vec::from([Button::new("Прекол".to_string(), "https://youtu.be/dQw4w9WgXcQ".to_string())]))
            .assets(Assets::new()
                .large_image("logo")
                .large_text("Aurora Launcher")
                .small_image("logo_mc")
                .small_text("Minecraft")
            )
            .timestamps(Timestamps::new().start(epoch_secs))
        ).unwrap();
    }
    else if status == "profile" {
        discord_ipc_client.set_activity(Activity::new()
            .state("Выбираю тестируемый профиль игры")
            .details("Загружаю {server}")
            .buttons(Vec::from([Button::new("Прекол".to_string(), "https://youtu.be/dQw4w9WgXcQ".to_string())]))
            .assets(Assets::new()
                .large_image("logo")
                .large_text("Aurora Launcher")
                .small_image("logo_mc")
                .small_text("Minecraft")
            )
            .timestamps(Timestamps::new().start(epoch_secs))
        ).unwrap();
    }
    else if status == "game" {
        discord_ipc_client.set_activity(Activity::new()
            .state("Играю на тестовом сервере")
            .details("Играю за {nickname}")
            .buttons(Vec::from([Button::new("Прекол".to_string(), "https://youtu.be/dQw4w9WgXcQ".to_string())]))
            .assets(Assets::new()
                .large_image("logo")
                .large_text("Aurora Launcher")
                .small_image("logo_mc")
                .small_text("Minecraft")
            )
            .timestamps(Timestamps::new().start(epoch_secs))
        ).unwrap();
    }
    else {
        discord_ipc_client.disable();
    }
    Ok(())
}