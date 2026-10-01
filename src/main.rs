mod handlers;
mod localization;
mod steam;

use std::sync::Arc;
use teloxide::prelude::*;
use teloxide::utils::command::BotCommands;
use tokio::sync::Mutex;
use std::collections::HashMap;

use handlers::{command_handler, callback_handler, Command, UserLangState};

#[tokio::main]
async fn main() {
    // Default log level for Railway/Docker when RUST_LOG is not set.
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    pretty_env_logger::init();
    log::info!("Starting Telegram Bot...");

    // Fail fast with a clear message instead of a teloxide panic when the token is missing.
    if std::env::var("TELOXIDE_TOKEN").is_err() {
        log::error!("TELOXIDE_TOKEN is not set. Add it in Railway Variables (or .env locally).");
        std::process::exit(1);
    }
    let bot = Bot::from_env();

    // Register commands to the bot menu natively
    let _ = bot.set_my_commands(Command::bot_commands()).await;

    // Initialize shared state
    let user_lang_state: UserLangState = Arc::new(Mutex::new(HashMap::new()));

    let handler = dptree::entry()
        .branch(Update::filter_message().filter_command::<Command>().endpoint(command_handler))
        .branch(Update::filter_message().endpoint(handlers::message_handler))
        .branch(Update::filter_callback_query().endpoint(callback_handler));

    // Provide the bot and state to the dispatcher
    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![user_lang_state])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
