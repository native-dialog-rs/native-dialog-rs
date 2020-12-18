use crate::Error;
use std::env;
use std::process::Command;

mod file;
mod message;

enum UseCommand {
    KDialog(Command),
    Zenity(Command),
}

fn should_use() -> Option<UseCommand> {
    #[allow(clippy::match_like_matches_macro)]
    let has_display = match env::var("DISPLAY") {
        Ok(display) if !display.is_empty() => true,
        _ => false,
    };

    if has_display {
        // Prefer KDialog if the user is logged into a KDE session
        let kdialog_available = which::which("kdialog").is_ok();

        if let Ok(desktop) = env::var("XDG_CURRENT_DESKTOP") {
            if kdialog_available && desktop == "KDE" {
                return Some(UseCommand::KDialog(Command::new("kdialog")));
            }
        }

        // Prefer Zenity otherwise
        if which::which("zenity").is_ok() {
            return Some(UseCommand::Zenity(Command::new("zenity")));
        }

        if kdialog_available {
            return Some(UseCommand::KDialog(Command::new("kdialog")));
        }
    }

    None
}
