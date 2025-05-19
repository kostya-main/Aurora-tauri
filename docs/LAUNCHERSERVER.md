# Сборка и запуск LauncherServer

Для сборки вам понадобится установить:
- [Node.js](https://nodejs.org/en/)
- [Git](https://git-scm.com/downloads)

## Загрузка репозитория

Для загрузки репозитория себе на компьютер просто выполните команду:
```bash
git clone https://github.com/AuroraTeam/AuroraLauncher.git -b features/new-net-steck
```

## Загрузка зависимостей

Для начала работы необходимо загрузить все зависимости проекта с помощью команды:
```bash
npm install
```

## Сборка

Для сборки проекта необходимо выполнить команду:
```bash
npm run build:libs
npm run build:prod -w packages/server
npm run build:bin -w packages/server
```

После этого вы проходите в папку `packages\server\dist` в котором будут лежать скомпилированный LauncherServer.