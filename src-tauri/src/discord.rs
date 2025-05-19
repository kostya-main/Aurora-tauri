use crate::config::CONFIG;
use declarative_discord_rich_presence::{
    activity::{Activity, Assets, Button, Timestamps},
    DeclarativeDiscordIpcClient,
};
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::State;

#[tauri::command]
pub fn set_activity(discord_ipc_client: State<'_, DeclarativeDiscordIpcClient>, status: &str) {
    let epoch_secs: i64 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
        .try_into()
        .unwrap();

    match status {
        "default" => {
            discord_ipc_client
                .set_activity(
                    Activity::new()
                        .state(CONFIG.discord.default.state.as_str())
                        .details(CONFIG.discord.default.details.as_str())
                        .buttons(Vec::from([Button::new(
                            "Прекол".to_string(),
                            "https://youtu.be/dQw4w9WgXcQ".to_string(),
                        )]))
                        .assets(
                            Assets::new()
                                .large_image(CONFIG.discord.default.large_image.as_str())
                                .large_text(CONFIG.discord.default.large_text.as_str())
                                .small_image(CONFIG.discord.default.smoll_image.as_str())
                                .small_text(CONFIG.discord.default.small_text.as_str()),
                        )
                        .timestamps(Timestamps::new().start(epoch_secs)),
                )
                .unwrap();
        }
        "profile" => {
            discord_ipc_client
                .set_activity(
                    Activity::new()
                        .state(CONFIG.discord.profile.state.as_str())
                        .details(CONFIG.discord.profile.details.as_str())
                        .buttons(Vec::from([Button::new(
                            "Прекол".to_string(),
                            "https://youtu.be/dQw4w9WgXcQ".to_string(),
                        )]))
                        .assets(
                            Assets::new()
                                .large_image(CONFIG.discord.profile.large_image.as_str())
                                .large_text(CONFIG.discord.profile.large_text.as_str())
                                .small_image(CONFIG.discord.profile.smoll_image.as_str())
                                .small_text(CONFIG.discord.profile.small_text.as_str()),
                        )
                        .timestamps(Timestamps::new().start(epoch_secs)),
                )
                .unwrap();
        }
        "game" => {
            discord_ipc_client
                .set_activity(
                    Activity::new()
                        .state(CONFIG.discord.game.state.as_str())
                        .details(CONFIG.discord.game.details.as_str())
                        .buttons(Vec::from([Button::new(
                            "Прекол".to_string(),
                            "https://youtu.be/dQw4w9WgXcQ".to_string(),
                        )]))
                        .assets(
                            Assets::new()
                                .large_image(CONFIG.discord.game.large_image.as_str())
                                .large_text(CONFIG.discord.game.large_text.as_str())
                                .small_image(CONFIG.discord.game.smoll_image.as_str())
                                .small_text(CONFIG.discord.game.small_text.as_str()),
                        )
                        .timestamps(Timestamps::new().start(epoch_secs)),
                )
                .unwrap();
        }
        _ => {
            discord_ipc_client.disable();
        }
    }
}
