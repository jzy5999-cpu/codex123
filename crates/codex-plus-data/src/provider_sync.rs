use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value, json};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

const DEFAULT_PROVIDER: &str = "openai";
const SESSION_DIRS: [&str; 2] = ["sessions", "archived_sessions"];
const BACKUP_KEEP_COUNT: usize = 5;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderSyncStatus {
    Disabled,
    Skipped,
    Synced,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderSyncResult {
    pub status: ProviderSyncStatus,
    pub message: String,
    pub target_provider: String,
    pub backup_dir: Option<PathBuf>,
    pub changed_session_files: usize,
    pub sqlite_rows_updated: usize,
}

#[derive(Debug, Clone)]
struct SessionChange {
    path: PathBuf,
    original_text: String,
    next_text: String,
    original_session_meta_lines: Vec<String>,
    thread_id: Option<String>,
    cwd: Option<String>,
    has_user_event: bool,
    rewrite_needed: bool,
    original_mtime: Option<SystemTime>,
}

#[derive(Debug, Default)]
struct RolloutRewrite {
    next_text: String,
    rewrite_needed: bool,
    thread_id: Option<String>,
    cwd: Option<String>,
    original_session_meta_lines: Vec<String>,
    session_meta_count: usize,
}

pub fn run_provider_sync(codex_home: Option<&Path>) -> ProviderSyncResult {
    run_provider_sync_with_target(codex_home, None)
}

pub fn run_provider_sync_with_target(
    codex_home: Option<&Path>,
    target_provider: Option<&str>,
) -> ProviderSyncResult {
    let home = codex_home
        .map(Path::to_path_buf)
        .unwrap_or_else(|| dirs_home().join(".codex"));
    let requested_target = match target_provider {
        Some(value) => match normalize_requested_provider(value) {
            Ok(provider) => Some(provider),
            Err(message) => {
                return result(
                    ProviderSyncStatus::Skipped,
                    message,
                    DEFAULT_PROVIDER,
                    None,
                    0,
                    0,
                );
            }
        },
        None => None,
    };
    if !home.exists() {
        return result(
            ProviderSyncStatus::Skipped,
            format!("Codex home not found: {}", home.to_string_lossy()),
            requested_target.as_deref().unwrap_or(DEFAULT_PROVIDER),
            None,
            0,
            0,
        );
    }
    let target_provider = requested_target.unwrap_or_else(|| current_provider(Some(&home)));
    let lock_dir = home.join("tmp/provider-sync.lock");
    if acquire_lock(&lock_dir).is_err() {
        return result(
            ProviderSyncStatus::Skipped,
            format!("Provider sync lock exists: {}", lock_dir.to_string_lossy()),
            &target_provider,
            None,
            0,
            0,
        );
    }
    let sync_result = (|| -> anyhow::Result<ProviderSyncResult> {
        let changes = collect_session_changes(&home, &target_provider)?;
        let rewrite_changes = changes
            .iter()
            .filter(|change| change.rewrite_needed)
            .cloned()
            .collect::<Vec<_>>();
        let thread_ids_with_user_events = changes
            .iter()
            .filter(|change| change.has_user_event)
            .filter_map(|change| change.thread_id.clone())
            .collect::<HashSet<_>>();
        let cwd_by_thread_id = changes
            .iter()
            .filter_map(|change| Some((change.thread_id.clone()?, change.cwd.clone()?)))
            .collect::<HashMap<_, _>>();
        let sqlite_paths = codex_plus_core::codex_sqlite::codex_session_db_paths_from_home(&home);
        let sqlite_update_count = count_sqlite_updates_for_paths(
            &sqlite_paths,
            &target_provider,
            &thread_ids_with_user_events,
            &cwd_by_thread_id,
        )?;
        let global_state_update_count =
            count_global_state_updates(&home.join(".codex-global-state.json"))?;
        if rewrite_changes.is_empty() && sqlite_update_count == 0 && global_state_update_count == 0
        {
            return Ok(result(
                ProviderSyncStatus::Synced,
                "Provider sync already up to date",
                &target_provider,
                None,
                0,
                0,
            ));
        }
        let backup_dir = create_backup(&home, &target_provider, &rewrite_changes)?;
        apply_session_changes(&rewrite_changes)?;
        let apply_result = (|| -> anyhow::Result<usize> {
            let sqlite_rows_updated = apply_sqlite_update_for_paths(
                &sqlite_paths,
                &target_provider,
                &thread_ids_with_user_events,
                &cwd_by_thread_id,
            )?;
            apply_global_state_update(&home.join(".codex-global-state.json"))?;
            prune_backups(&home)?;
            Ok(sqlite_rows_updated)
        })();
        let sqlite_rows_updated = match apply_result {
            Ok(count) => count,
            Err(err) => {
                let _ = restore_session_changes(&rewrite_changes);
                return Err(err);
            }
        };
        Ok(result(
            ProviderSyncStatus::Synced,
            "Provider sync complete",
            &target_provider,
            Some(backup_dir),
            rewrite_changes.len(),
            sqlite_rows_updated,
        ))
    })();
    let _ = release_lock(&lock_dir);
    sync_result.unwrap_or_else(|err| {
        result(
            ProviderSyncStatus::Skipped,
            format!("Provider sync skipped: {err}"),
            &target_provider,
            None,
            0,
            0,
        )
    })
}

pub fn current_provider(codex_home: Option<&Path>) -> String {
    let home = codex_home
        .map(Path::to_path_buf)
        .unwrap_or_else(|| dirs_home().join(".codex"));
    read_current_provider(&home.join("config.toml"))
}

fn result(
    status: ProviderSyncStatus,
    message: impl Into<String>,
    target_provider: &str,
    backup_dir: Option<PathBuf>,
    changed_session_files: usize,
    sqlite_rows_updated: usize,
) -> ProviderSyncResult {
    ProviderSyncResult {
        status,
        message: message.into(),
        target_provider: target_provider.to_string(),
        backup_dir,
        changed_session_files,
        sqlite_rows_updated,
    }
}

fn dirs_home() -> PathBuf {
    std::env::var_os("USERPROFILE")
        .or_else(|| std::env::var_os("HOME"))
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."))
}

fn read_current_provider(path: &Path) -> String {
    let Ok(text) = fs::read_to_string(path) else {
        return DEFAULT_PROVIDER.to_string();
    };
    for line in text.lines() {
        let stripped = line.trim();
        if stripped.starts_with("model_provider") && stripped.contains('=') {
            let raw = stripped
                .split_once('=')
                .map(|(_, value)| value.trim())
                .unwrap_or("");
            if raw.len() >= 2 && raw.starts_with('"') && raw.ends_with('"') {
                let value = &raw[1..raw.len() - 1];
                return if value.is_empty() {
                    DEFAULT_PROVIDER.to_string()
                } else {
                    value.to_string()
                };
            }
        }
    }
    DEFAULT_PROVIDER.to_string()
}

fn normalize_requested_provider(value: &str) -> Result<String, String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err("Provider sync target provider is empty".to_string());
    }
    if trimmed
        .chars()
        .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '_' | '-' | '.'))
    {
        Ok(trimmed.to_string())
    } else {
        Err("Provider sync target provider can only contain letters, numbers, underscores, dashes, and dots".to_string())
    }
}

