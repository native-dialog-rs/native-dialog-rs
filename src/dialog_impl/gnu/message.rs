use ascii::AsAsciiStr;

use super::{escape_pango_entities, should_use, UseCommand};
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

/// See https://github.com/qt/qtbase/blob/2e2f1e2/src/gui/text/qtextdocument.cpp#L166
fn convert_qt_text_document(text: &str) -> String {
    if let Some(version) = get_kdialog_version() {
        if version.0 <= 19 {
            return text
                .replace('&', "&amp;")
                .replace('<', "&lt;")
                .replace('>', "&gt;")
                .replace('"', "&quot;")
                .replace('\n', "<br>")
                .replace(' ', "&nbsp")
                .replace('\t', "&nbsp;");
        }
    }

    text.replace('\n', "<br>")
        .replace('\t', " ")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

pub(crate) fn get_kdialog_version() -> Option<(i32, i32, i32)> {
    let mut command = Command::new("kdialog");
    command.arg("--version");

    let output = command.output().ok()?;
    let stdout = output.stdout.as_ascii_str().ok()?;
    let ver_str = stdout.to_string();

    let mut split = ver_str.split(".");

    let major_with_name = split.next()?;
    let major_ver_str = major_with_name.split(" ").last()?;
    let major_ver = major_ver_str.parse::<i32>().ok()?;

    let minor_ver_str = split.next()?;
    let minor_ver = minor_ver_str.parse::<i32>().ok()?;

    let patch_str_ascii = split.next()?;
    let patch_ver = patch_str_ascii.replace('\n', "").parse::<i32>().ok()?;

    Some((major_ver, minor_ver, patch_ver))
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
