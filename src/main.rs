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
    pretty_env_logger::init();
    log::info!("Starting Telegram Bot...");

    // You need to set TELOXIDE_TOKEN in your environment variables.
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
