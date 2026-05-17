/// Generates the VDF formatted `appmanifest_[id].acf` content.
pub fn generate_appmanifest(appid: u32) -> String {
    format!(
r#""AppState"
{{
    "appid"     "{}"
    "Universe"  "1"
    "name"      "SteamTools App {}"
    "StateFlags" "1026"
    "installdir" "SteamToolsApp_{}"
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
        appid, appid, appid
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
