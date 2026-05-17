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
use tempfile::NamedTempFile;

use crate::localization::{Language, Translator};
use crate::media::{is_supported_media_link, media_format_keyboard};
use crate::steam::{generate_appmanifest, generate_lua_script};

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
pub enum Command {
    #[command(description = "Start the bot and pick your language")]
    Start,
    #[command(description = "Generate Steam tools by AppID")]
    Id(String),
    #[command(description = "Download a video/audio from a link")]
    Video(String),
}

// In-memory state to keep track of user languages.
// In production, this should be backed by a database (SQLite, Postgres, Redis).
pub type UserLangState = Arc<Mutex<HashMap<ChatId, Language>>>;

pub async fn get_user_lang(state: &UserLangState, chat_id: ChatId) -> Language {
    let map = state.lock().await;
    map.get(&chat_id).cloned().unwrap_or(Language::En)
}

pub async fn set_user_lang(state: &UserLangState, chat_id: ChatId, lang: Language) {
    let mut map = state.lock().await;
    map.insert(chat_id, lang);
}

pub async fn command_handler(
    bot: Bot,
    msg: Message,
    cmd: Command,
    state: UserLangState,
) -> ResponseResult<()> {
    let chat_id = msg.chat.id;
    let lang = get_user_lang(&state, chat_id).await;

    match cmd {
        Command::Start => {
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
            // Check if arg is empty
            if arg.is_empty() {
                bot.send_message(chat_id, "Please provide an AppID. Example: /id 1245620").await?;
                return Ok(());
            }

            // Check if it's actually a media link fallback
            if is_supported_media_link(&arg) {
                let text = format!(
                    "{}\n\n{}",
                    Translator::sarcastic_url_fallback(lang),
                    Translator::media_choice_prompt(lang)
                );
                
                let keyboard = media_format_keyboard(&arg);
                bot.send_message(chat_id, text)
                    .reply_markup(keyboard)
                    .await?;
                return Ok(());
            }

            // Try parse to u32
            match arg.parse::<u32>() {
                Ok(appid) => {
                    let acf_content = generate_appmanifest(appid);
                    let lua_content = generate_lua_script(appid);
                    
                    // Create temporary files to send as documents
                    let mut acf_file = NamedTempFile::new().unwrap();
                    write!(acf_file, "{}", acf_content).unwrap();
                    let acf_path = acf_file.into_temp_path();
                    
                    let mut lua_file = NamedTempFile::new().unwrap();
                    write!(lua_file, "{}", lua_content).unwrap();
                    let lua_path = lua_file.into_temp_path();
                    
                    // Rename paths conceptually (InputFile requires a standard path or stream)
                    // For simplicity we will rename them in the filesystem temp dir temporarily
                    let final_acf = std::env::temp_dir().join(format!("appmanifest_{}.acf", appid));
                    let final_lua = std::env::temp_dir().join(format!("{}.lua", appid));
                    
                    std::fs::rename(acf_path, &final_acf).unwrap();
                    std::fs::rename(lua_path, &final_lua).unwrap();

                    bot.send_document(chat_id, InputFile::file(&final_acf)).await?;
                    bot.send_document(chat_id, InputFile::file(&final_lua)).await?;

                    // Cleanup
                    let _ = std::fs::remove_file(final_acf);
                    let _ = std::fs::remove_file(final_lua);
                }
                Err(_) => {
                    bot.send_message(chat_id, Translator::invalid_id(lang)).await?;
                }
            }
        }
        Command::Video(arg) => {
            if arg.is_empty() || !is_supported_media_link(&arg) {
                bot.send_message(chat_id, "Please provide a valid media URL. Example: /video https://youtube.com/...").await?;
                return Ok(());
            }

            let text = Translator::media_choice_prompt(lang);
            let keyboard = media_format_keyboard(&arg);
            bot.send_message(chat_id, text)
                .reply_markup(keyboard)
                .await?;
        }
    };

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
        } else if data.starts_with("dl_mp3:") || data.starts_with("dl_mp4:") {
            let is_audio = data.starts_with("dl_mp3:");
            let prefix = if is_audio { "dl_mp3:" } else { "dl_mp4:" };
            let url = data.trim_start_matches(prefix);
            
            // Acknowledge the query immediately so telegram UI stops loading
            bot.answer_callback_query(q.id.clone()).text("Download started...").await?;
            
            if let Some(msg) = q.message {
                bot.edit_message_text(chat_id, msg.id, "Processing your download... Please wait.").await?;
                
                // Note: The actual download_media function in media.rs is mocked.
                // You can spawn a task here or call it directly.
                // let result = crate::media::download_media(url, is_audio).await;
                // Since this might take a while, normally spawn it:
                let bot_clone = bot.clone();
                let url_owned = url.to_string();
                
                tokio::spawn(async move {
                    // let file_path = crate::media::download_media(&url_owned, is_audio).await;
                    // Mock delay
                    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                    
                    let _ = bot_clone.send_message(
                        chat_id, 
                        format!("✅ Download complete for: {}\n(Simulated)", url_owned)
                    ).await;
                });
            }
        }
    }
    
    Ok(())
}
