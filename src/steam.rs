/// Fetches the real game name from the Steam API
pub async fn fetch_game_name(appid: u32) -> Option<String> {
    let url = format!("https://store.steampowered.com/api/appdetails?appids={}", appid);
    if let Ok(resp) = reqwest::get(&url).await {
        if let Ok(json) = resp.json::<serde_json::Value>().await {
            if let Some(app_data) = json.get(&appid.to_string()) {
                if app_data.get("success").and_then(|v| v.as_bool()).unwrap_or(false) {
                    if let Some(data) = app_data.get("data") {
                        if let Some(name) = data.get("name").and_then(|n| n.as_str()) {
                            // Sanitize filename characters
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

/// Generates the VDF formatted `appmanifest_[id].acf` content.
pub fn generate_appmanifest(appid: u32, game_name: &str) -> String {
    format!(
r#""AppState"
{{
    "appid"     "{}"
    "Universe"  "1"
    "name"      "{}"
    "StateFlags" "1026"
    "installdir" "{}"
    "LastUpdated" "1672531200"
    "UpdateResult" "0"
    "SizeOnDisk" "0"
    "buildid" "1234567"
    "LastOwner" "0"
    "BytesToDownload" "0"
    "BytesDownloaded" "0"
    "BytesToStage" "0"
    "BytesStaged" "0"
    "AutoUpdateBehavior" "0"
    "AllowOtherDownloadsWhileRunning" "0"
    "ScheduledAutoUpdate" "0"
}}
"#,
        appid, game_name, game_name
    )
}

/// Generates the `.lua` persistence script content.
pub fn generate_lua_script(appid: u32) -> String {
    format!(
r#"-- SteamTools lua generator script
-- AppID: {}

local app_id = {}
local depot_id = app_id + 1

local function override_app_state()
    print(string.format("Intercepting Steam API for AppID %d", app_id))
    -- Mock payload for game manifest overrides
    local payload = {{
        app_id = app_id,
        depots = {{
            [depot_id] = {{ manifest = "1234567890123456789" }}
        }}
    }}
    return payload
end

override_app_state()
print("Success: Generated SteamTools bypass configuration.")
"#,
        appid, appid
    )
}
