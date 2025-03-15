pub const IP_GRPC: &str = "http://127.0.0.1:1371";
pub const IP_WEB: &str = "http://127.0.0.1:1370";
pub const STORAGE: &str = ".aurora-launcher";
pub const DISCORD: DiscordRPC = DiscordRPC {
    app_id: "1214685301793103902",
    default: Activity {
        state: "Тестирую лаунчер",
        details: "Чувак, ты думал здесь что-то будет?",
        large_image: "logo",
        smoll_image: "logo_mc",
        large_text: "Aurora Launcher",
        small_text: "Minecraft",
    },
    profile: Activity {
        state: "Выбираю тестируемый профиль игры",
        details: "Загружаю {server}",
        large_image: "logo",
        smoll_image: "logo_mc",
        large_text: "Aurora Launcher",
        small_text: "Minecraft",
    },
    game: Activity {
        state: "Играю на тестовом сервере",
        details: "Играю за {nickname}",
        large_image: "logo",
        smoll_image: "logo_mc",
        large_text: "Aurora Launcher",
        small_text: "Minecraft",
    },
};

pub struct DiscordRPC<'a> {
    pub app_id: &'a str,
    pub default: Activity<'a>,
    pub profile: Activity<'a>,
    pub game: Activity<'a>,
}

pub struct Activity<'a> {
    pub state: &'a str,
    pub details: &'a str,
    pub large_image: &'a str,
    pub smoll_image: &'a str,
    pub large_text: &'a str,
    pub small_text: &'a str,
}
