use crate::Error;
use ascii::AsAsciiStr;
use std::process::Command;
use versions::SemVer;

mod file;
pub(crate) mod message;

enum UseCommand {
    KDialog(Command),
    Zenity(Command),
}

fn should_use() -> Option<UseCommand> {
    let has_display = match std::env::var("DISPLAY") {
        Ok(display) => !display.is_empty(),
        _ => false,
    };
    if !has_display {
        return None;
    }

    let candidates = match std::env::var("XDG_CURRENT_DESKTOP").as_deref() {
        Ok("KDE") => [use_kdialog, use_zenity],
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

fn get_kdialog_version() -> Option<SemVer> {
    get_version_output("kdialog")
        .as_deref()
        .and_then(|s| s.split_whitespace().last())
        .and_then(SemVer::new)
}

fn get_zenity_version() -> Option<SemVer> {
    get_version_output("zenity")
        .as_deref()
        .and_then(SemVer::new)
}

fn get_version_output(program: &str) -> Option<String> {
    let output = Command::new(program).arg("--version").output().ok()?;
    Some(output.stdout.as_ascii_str().ok()?.to_string())
}
