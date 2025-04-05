use crate::config;
use declarative_discord_rich_presence::{
    DeclarativeDiscordIpcClient,
    activity::{Activity, Assets, Button, Timestamps},
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
                        .state(config::DISCORD.default.state)
                        .details(config::DISCORD.default.details)
                        .buttons(Vec::from([Button::new(
                            "Прекол".to_string(),
                            "https://youtu.be/dQw4w9WgXcQ".to_string(),
                        )]))
                        .assets(
                            Assets::new()
                                .large_image(config::DISCORD.default.large_image)
                                .large_text(config::DISCORD.default.large_text)
                                .small_image(config::DISCORD.default.smoll_image)
                                .small_text(config::DISCORD.default.small_text),
                        )
                        .timestamps(Timestamps::new().start(epoch_secs)),
                )
                .unwrap();
        }
        "profile" => {
            discord_ipc_client
                .set_activity(
                    Activity::new()
                        .state(config::DISCORD.profile.state)
                        .details(config::DISCORD.profile.details)
                        .buttons(Vec::from([Button::new(
                            "Прекол".to_string(),
                            "https://youtu.be/dQw4w9WgXcQ".to_string(),
                        )]))
                        .assets(
                            Assets::new()
                                .large_image(config::DISCORD.profile.large_image)
                                .large_text(config::DISCORD.profile.large_text)
                                .small_image(config::DISCORD.profile.smoll_image)
                                .small_text(config::DISCORD.profile.small_text),
                        )
                        .timestamps(Timestamps::new().start(epoch_secs)),
                )
                .unwrap();
        }
        "game" => {
            discord_ipc_client
                .set_activity(
                    Activity::new()
                        .state(config::DISCORD.game.state)
                        .details(config::DISCORD.game.details)
                        .buttons(Vec::from([Button::new(
                            "Прекол".to_string(),
                            "https://youtu.be/dQw4w9WgXcQ".to_string(),
                        )]))
                        .assets(
                            Assets::new()
                                .large_image(config::DISCORD.game.large_image)
                                .large_text(config::DISCORD.game.large_text)
                                .small_image(config::DISCORD.game.smoll_image)
                                .small_text(config::DISCORD.game.small_text),
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
