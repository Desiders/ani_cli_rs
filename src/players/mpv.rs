use subprocess;

const CHECK_VERSION: &[&str] = &["mpv", "--version"];

/// Check if player is installed
pub fn check_installation() -> bool {
    match subprocess::Exec::cmd(CHECK_VERSION[0])
        .args(&CHECK_VERSION[1..])
        .stdout(subprocess::Redirection::Pipe)
        .stderr(subprocess::Redirection::Merge)
        .capture()
    {
        Ok(output) => output.exit_status.success(),
        Err(_) => false,
    }
}
