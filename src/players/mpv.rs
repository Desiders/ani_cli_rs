use subprocess;

pub fn is_installed() -> bool {
    match subprocess::Exec::cmd("mpv")
        .args(&["--version"])
        .stdout(subprocess::Redirection::Pipe)
        .stderr(subprocess::Redirection::Merge)
        .capture()
    {
        Ok(output) => output.exit_status.success(),
        Err(_) => false,
    }
}

/// Launch the player
pub fn launch(url: &str) -> Result<(), subprocess::PopenError> {
    subprocess::Popen::create(
        &["mpv", url, "--fs"],
        subprocess::PopenConfig {
            stdin: subprocess::Redirection::None,
            stdout: subprocess::Redirection::Pipe,
            stderr: subprocess::Redirection::Merge,
            ..Default::default()
        },
    )?;

    Ok(())
}
