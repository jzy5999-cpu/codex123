#[cfg(target_os = "macos")]
pub const SKY_COMPUTER_USE_CLIENT: &str = "SkyComputerUseClient";

#[cfg(target_os = "macos")]
pub fn sky_computer_use_cleanup_command() -> std::process::Command {
    let mut command = std::process::Command::new("pkill");
    command.arg("-f").arg(SKY_COMPUTER_USE_CLIENT);
    command
}

#[cfg(target_os = "macos")]
pub fn kill_orphaned_computer_use_processes() {
    let mut command = sky_computer_use_cleanup_command();
    let _ = command
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status();
}

#[cfg(not(target_os = "macos"))]
pub fn kill_orphaned_computer_use_processes() {}
