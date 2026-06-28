#[cfg(target_os = "macos")]
#[test]
fn macos_cleanup_command_targets_only_sky_computer_use_client() {
    let command = codex_plus_core::computer_use_cleanup::sky_computer_use_cleanup_command();
    let program = command.get_program().to_string_lossy();
    let args = command
        .get_args()
        .map(|arg| arg.to_string_lossy().to_string())
        .collect::<Vec<_>>();

    assert_eq!(program, "pkill");
    assert_eq!(
        args,
        vec![
            "-f".to_string(),
            codex_plus_core::computer_use_cleanup::SKY_COMPUTER_USE_CLIENT.to_string()
        ]
    );
    assert!(!args.iter().any(|arg| arg.contains("node_repl")));
}

#[cfg(not(target_os = "macos"))]
#[test]
fn non_macos_cleanup_is_a_noop() {
    codex_plus_core::computer_use_cleanup::kill_orphaned_computer_use_processes();
}
