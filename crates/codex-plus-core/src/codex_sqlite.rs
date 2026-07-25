use std::ffi::{OsStr, OsString};
use std::fs;
use std::path::{Path, PathBuf};

use rusqlite::Connection;

pub fn default_codex_home_dir() -> PathBuf {
    std::env::var_os("CODEX_HOME")
        .map(PathBuf::from)
        .or_else(|| {
            std::env::var_os("HOME")
                .or_else(|| std::env::var_os("USERPROFILE"))
                .map(PathBuf::from)
                .map(|home| home.join(".codex"))
        })
        .unwrap_or_else(|| PathBuf::from(".codex"))
}

pub fn codex_session_db_path_from_home(home: &Path) -> PathBuf {
    let paths = codex_session_db_paths_from_home(home);
    paths
        .iter()
        .find(|path| sqlite_has_table(path, "threads"))
        .cloned()
        .or_else(|| paths.into_iter().next())
        .unwrap_or_else(|| legacy_state_db_path(home))
}

pub fn codex_session_db_paths_from_home(home: &Path) -> Vec<PathBuf> {
    let sqlite_home = resolve_sqlite_home_home_or_default(home);
    codex_session_db_paths_in_home(&sqlite_home)
}

fn codex_session_db_paths_in_home(home: &Path) -> Vec<PathBuf> {
    let mut paths = codex_sqlite_dir_session_dbs(home);
    let legacy = legacy_state_db_path(home);
    if !paths.iter().any(|path| path == &legacy) {
        paths.push(legacy);
    }
    paths
}

pub fn codex_thread_reference_db_paths_from_home(home: &Path) -> Vec<PathBuf> {
    let sqlite_home = resolve_sqlite_home_home_or_default(home);
    let mut paths = codex_sqlite_dir_thread_reference_dbs(&sqlite_home);
    let legacy = legacy_state_db_path(&sqlite_home);
    if !paths.iter().any(|path| path == &legacy) {
        paths.push(legacy);
    }
    paths
}

pub fn codex_logs_db_path_from_home(home: &Path) -> PathBuf {
    let sqlite_home = resolve_sqlite_home_home_or_default(home);
    sqlite_home.join("logs_2.sqlite")
}

pub fn codex_sqlite_sidecar_paths(db_path: &Path) -> [PathBuf; 3] {
    [
        db_path.to_path_buf(),
        PathBuf::from(format!("{}-wal", db_path.to_string_lossy())),
        PathBuf::from(format!("{}-shm", db_path.to_string_lossy())),
    ]
}

pub fn relative_to_codex_home(home: &Path, path: &Path) -> PathBuf {
    path.strip_prefix(home).unwrap_or(path).to_path_buf()
}

fn resolve_sqlite_home_from_env() -> Option<PathBuf> {
    resolve_sqlite_home(std::env::var_os("CODEX_SQLITE_HOME"))
}

fn resolve_sqlite_home_home_or_default(home: &Path) -> PathBuf {
    resolve_sqlite_home_from_env().unwrap_or_else(|| home.to_path_buf())
}

fn resolve_sqlite_home(value: Option<OsString>) -> Option<PathBuf> {
    let path = PathBuf::from(value?);
    (!path.as_os_str().is_empty() && path.is_dir()).then_some(path)
}

fn legacy_state_db_path(home: &Path) -> PathBuf {
    home.join("state_5.sqlite")
}

fn codex_sqlite_dir_session_dbs(home: &Path) -> Vec<PathBuf> {
    codex_sqlite_dir_dbs_with_tables(home, &["threads", "automation_runs", "inbox_items"])
}

fn codex_sqlite_dir_thread_reference_dbs(home: &Path) -> Vec<PathBuf> {
    codex_sqlite_dir_dbs_with_tables(
        home,
        &[
            "threads",
            "local_thread_catalog",
            "automation_runs",
            "inbox_items",
            "sessions",
            "messages",
            "thread_dynamic_tools",
            "thread_goals",
            "thread_spawn_edges",
            "stage1_outputs",
            "agent_job_items",
        ],
    )
}

