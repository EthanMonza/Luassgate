use std::sync::Arc;
use teloxide::{
    payloads::SendMessageSetters,
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup, InputFile},
    utils::command::BotCommands,
};
use tokio::sync::Mutex;
use std::collections::HashMap;
use std::io::Write;
use zip::write::FileOptions;

use crate::localization::{Language, Translator};
use crate::steam::{generate_appmanifest, generate_lua_script};

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
pub enum Command {
    #[command(description = "Start the bot and pick your language")]
    Start,
    #[command(description = "Generate Steam tools by AppID")]
    Id(String),
}

#[derive(Clone, Default)]
pub struct UserSession {
    pub lang: Language,
    pub waiting_for_id: bool,
}

pub type UserLangState = Arc<Mutex<HashMap<ChatId, UserSession>>>;

pub async fn get_user_session(state: &UserLangState, chat_id: ChatId) -> UserSession {
    let map = state.lock().await;
    map.get(&chat_id).cloned().unwrap_or_default()
}

pub async fn set_user_lang(state: &UserLangState, chat_id: ChatId, lang: Language) {
    let mut map = state.lock().await;
    let mut session = map.get(&chat_id).cloned().unwrap_or_default();
    session.lang = lang;
    map.insert(chat_id, session);
}

pub async fn set_waiting_for_id(state: &UserLangState, chat_id: ChatId, waiting: bool) {
    let mut map = state.lock().await;
    let mut session = map.get(&chat_id).cloned().unwrap_or_default();
    session.waiting_for_id = waiting;
    map.insert(chat_id, session);
}

async fn generate_and_send_steam_tools(bot: Bot, chat_id: ChatId, appid: u32, lang: Language) -> ResponseResult<()> {
    // Attempt to fetch real game name, fallback to a default
    let mut game_name = format!("SteamTools App {}", appid);
    if let Some(fetched_name) = crate::steam::fetch_game_name(appid).await {
        if !fetched_name.is_empty() {
            game_name = fetched_name;
        }
    }

    let acf_content = generate_appmanifest(appid, &game_name);
    let lua_content = generate_lua_script(appid);
    
    let zip_filename = format!("{}; {}.zip", appid, game_name);
    let final_zip = std::env::temp_dir().join(&zip_filename);
    
    let file = std::fs::File::create(&final_zip).unwrap();
    let mut zip = zip::ZipWriter::new(file);
    let options = FileOptions::default().compression_method(zip::CompressionMethod::Stored);
    
    zip.start_file(format!("appmanifest_{}.acf", appid), options).unwrap();
    zip.write_all(acf_content.as_bytes()).unwrap();
    
    zip.start_file(format!("{}.lua", appid), options).unwrap();
    zip.write_all(lua_content.as_bytes()).unwrap();
    
    zip.finish().unwrap();

    let input_file = InputFile::file(&final_zip).file_name(zip_filename);
    bot.send_document(chat_id, input_file).await?;
    let _ = std::fs::remove_file(final_zip);
    Ok(())
}

pub async fn command_handler(
    bot: Bot,
    msg: Message,
    cmd: Command,
    state: UserLangState,
) -> ResponseResult<()> {
    let chat_id = msg.chat.id;
    let session = get_user_session(&state, chat_id).await;
    let lang = session.lang;

    match cmd {
        Command::Start => {
            set_waiting_for_id(&state, chat_id, false).await;
            let keyboard = InlineKeyboardMarkup::default()
                .append_row(vec![
                    InlineKeyboardButton::callback("🇬🇧 English", "set_lang:en"),
                    InlineKeyboardButton::callback("🇪🇸 Español", "set_lang:es"),
                    InlineKeyboardButton::callback("🇹🇷 Türkçe", "set_lang:tr"),
                    InlineKeyboardButton::callback("🇷🇺 Русский", "set_lang:ru"),
                ])
                .append_row(vec![
                    InlineKeyboardButton::callback("🇩🇪 Deutsch", "set_lang:de"),
                    InlineKeyboardButton::callback("🇫🇷 Français", "set_lang:fr"),
                    InlineKeyboardButton::callback("🇬🇧 UK English", "set_lang:en_UK"),
                    InlineKeyboardButton::callback("🇳🇿 NZ English", "set_lang:en_NZ"),
                ]);

            let text = Translator::start_message(lang);
            bot.send_message(chat_id, text)
                .reply_markup(keyboard)
                .await?;
        }
        Command::Id(arg) => {
            if arg.is_empty() {
                set_waiting_for_id(&state, chat_id, true).await;
                bot.send_message(chat_id, "Please enter the Steam AppID (e.g., 1962700):").await?;
                return Ok(());
            }

            match arg.parse::<u32>() {
                Ok(appid) => {
                    generate_and_send_steam_tools(bot, chat_id, appid, lang).await?;
                }
                Err(_) => {
                    bot.send_message(chat_id, Translator::invalid_id(lang)).await?;
                }
            }
        }
    };

    Ok(())
}

pub async fn message_handler(
    bot: Bot,
    msg: Message,
    state: UserLangState,
) -> ResponseResult<()> {
    let chat_id = msg.chat.id;
    let session = get_user_session(&state, chat_id).await;

    if session.waiting_for_id {
        if let Some(text) = msg.text() {
            match text.trim().parse::<u32>() {
                Ok(appid) => {
                    set_waiting_for_id(&state, chat_id, false).await;
                    generate_and_send_steam_tools(bot, chat_id, appid, session.lang).await?;
                }
                Err(_) => {
                    bot.send_message(chat_id, Translator::invalid_id(session.lang)).await?;
                }
            }
        }
    }
    Ok(())
}

pub async fn callback_handler(
    bot: Bot,
    q: CallbackQuery,
    state: UserLangState,
) -> ResponseResult<()> {
    if let Some(ref data) = q.data {
        let chat_id = q.message.as_ref().map(|m| m.chat.id).unwrap_or(ChatId(0));
        
        if data.starts_with("set_lang:") {
            let lang_str = data.trim_start_matches("set_lang:");
            if let Some(lang) = Language::from_str(lang_str) {
                set_user_lang(&state, chat_id, lang).await;
                
                let text = Translator::language_updated(lang);
                
                if let Some(msg) = q.message {
                    bot.edit_message_text(chat_id, msg.id, text).await?;
                }
                bot.answer_callback_query(q.id).await?;
            }
        }
    }
    
    Ok(())
}
