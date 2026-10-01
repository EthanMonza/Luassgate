# Luassgate — Telegram bot (Rust + teloxide)

A Telegram bot that serves Steam manifests by AppID. Multilingual: the language is picked via buttons after `/start`.

## Commands

| Command | What it does |
|---|---|
| `/start` | Greeting + language picker (inline buttons) |
| `/id <AppID>` | Download the manifest zip for a game, e.g. `/id 1962700` |
| (just a number) | If `/id` is sent without an argument, the bot asks for the AppID in the next message |

The bot resolves the real game name via the Steam Store API and sends a file like `1962700; Game Name.zip`.

## Environment variables

| Variable | Required | Description |
|---|---|---|
| `TELOXIDE_TOKEN` | yes | Bot token from [@BotFather](https://t.me/BotFather) |
| `RUST_LOG` | no | Log level, defaults to `info` |
| `MANIFEST_COOKIE` | no | Session cookie for the manifest API (if the API blocks requests without a browser session) |
| `MANIFEST_API_URL` | no | Custom manifest API URL. Supports the `{app_id}` placeholder, otherwise the AppID is appended at the end. Falls back to the built-in R2 URL |

## Run locally

```bash
cp .env.example .env   # put your TELOXIDE_TOKEN in there
cargo run --release
```

## Docker locally

```bash
docker build -t luassgate .
docker run --rm --env-file .env luassgate
# or
docker compose up --build
```

## Deploy to Railway

1. Push the code to GitHub so that `Cargo.toml`, `Dockerfile`, `railway.toml`, and the `src/` folder sit at the repository root (as they do now).
2. In Railway: **New Project → Deploy from GitHub** → pick the repository.
3. Railway picks up the `Dockerfile` automatically (see `railway.toml`, builder `DOCKERFILE`).
4. Under **Variables**, add:
   - `TELOXIDE_TOKEN` — token from @BotFather;
   - optionally `MANIFEST_COOKIE` / `MANIFEST_API_URL`.
5. Hit **Deploy**. The logs should show `Starting Telegram Bot...`.
6. This is a polling bot (it accepts no inbound HTTP traffic), so:
   - **don't add a domain** (Generate Domain is not needed);
   - **don't enable an HTTP healthcheck** — the service runs as a worker.

Railway rebuilds automatically on every push to the connected branch.

## Layout

```
.
├── Dockerfile          # multi-stage build (rust → debian-slim, non-root user)
├── railway.toml        # DOCKERFILE builder + restart policy
├── docker-compose.yml  # local Docker run
├── .env.example        # environment variable template
└── src/
    ├── main.rs         # entry point, teloxide dispatcher
    ├── handlers.rs     # /start and /id commands, language callbacks
    ├── localization.rs # 8 locales (en/es/tr/ru/de/fr/en_UK/en_NZ)
    ├── steam.rs        # Steam Store API + manifest-zip download
    └── media.rs        # (not wired up yet) yt-dlp scaffolding
```

## Notes

- `RUST_LOG` falls back to `info` when the variable is not set.
- If `TELOXIDE_TOKEN` is missing, the bot exits right away with a clear log message instead of a teloxide panic.
- `src/media.rs` is currently unused (the module is not wired into `main.rs`) — kept as scaffolding.