fn acquire_lock(path: &Path) -> std::io::Result<()> {
    fs::create_dir_all(path.parent().unwrap_or_else(|| Path::new(".")))?;
    fs::create_dir(path)?;
    fs::write(
        path.join("owner.json"),
        json!({"pid": std::process::id(), "startedAt": now_secs()}).to_string(),
    )
}

fn release_lock(path: &Path) -> std::io::Result<()> {
    if path.exists() {
        fs::remove_dir_all(path)?;
    }
    Ok(())
}

fn collect_session_changes(
    home: &Path,
    target_provider: &str,
) -> anyhow::Result<Vec<SessionChange>> {
    let mut changes = Vec::new();
    for path in rollout_files(home)? {
        let text = fs::read_to_string(&path)?;
        let rewrite = rewrite_rollout_session_meta_providers(&text, target_provider)?;
        if rewrite.session_meta_count == 0 {
            continue;
        }
        let has_user_event = text.contains("\"user_message\"") || text.contains("\"user_input\"");
        let original_mtime = fs::metadata(&path).and_then(|m| m.modified()).ok();
        changes.push(SessionChange {
            path,
            original_text: text,
            next_text: rewrite.next_text,
            original_session_meta_lines: rewrite.original_session_meta_lines,
            thread_id: rewrite.thread_id,
            cwd: rewrite.cwd,
            has_user_event,
            rewrite_needed: rewrite.rewrite_needed,
            original_mtime,
        });
    }
    Ok(changes)
}