fn codex_sqlite_dir_dbs_with_tables(home: &Path, tables: &[&str]) -> Vec<PathBuf> {
    let sqlite_dir = home.join("sqlite");
    let Ok(entries) = fs::read_dir(sqlite_dir) else {
        return Vec::new();
    };
    let mut candidates = entries
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.is_file())
        .filter(|path| is_sqlite_candidate(path))
        .filter(|path| has_any_table(path, tables))
        .collect::<Vec<_>>();
    candidates.sort_by_key(|path| {
        (
            path.file_name()
                .map(|name| name != OsStr::new("codex-dev.db"))
                .unwrap_or(true),
            path.file_name().map(|name| name.to_os_string()),
        )
    });
    candidates
}

fn is_sqlite_candidate(path: &Path) -> bool {
    matches!(
        path.extension().and_then(OsStr::to_str),
        Some("db") | Some("sqlite") | Some("sqlite3")
    )
}

fn has_any_table(path: &Path, tables: &[&str]) -> bool {
    tables.iter().any(|table| sqlite_has_table(path, table))
}

fn sqlite_has_table(path: &Path, table: &str) -> bool {
    let Ok(db) = Connection::open_with_flags(path, rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY)
    else {
        return false;
    };
    db.query_row(
        "SELECT 1 FROM sqlite_master WHERE type = 'table' AND name = ?1 LIMIT 1",
        [table],
        |_| Ok(()),
    )
    .is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    static SQLITE_HOME_MUTEX: Mutex<()> = Mutex::new(());

    fn with_sqlite_home_env<T>(value: Option<&Path>, action: impl FnOnce() -> T) -> T {
        let _guard = SQLITE_HOME_MUTEX.lock().unwrap();
        let previous = std::env::var_os("CODEX_SQLITE_HOME");
        match value {
            Some(value) => unsafe { std::env::set_var("CODEX_SQLITE_HOME", value) },
            None => unsafe { std::env::remove_var("CODEX_SQLITE_HOME") },
        }
        let result = action();
        match previous {
            Some(value) => unsafe { std::env::set_var("CODEX_SQLITE_HOME", value) },
            None => unsafe { std::env::remove_var("CODEX_SQLITE_HOME") },
        }
        result
    }

    #[test]
    fn session_thread_reference_and_logs_paths_share_sqlite_home_override() {
        let temp = tempfile::tempdir().unwrap();
        let home = temp.path().join("home");
        let sqlite_home = temp.path().join("sqlite-override");
        std::fs::create_dir_all(&home).unwrap();
        std::fs::create_dir_all(sqlite_home.join("sqlite")).unwrap();

        let thread_reference_db = sqlite_home.join("sqlite").join("threads-reference.db");
        let db = rusqlite::Connection::open(&thread_reference_db).unwrap();
        db.execute("CREATE TABLE threads (id TEXT PRIMARY KEY)", [])
            .unwrap();
        drop(db);

        let session_db = sqlite_home.join("state_5.sqlite");
        let db = rusqlite::Connection::open(&session_db).unwrap();
        db.execute("CREATE TABLE threads (id TEXT PRIMARY KEY)", [])
            .unwrap();
        drop(db);
        std::fs::write(home.join("state_5.sqlite"), b"legacy").unwrap();

        with_sqlite_home_env(Some(&sqlite_home), || {
            let session_paths = codex_session_db_paths_from_home(&home);
            let thread_reference_paths = codex_thread_reference_db_paths_from_home(&home);
            let logs_path = codex_logs_db_path_from_home(&home);

            assert!(session_paths.contains(&session_db));
            assert!(!session_paths.iter().any(|path| path.starts_with(&home)));
            assert!(thread_reference_paths.contains(&thread_reference_db));
            assert_eq!(logs_path, sqlite_home.join("logs_2.sqlite"));
        });
    }

    #[test]
    fn missing_sqlite_home_override_falls_back_to_codex_home() {
        let temp = tempfile::tempdir().unwrap();
        let home = temp.path().join("home");
        let missing_sqlite_home = temp.path().join("missing-sqlite-home");
        let home_sqlite = home.join("sqlite");
        let home_db = home_sqlite.join("codex-dev.db");
        std::fs::create_dir_all(&home_sqlite).unwrap();
        let db = rusqlite::Connection::open(&home_db).unwrap();
        db.execute("CREATE TABLE threads (id TEXT PRIMARY KEY)", [])
            .unwrap();
        drop(db);

        with_sqlite_home_env(Some(&missing_sqlite_home), || {
            let session_paths = codex_session_db_paths_from_home(&home);
            assert!(session_paths.contains(&home_db));
            assert_eq!(
                codex_logs_db_path_from_home(&home),
                home.join("logs_2.sqlite")
            );
        });
    }
}
