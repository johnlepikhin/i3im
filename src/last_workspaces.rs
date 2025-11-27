use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

/// Key used for workspaces without a group
const DEFAULT_GROUP_KEY: &str = "";

fn get_state_file_path() -> Result<PathBuf> {
    let state_home = std::env::var("XDG_STATE_HOME")
        .map(PathBuf::from)
        .or_else(|_| {
            std::env::var("HOME")
                .map(|home| PathBuf::from(home).join(".local/state"))
                .map_err(|_| anyhow::anyhow!("Neither XDG_STATE_HOME nor HOME is set"))
        })?;
    let path = state_home.join("i3im/last_workspaces.json");
    slog_scope::debug!("State file path: {:?}", path);
    Ok(path)
}

fn load() -> HashMap<String, i64> {
    let path = match get_state_file_path() {
        Ok(p) => p,
        Err(e) => {
            slog_scope::warn!("Failed to get state file path: {:?}", e);
            return HashMap::new();
        }
    };

    match fs::read_to_string(&path) {
        Ok(content) => {
            slog_scope::debug!("Loaded state file content: {}", content);
            let state: HashMap<String, i64> = serde_json::from_str(&content).unwrap_or_default();
            slog_scope::debug!("Parsed state: {:?}", state);
            state
        }
        Err(e) => {
            slog_scope::debug!("Failed to read state file: {:?}", e);
            HashMap::new()
        }
    }
}

/// Atomically saves the state to disk using write-to-temp-then-rename pattern
fn save(state: &HashMap<String, i64>) -> Result<()> {
    let path = get_state_file_path()?;

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).context("Failed to create state directory")?;
    }

    let content = serde_json::to_string_pretty(state).context("Failed to serialize state")?;
    slog_scope::debug!("Saving state: {}", content);

    // Atomic write: write to temp file, then rename
    let temp_path = path.with_extension("json.tmp");
    let mut file = fs::File::create(&temp_path).context("Failed to create temporary state file")?;
    file.write_all(content.as_bytes())
        .context("Failed to write to temporary state file")?;
    file.sync_all()
        .context("Failed to sync temporary state file")?;
    fs::rename(&temp_path, &path).context("Failed to rename temporary state file")?;

    Ok(())
}

fn group_to_key(group: Option<&str>) -> String {
    group.unwrap_or(DEFAULT_GROUP_KEY).to_string()
}

/// Updates the last workspace for current_group and returns the last workspace for target_group.
/// This is more efficient than separate set + get calls as it only reads/writes the file once.
pub fn update_and_get(
    current_group: Option<&str>,
    current_workspace: i64,
    target_group: Option<&str>,
) -> Option<i64> {
    slog_scope::debug!(
        "update_and_get: current_group={:?}, current_workspace={}, target_group={:?}",
        current_group,
        current_workspace,
        target_group
    );

    let mut state = load();

    // Save current workspace for current group
    let current_key = group_to_key(current_group);
    state.insert(current_key, current_workspace);

    // Get last workspace for target group
    let target_key = group_to_key(target_group);
    let result = state.get(&target_key).copied();

    // Save state
    if let Err(e) = save(&state) {
        slog_scope::warn!("Failed to save state: {:?}", e);
    }

    slog_scope::debug!(
        "update_and_get: target_key={:?}, result={:?}",
        target_key,
        result
    );
    result
}

pub fn get_last_workspace(group: Option<&str>) -> Option<i64> {
    let state = load();
    let key = group_to_key(group);
    let result = state.get(&key).copied();
    slog_scope::debug!(
        "get_last_workspace: group={:?}, key={:?}, result={:?}",
        group,
        key,
        result
    );
    result
}
