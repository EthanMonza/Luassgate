/// Fetches the real game name from the Steam API
pub async fn fetch_game_name(appid: u32) -> Option<String> {
    let url = format!("https://store.steampowered.com/api/appdetails?appids={}", appid);
    if let Ok(resp) = reqwest::get(&url).await {
        if let Ok(json) = resp.json::<serde_json::Value>().await {
            if let Some(app_data) = json.get(&appid.to_string()) {
                if app_data.get("success").and_then(|v| v.as_bool()).unwrap_or(false) {
                    if let Some(data) = app_data.get("data") {
                        if let Some(name) = data.get("name").and_then(|n| n.as_str()) {
                            let safe_name = name.replace(|c: char| {
                                c == '<' || c == '>' || c == ':' || c == '"' || 
                                c == '/' || c == '\\' || c == '|' || c == '?' || c == '*'
                            }, "");
                            return Some(safe_name);
                        }
                    }
                }
            }
        }
    }
    None
}

pub async fn download_real_manifest_zip(appid: u32) -> Result<String, String> {
    // Many sites (like generator.ryuu.lol) require an active session cookie to prevent bot abuse.
    // Set MANIFEST_COOKIE to your browser session cookie.
    let api_cookie = std::env::var("MANIFEST_COOKIE").unwrap_or_default();
    
    // The base URL can be customized since these manifest sites frequently change domains.
    let custom_url = std::env::var("MANIFEST_API_URL").unwrap_or_default();
    
    let target_url = if custom_url.is_empty() {
        format!("https://pub-5b6d3b7c03fd4ac1afb5bd3017850e20.r2.dev/{}.zip", appid)
    } else {
        if custom_url.contains("{app_id}") {
            custom_url.replace("{app_id}", &appid.to_string())
        } else {
            format!("{}{}", custom_url, appid)
        }
    };
    
    let client = reqwest::Client::new();
    let mut req = client.get(&target_url);
    
    if !api_cookie.is_empty() {
        req = req.header("Cookie", api_cookie);
    }
    
    let resp = req.send().await.map_err(|e| format!("Failed to connect to API: {}", e))?;
    
    if !resp.status().is_success() {
        return Err(format!("API returned error: {}", resp.status()));
    }
    
    let temp_id = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis();
    let file_path = std::env::temp_dir().join(format!("steamtools_{}.zip", temp_id));
    
    let mut file = std::fs::File::create(&file_path).map_err(|e| e.to_string())?;
    let bytes = resp.bytes().await.map_err(|e| e.to_string())?;
    
    use std::io::Write;
    file.write_all(&bytes).map_err(|e| e.to_string())?;
    
    Ok(file_path.to_string_lossy().to_string())
}
