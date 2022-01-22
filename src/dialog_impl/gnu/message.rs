use super::{should_use, UseCommand};
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
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\n', "<br>")
        .replace(&[' ', '\t'], "&nbsp;")
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
        Some(_) => Ok(false),
        _ => Err(Error::UnexpectedOutput("kdialog")),
    }
}

fn call_zenity(mut command: Command, params: Params) -> Result<bool> {
    command.arg("--width=400");

    if params.ask {
        command.arg("--question");
        match params.typ {
            MessageType::Info => command.arg("--icon-name=dialog-information"),
            MessageType::Warning => command.arg("--icon-name=dialog-warning"),
            MessageType::Error => command.arg("--icon-name=dialog-error"),
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

    match output.status.code() {
        Some(0) => Ok(true),
        Some(_) => Ok(false),
        _ => Err(Error::UnexpectedOutput("zenity")),
    }
}
