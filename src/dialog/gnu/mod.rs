use std::process::Command;

use ascii::AsAsciiStr;
use version::Version;

use crate::{Error, Result};

mod file;
mod message;
mod version;

enum UseCommand {
    KDialog(Command),
    Zenity(Command),
}

fn should_use() -> Option<UseCommand> {
    let has_display = match std::env::var("DISPLAY") {
        Ok(display) => !display.is_empty(),
        _ => false,
    };

    let candidates = match std::env::var("XDG_CURRENT_DESKTOP").as_deref() {
        Ok("KDE") if has_display => [use_kdialog, use_zenity],
        _ => [use_zenity, use_kdialog],
    };

    for candidate in candidates {
        if let Some(command) = candidate() {
            return Some(command);
        }
    }

    None
}

fn use_kdialog() -> Option<UseCommand> {
    let path = which::which("kdialog").ok()?;
    let command = Command::new(path);
    Some(UseCommand::KDialog(command))
}

fn use_zenity() -> Option<UseCommand> {
    let path = which::which("zenity").ok()?;
    let command = Command::new(path);
    Some(UseCommand::Zenity(command))
}

fn get_kdialog_version() -> Option<Version> {
    get_version_output("kdialog")
        .as_deref()
        .and_then(|s| s.split_whitespace().last())
        .and_then(Version::parse)
}

fn get_zenity_version() -> Option<Version> {
    get_version_output("zenity")
        .as_deref()
        .and_then(Version::parse)
}

fn get_version_output(program: &str) -> Option<String> {
    let output = Command::new(program).arg("--version").output().ok()?;
    Some(output.stdout.as_ascii_str().ok()?.to_string())
}

pub fn execute_command(mut command: Command) -> Result<Option<Vec<u8>>> {
    let program = command.get_program().to_os_string();
    let output = command.output()?;

    match output.status.code() {
        Some(0) => Ok(Some(output.stdout)),
        Some(_) => Ok(None),
        None => Err(Error::Killed(program)),
    }
}

#[cfg(feature = "async")]
pub async fn spawn_command(command: Command) -> Result<Option<Vec<u8>>> {
    let (send, recv) = futures_channel::oneshot::channel();

    std::thread::spawn(move || {
        let _ = send.send(execute_command(command));
    });

    recv.await.unwrap_or(Ok(None))
}
