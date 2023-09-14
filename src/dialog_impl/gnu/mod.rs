use crate::Error;
use std::env;
use std::process::Command;

mod file;
pub(crate) mod message;
mod progress;

enum UseCommand {
    KDialog(Command),
    Zenity(Command),
}

/// GMarkup flavoured XML has defined only 5 entities and doesn't support user-defined entities.
/// Should we reimplement the complete `g_markup_escape_text` function?
/// See https://gitlab.gnome.org/GNOME/glib/-/blob/353942c6/glib/gmarkup.c#L2296
fn escape_pango_entities(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

fn should_use() -> Option<UseCommand> {
    let has_display = match env::var("DISPLAY") {
        Ok(display) => !display.is_empty(),
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
