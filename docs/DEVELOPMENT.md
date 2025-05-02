# Подготовка к разработке

Для работы вам понадобится установить:
- [Rust](https://www.rust-lang.org/tools/install)
- [Bun](https://bun.sh/)
- [Git](https://git-scm.com/downloads)
- [MSBuilder](https://visualstudio.microsoft.com/visual-cpp-build-tools/) с включённым модулем "Разработка классических приложений на C++"

Так же у вас уже должен быть настроен модифицированный LauncherServer.

## Загрузка репозитория

Для загрузки репозитория себе на компьютер просто выполните команду:
```bash
git clone https://github.com/kostya-main/Aurora-tauri.git
```

## Загрузка зависимостей

Для начала работы необходимо загрузить все зависимости проекта с помощью команды:
```bash
bun install
```

## Запуск приложения

Для запуска приложения необходимо выполнить команду:
```bash
bun dev
```

Конфигурация приложения находится в файле `src-tauri\src\config.rs`.