fn rewrite_rollout_session_meta_providers(
    text: &str,
    target_provider: &str,
) -> anyhow::Result<RolloutRewrite> {
    let mut rewrite = RolloutRewrite::default();
    for segment in text.split_inclusive('\n') {
        let (line, line_ending) = split_line_ending(segment);
        let mut next_line = line.to_string();
        if !line.trim().is_empty() {
            if let Ok(mut record) = serde_json::from_str::<Value>(line) {
                if record.get("type").and_then(Value::as_str) == Some("session_meta") {
                    let Some(payload) = record.get_mut("payload").and_then(Value::as_object_mut)
                    else {
                        rewrite.next_text.push_str(&next_line);
                        rewrite.next_text.push_str(line_ending);
                        continue;
                    };
                    rewrite.session_meta_count += 1;
                    rewrite.original_session_meta_lines.push(line.to_string());
                    if rewrite.thread_id.is_none() {
                        rewrite.thread_id = payload
                            .get("id")
                            .and_then(Value::as_str)
                            .map(ToString::to_string);
                    }
                    if rewrite.cwd.is_none() {
                        rewrite.cwd = payload
                            .get("cwd")
                            .and_then(Value::as_str)
                            .and_then(to_desktop_workspace_path);
                    }
                    if payload.get("model_provider").and_then(Value::as_str)
                        != Some(target_provider)
                    {
                        payload.insert("model_provider".to_string(), json!(target_provider));
                        next_line = serde_json::to_string(&record)?;
                        rewrite.rewrite_needed = true;
                    }
                }
            }
        }
        rewrite.next_text.push_str(&next_line);
        rewrite.next_text.push_str(line_ending);
    }
    Ok(rewrite)
}

fn rollout_files(home: &Path) -> anyhow::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for dirname in SESSION_DIRS {
        let root = home.join(dirname);
        if root.exists() {
            collect_rollout_files(&root, &mut files)?;
        }
    }
    files.sort();
    Ok(files)
}

fn collect_rollout_files(root: &Path, files: &mut Vec<PathBuf>) -> anyhow::Result<()> {
    for entry in fs::read_dir(root)? {
        let path = entry?.path();
        if path.is_dir() {
            collect_rollout_files(&path, files)?;
        } else if path
            .file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| name.starts_with("rollout-") && name.ends_with(".jsonl"))
        {
            files.push(path);
        }
    }
    Ok(())
}

fn split_line_ending(segment: &str) -> (&str, &str) {
    if let Some(line) = segment.strip_suffix("\r\n") {
        (line, "\r\n")
    } else if let Some(line) = segment.strip_suffix('\n') {
        (line, "\n")
    } else {
        (segment, "")
    }
}

