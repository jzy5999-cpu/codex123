use codex_plus_core::codex_sqlite::codex_session_db_path_from_home;
use rusqlite::Connection;
use std::fs;
use tempfile::tempdir;

#[test]
fn codex_session_db_path_prefers_threads_db_over_inbox_db() {
    let tmp = tempdir().unwrap();
    let home = tmp.path().join(".codex");
    let sqlite_dir = home.join("sqlite");
    fs::create_dir_all(&sqlite_dir).unwrap();

    let inbox_db = sqlite_dir.join("codex-dev.db");
    let threads_db = sqlite_dir.join("state.db");

    Connection::open(&inbox_db)
        .unwrap()
        .execute("CREATE TABLE inbox_items (id TEXT PRIMARY KEY)", [])
        .unwrap();
    Connection::open(&threads_db)
        .unwrap()
        .execute("CREATE TABLE threads (id TEXT PRIMARY KEY)", [])
        .unwrap();

    assert_eq!(codex_session_db_path_from_home(&home), threads_db);
}
