use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};
use url::Url;
use log::info;
use tokio::process::Command;

/// Parses a URL and determines if it belongs to a supported media platform
pub fn is_supported_media_link(url_str: &str) -> bool {
    if let Ok(url) = Url::parse(url_str) {
        if let Some(domain) = url.domain() {
            return domain.contains("youtube.com")
                || domain.contains("youtu.be")
                || domain.contains("instagram.com")
                || domain.contains("pinterest.com")
                || domain.contains("spotify.com")
                || domain.contains("music.youtube.com");
        }
    }
    false
}

/// Generates the Inline Keyboard for selecting MP3 or MP4 download options
pub fn media_format_keyboard(url: &str) -> InlineKeyboardMarkup {
    // Note: Callback data is limited to 64 bytes. 
    // If URLs can be longer, we might need to store them temporarily in a cache and pass an ID.
    // For simplicity, we are passing the URL directly or truncating/encoding if needed.
    // Assuming the URL fits or using a placeholder strategy if it exceeds.
    // In production, consider saving `url` to a db and passing `download_mp3:<db_id>`.
    
    // To be safe for this demo, we'll slice if it's too long, but it might break the URL.
    let safe_url = if url.len() > 45 {
        // Fallback for huge URLs - requires caching in a real scenario
        &url[..45] 
    } else {
        url
    };

    InlineKeyboardMarkup::default()
        .append_row(vec![
            InlineKeyboardButton::callback("🎵 Audio (MP3)", format!("dl_mp3:{}", safe_url)),
            InlineKeyboardButton::callback("🎬 Video (MP4)", format!("dl_mp4:{}", safe_url)),
        ])
}

/// Mocks downloading a media file using an external tool like yt-dlp.
/// Note: yt-dlp binary is expected to be accessible in the system PATH.
pub async fn download_media(url: &str, is_audio: bool) -> Result<String, String> {
    info!("Triggered media download for: {} (Audio: {})", url, is_audio);
    
    let mut cmd = Command::new("yt-dlp");
    if is_audio {
        cmd.args(&["-x", "--audio-format", "mp3", url]);
    } else {
        cmd.args(&["-f", "mp4", url]);
    }

    // This is mocked out since we might not have yt-dlp in the environment.
    // In a real environment, uncomment the process execution.
    /*
    let output = cmd
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to start yt-dlp: {}", e))?
        .wait_with_output()
        .await
        .map_err(|e| format!("Failed to wait for yt-dlp: {}", e))?;

    if !output.status.success() {
        let err_msg = String::from_utf8_lossy(&output.stderr);
        error!("yt-dlp error: {}", err_msg);
        return Err("Download failed due to external tool error".into());
    }
    */
    
    // Mock return: pretend we saved a file locally.
    let simulated_filename = if is_audio { "downloaded_audio.mp3" } else { "downloaded_video.mp4" };
    Ok(simulated_filename.to_string())
}