fn to_desktop_workspace_path(value: &str) -> Option<String> {
    let stripped = value.trim();
    if stripped.is_empty() {
        return None;
    }
    let lower = stripped.to_ascii_lowercase();
    if lower.starts_with(r"\\?\unc\") {
        return Some(format!(r"\\{}", stripped[8..].replace('/', r"\")));
    }
    if stripped.starts_with(r"\\?\") {
        return Some(stripped[4..].replace('\\', "/"));
    }
    Some(stripped.to_string())
}

fn create_backup(
    home: &Path,
    target_provider: &str,
    changes: &[SessionChange],
) -> anyhow::Result<PathBuf> {
    let backup_root = home.join("backups_state/provider-sync");
    let mut backup_dir = backup_root.join(timestamp_name());
    let mut suffix = 0;
    while backup_dir.exists() {
        suffix += 1;
        backup_dir = backup_root.join(format!("{}-{suffix}", timestamp_name()));
    }
    fs::create_dir_all(&backup_dir)?;
    for name in [
        "config.toml",
        ".codex-global-state.json",
        ".codex-global-state.json.bak",
    ] {
        let source = home.join(name);
        if source.exists() {
            fs::copy(&source, backup_dir.join(name))?;
        }
    }
    let db_dir = backup_dir.join("db");
    for db_path in codex_plus_core::codex_sqlite::codex_session_db_paths_from_home(home) {
        for source in codex_plus_core::codex_sqlite::codex_sqlite_sidecar_paths(&db_path) {
            if !source.exists() {
                continue;
            }
            let relative = codex_plus_core::codex_sqlite::relative_to_codex_home(home, &source);
            let target = db_dir.join(&relative);
            if let Some(parent) = target.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(&source, target)?;
        }
    }
    let manifest = changes
        .iter()
        .map(|change| {
            json!({
                "path": change.path.to_string_lossy(),
                "originalSessionMetaLines": change.original_session_meta_lines,
            })
        })
        .collect::<Vec<_>>();
    fs::write(
        backup_dir.join("session-meta-backup.json"),
        serde_json::to_string_pretty(&manifest)?,
    )?;
    fs::write(
        backup_dir.join("metadata.json"),
        serde_json::to_string_pretty(
            &json!({"managedBy": "Codex++ provider sync", "targetProvider": target_provider}),
        )?,
    )?;
    Ok(backup_dir)
}

fn apply_session_changes(changes: &[SessionChange]) -> anyhow::Result<()> {
    for change in changes {
        fs::write(&change.path, &change.next_text)?;
        restore_file_mtime(&change.path, change.original_mtime);
    }
    Ok(())
}

fn restore_session_changes(changes: &[SessionChange]) -> anyhow::Result<()> {
    for change in changes {
        fs::write(&change.path, &change.original_text)?;
        restore_file_mtime(&change.path, change.original_mtime);
    }
    Ok(())
}

fn restore_file_mtime(path: &Path, mtime: Option<SystemTime>) {
    let Some(mtime) = mtime else { return };
    let Ok(file) = fs::File::options().write(true).open(path) else {
        return;
    };
    let times = std::fs::FileTimes::new().set_modified(mtime);
    let _ = file.set_times(times);
}

fn table_columns(db: &Connection, table: &str) -> anyhow::Result<HashSet<String>> {
    let mut stmt = db.prepare(&format!(
        "PRAGMA table_info(\"{}\")",
        table.replace('"', "\"\"")
    ))?;
    Ok(stmt
        .query_map([], |row| row.get::<_, String>(1))?
        .collect::<rusqlite::Result<HashSet<_>>>()?)
}

fn count_sqlite_updates(
    path: &Path,
    target_provider: &str,
    user_event_thread_ids: &HashSet<String>,
    cwd_by_thread_id: &HashMap<String, String>,
) -> anyhow::Result<usize> {
    if !path.exists() {
        return Ok(0);
    }
    let db = Connection::open(path)?;
    let columns = table_columns(&db, "threads")?;
    if !columns.contains("model_provider") {
        return Ok(0);
    }
    let mut total: usize = db.query_row(
        "SELECT COUNT(*) FROM threads WHERE COALESCE(model_provider, '') <> ?1",
        [target_provider],
        |row| row.get::<_, i64>(0),
    )? as usize;
    if columns.contains("has_user_event") {
        for thread_id in user_event_thread_ids {
            total += db.query_row(
                "SELECT COUNT(*) FROM threads WHERE id = ?1 AND COALESCE(has_user_event, 0) <> 1",
                [thread_id],
                |row| row.get::<_, i64>(0),
            )? as usize;
        }
    }
    if columns.contains("cwd") {
        for (thread_id, cwd) in cwd_by_thread_id {
            total += db.query_row(
                "SELECT COUNT(*) FROM threads WHERE id = ?1 AND COALESCE(cwd, '') <> ?2",
                (thread_id, cwd),
                |row| row.get::<_, i64>(0),
            )? as usize;
        }
    }
    Ok(total)
}

fn count_sqlite_updates_for_paths(
    paths: &[PathBuf],
    target_provider: &str,
    user_event_thread_ids: &HashSet<String>,
    cwd_by_thread_id: &HashMap<String, String>,
) -> anyhow::Result<usize> {
    let mut total = 0;
    for path in paths {
        total += count_sqlite_updates(
            path,
            target_provider,
            user_event_thread_ids,
            cwd_by_thread_id,
        )?;
    }
    Ok(total)
}

fn apply_sqlite_update(
    path: &Path,
    target_provider: &str,
    user_event_thread_ids: &HashSet<String>,
    cwd_by_thread_id: &HashMap<String, String>,
) -> anyhow::Result<usize> {
    if !path.exists() {
        return Ok(0);
    }
    let mut db = Connection::open(path)?;
    let columns = table_columns(&db, "threads")?;
    if !columns.contains("model_provider") {
        return Ok(0);
    }
    let tx = db.transaction()?;
    let provider_rows = tx.execute(
        "UPDATE threads SET model_provider = ?1 WHERE COALESCE(model_provider, '') <> ?1",
        [target_provider],
    )?;
    if columns.contains("has_user_event") {
        for thread_id in user_event_thread_ids {
            tx.execute(
                "UPDATE threads SET has_user_event = 1 WHERE id = ?1 AND COALESCE(has_user_event, 0) <> 1",
                [thread_id],
            )?;
        }
    }
    if columns.contains("cwd") {
        for (thread_id, cwd) in cwd_by_thread_id {
            tx.execute(
                "UPDATE threads SET cwd = ?1 WHERE id = ?2 AND COALESCE(cwd, '') <> ?1",
                (cwd, thread_id),
            )?;
        }
    }
    tx.commit()?;
    Ok(provider_rows)
}

fn apply_sqlite_update_for_paths(
    paths: &[PathBuf],
    target_provider: &str,
    user_event_thread_ids: &HashSet<String>,
    cwd_by_thread_id: &HashMap<String, String>,
) -> anyhow::Result<usize> {
    let mut total = 0;
    for path in paths {
        total += apply_sqlite_update(
            path,
            target_provider,
            user_event_thread_ids,
            cwd_by_thread_id,
        )?;
    }
    Ok(total)
}

fn load_global_state(path: &Path) -> anyhow::Result<Map<String, Value>> {
    if !path.exists() {
        return Ok(Map::new());
    }
    Ok(serde_json::from_str::<Value>(&fs::read_to_string(path)?)?
        .as_object()
        .cloned()
        .unwrap_or_default())
}

fn normalized_global_state(state: &Map<String, Value>) -> Map<String, Value> {
    let mut next = Map::new();
    if let Some(value) = state.get("electron-saved-workspace-roots") {
        next.insert(
            "electron-saved-workspace-roots".to_string(),
            json!(dedupe_paths(path_array(value))),
        );
    }
    if let Some(value) = state.get("project-order") {
        next.insert(
            "project-order".to_string(),
            json!(dedupe_paths(path_array(value))),
        );
    }
    if let Some(value) = state.get("active-workspace-roots") {
        let normalized = dedupe_paths(path_array(value));
        let next_value = if value.is_array() {
            json!(normalized)
        } else if let Some(first) = normalized.first() {
            json!(first)
        } else {
            value.clone()
        };
        next.insert("active-workspace-roots".to_string(), next_value);
    }
    if let Some(value) = state
        .get("electron-workspace-root-labels")
        .and_then(Value::as_object)
    {
        let mut labels = Map::new();
        for (key, item) in value {
            labels.insert(
                to_desktop_workspace_path(key).unwrap_or_else(|| key.clone()),
                item.clone(),
            );
        }
        next.insert(
            "electron-workspace-root-labels".to_string(),
            Value::Object(labels),
        );
    }
    next
}

fn count_global_state_updates(path: &Path) -> anyhow::Result<usize> {
    let state = load_global_state(path)?;
    let next = normalized_global_state(&state);
    Ok(next
        .iter()
        .filter(|(key, value)| state.get(*key) != Some(*value))
        .count())
}

fn apply_global_state_update(path: &Path) -> anyhow::Result<usize> {
    let mut state = load_global_state(path)?;
    let next = normalized_global_state(&state);
    let count = next
        .iter()
        .filter(|(key, value)| state.get(*key) != Some(*value))
        .count();
    if count > 0 {
        for (key, value) in next {
            state.insert(key, value);
        }
        fs::write(path, serde_json::to_string_pretty(&Value::Object(state))?)?;
    }
    Ok(count)
}

fn path_array(value: &Value) -> Vec<String> {
    if let Some(items) = value.as_array() {
        items
            .iter()
            .filter_map(Value::as_str)
            .filter(|item| !item.trim().is_empty())
            .map(ToString::to_string)
            .collect()
    } else if let Some(value) = value.as_str().filter(|item| !item.trim().is_empty()) {
        vec![value.to_string()]
    } else {
        Vec::new()
    }
}

fn dedupe_paths(paths: Vec<String>) -> Vec<String> {
    let mut seen = HashSet::new();
    let mut result = Vec::new();
    for path in paths {
        let Some(desktop) = to_desktop_workspace_path(&path) else {
            continue;
        };
        let comparable = desktop
            .replace('/', r"\")
            .trim_end_matches('\\')
            .to_ascii_lowercase();
        if seen.insert(comparable) {
            result.push(desktop);
        }
    }
    result
}

fn prune_backups(home: &Path) -> anyhow::Result<()> {
    let root = home.join("backups_state/provider-sync");
    if !root.exists() {
        return Ok(());
    }
    let mut managed = Vec::new();
    for entry in fs::read_dir(&root)? {
        let path = entry?.path();
        if !path.is_dir() {
            continue;
        }
        let Ok(text) = fs::read_to_string(path.join("metadata.json")) else {
            continue;
        };
        let Ok(value) = serde_json::from_str::<Value>(&text) else {
            continue;
        };
        if value.get("managedBy").and_then(Value::as_str) == Some("Codex++ provider sync") {
            managed.push(path);
        }
    }
    managed.sort_by(|a, b| b.file_name().cmp(&a.file_name()));
    for path in managed.into_iter().skip(BACKUP_KEEP_COUNT) {
        let _ = fs::remove_dir_all(path);
    }
    Ok(())
}

fn timestamp_name() -> String {
    chrono::Local::now().format("%Y%m%d%H%M%S").to_string()
}

fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
