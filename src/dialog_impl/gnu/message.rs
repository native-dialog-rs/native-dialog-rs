use super::{get_kdialog_version, get_zenity_version, should_use, UseCommand};
use crate::dialog::{DialogImpl, MessageAlert, MessageConfirm};
use crate::{Error, MessageType, Result};
use std::process::Command;

impl DialogImpl for MessageAlert<'_> {
    fn show(&mut self) -> Result<Self::Output> {
        let command = should_use().ok_or(Error::NoImplementation)?;

        let params = Params {
            title: self.title,
            text: self.text,
            typ: self.typ,
            ask: false,
        };

        match command {
            UseCommand::KDialog(cmd) => call_kdialog(cmd, params)?,
            UseCommand::Zenity(cmd) => call_zenity(cmd, params)?,
        };

        Ok(())
    }
}

impl DialogImpl for MessageConfirm<'_> {
    fn show(&mut self) -> Result<Self::Output> {
        let command = should_use().ok_or(Error::NoImplementation)?;

        let params = Params {
            title: self.title,
            text: self.text,
            typ: self.typ,
            ask: true,
        };

        match command {
            UseCommand::KDialog(cmd) => call_kdialog(cmd, params),
            UseCommand::Zenity(cmd) => call_zenity(cmd, params),
        }
    }
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

/// See https://github.com/qt/qtbase/blob/2e2f1e2/src/gui/text/qtextdocument.cpp#L166
fn convert_qt_text_document(text: &str) -> String {
    if matches!(get_kdialog_version(), Some(v) if v < (19, 0, 0)) {
        text.replace('\n', "<br>")
            .replace('\t', " ")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('&', "&amp;")
            .replace('"', "&quot;")
    } else {
        text.replace('\n', "<br>")
            .replace('\t', " ")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
    }
}

struct Params<'a> {
    title: &'a str,
    text: &'a str,
    typ: MessageType,
    ask: bool,
}

fn call_kdialog(mut command: Command, params: Params) -> Result<bool> {
    if params.ask {
        command.arg("--yesno");
    } else {
        command.arg("--msgbox");
    }

    command.arg(convert_qt_text_document(params.text));

    command.arg("--title");
    command.arg(params.title);

    match params.typ {
        MessageType::Info => command.arg("--icon=dialog-information"),
        MessageType::Warning => command.arg("--icon=dialog-warning"),
        MessageType::Error => command.arg("--icon=dialog-error"),
    };

    let output = command.output()?;

    match output.status.code() {
        Some(0) => Ok(true),
        Some(1) => Ok(false),
        Some(exit_code) => Err(Error::UnexpectedOutput(format!(
            "'kdialog' exit code:{} with error:{}",
            exit_code,
            std::str::from_utf8(&output.stderr).unwrap_or_default()
        ))),
        None => Err(Error::UnexpectedOutput(
            "'kdialog' process terminated by signal".to_string(),
        )),
    }
}

fn call_zenity(mut command: Command, params: Params) -> Result<bool> {
    command.arg("--width=400");

    if params.ask {
        command.arg("--question");

        // `--icon-name` was renamed to `--icon` at zenity 3.90.0
        match get_zenity_version() {
            Some(v) if v < (3, 90, 0) => command.arg("--icon-name"),
            _ => command.arg("--icon"),
        };
        match params.typ {
            MessageType::Info => command.arg("dialog-information"),
            MessageType::Warning => command.arg("dialog-warning"),
            MessageType::Error => command.arg("dialog-error"),
        };
    } else {
        match params.typ {
            MessageType::Info => command.arg("--info"),
            MessageType::Warning => command.arg("--warning"),
            MessageType::Error => command.arg("--error"),
        };
    }

    command.arg("--title");
    command.arg(params.title);

    command.arg("--text");
    command.arg(escape_pango_entities(params.text));

    let output = command.output()?;

    // "zenity" exit codes are '0', '1', '5', '-1' (https://help.gnome.org/users/zenity/stable/usage.html.en)
    match output.status.code() {
        Some(0) => Ok(true),
        Some(1) => Ok(false),
        Some(exit_code) => Err(Error::UnexpectedOutput(format!(
            "'zenity' exit code:{} with error:{}",
            exit_code,
            std::str::from_utf8(&output.stderr).unwrap_or_default()
        ))),
        None => Err(Error::UnexpectedOutput(
            "'zenity' process terminated by signal".to_string(),
        )),
    }
}
