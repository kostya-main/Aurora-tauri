# Подготовка к разработке

Для работы на Windows вам понадобится установить:
- [Rust](https://www.rust-lang.org/tools/install)
- [Bun](https://bun.sh/)
- [Git](https://git-scm.com/downloads)
- [MSBuilder](https://visualstudio.microsoft.com/visual-cpp-build-tools/) с включённым модулем "Разработка классических приложений на C++"

Так же у вас уже должен быть настроен модифицированный LauncherServer.\
Если вы работаете на другой OC то посетите офф. сайт [Tauri](https://v2.tauri.app/start/prerequisites/) для уточнения какие библиотеки вам нужны.

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

Если вам нужен исполняемый файл то выполните команду:
```bash
bun run build
```
Скомпилированный файл будет лежать в папке `src-tauri/target/release`.
Версии инсталеров лежат в папке `bundle`.

Конфигурация приложения находится в файле [config.json](https://github.com/kostya-main/Aurora-tauri/blob/master/config.json).