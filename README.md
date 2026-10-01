# Luassgate — Telegram-бот (Rust + teloxide)

Telegram-бот, который выдаёт Steam-манифесты по AppID. Мультиязычный: язык выбирается кнопками после `/start`.

## Команды

| Команда | Что делает |
|---|---|
| `/start` | Приветствие + выбор языка (кнопки) |
| `/id <AppID>` | Скачать manifest-zip для игры, например `/id 1962700` |
| (просто число) | Если вызвать `/id` без аргумента, бот попросит ввести AppID следующим сообщением |

Бот подтягивает реальное название игры из Steam Store API и присылает файл вида `1962700; Название.zip`.

## Переменные окружения

| Переменная | Обязательна | Описание |
|---|---|---|
| `TELOXIDE_TOKEN` | да | Токен бота от [@BotFather](https://t.me/BotFather) |
| `RUST_LOG` | нет | Уровень логов, по умолчанию `info` |
| `MANIFEST_COOKIE` | нет | Cookie сессии для manifest-API (если API режет запросы без сессии браузера) |
| `MANIFEST_API_URL` | нет | Кастомный URL manifest-API. Можно с плейсхолдером `{app_id}`, иначе AppID дописывается в конец. По умолчанию используется встроенный R2-URL |

## Локальный запуск

```bash
cp .env.example .env   # вписать TELOXIDE_TOKEN
cargo run --release
```

## Docker локально

```bash
docker build -t luassgate .
docker run --rm --env-file .env luassgate
# или
docker compose up --build
```

## Деплой на Railway

1. Запушьте код на GitHub так, чтобы в корне репозитория лежали `Cargo.toml`, `Dockerfile`, `railway.toml` и папка `src/` (как сейчас).
2. В Railway: **New Project → Deploy from GitHub** → выберите репозиторий.
3. Railway сам найдёт `Dockerfile` (см. `railway.toml`, builder `DOCKERFILE`).
4. Во вкладке **Variables** добавьте:
   - `TELOXIDE_TOKEN` — токен от @BotFather;
   - при необходимости `MANIFEST_COOKIE` / `MANIFEST_API_URL`.
5. Нажмите **Deploy**. Логи должны показать `Starting Telegram Bot...`.
6. Это polling-бот (входящие HTTP-запросы не принимает), поэтому:
   - **не добавляйте домен** (Generate Domain не нужен);
   - **не включайте healthcheck по HTTP** — сервис работает как worker.

Пересборка происходит автоматически при каждом пуше в подключённую ветку.

## Структура

```
.
├── Dockerfile          # multi-stage сборка (rust → debian-slim, non-root user)
├── railway.toml        # builder DOCKERFILE + restart policy
├── docker-compose.yml  # локальный запуск через Docker
├── .env.example        # шаблон переменных окружения
└── src/
    ├── main.rs         # точка входа, dispatcher teloxide
    ├── handlers.rs     # команды /start, /id, колбэки языка
    ├── localization.rs # 8 языков (en/es/tr/ru/de/fr/en_UK/en_NZ)
    ├── steam.rs        # Steam Store API + скачивание manifest-zip
    └── media.rs        # (пока не подключён) заготовки под yt-dlp
```

## Заметки

- `RUST_LOG` по умолчанию выставляется в `info`, если переменная не задана.
- Если `TELOXIDE_TOKEN` не задан, бот сразу завершается с понятной ошибкой в логах (а не паникой teloxide).
- `src/media.rs` сейчас нигде не используется (модуль не подключён в `main.rs`) — оставлен как заготовка.
