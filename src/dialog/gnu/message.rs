use super::{
    execute_command, get_kdialog_version, get_zenity_version, should_use, spawn_command, UseCommand,
};
use crate::dialog::{DialogImpl, MessageAlert, MessageConfirm};
use crate::{Error, MessageLevel, Result};
use std::process::Command;

impl MessageAlert {
    fn create(&self) -> Result<Command> {
        let params = Params {
            title: &self.title,
            text: &self.text,
            level: self.level,
            ask: false,
            attach: self.owner.and_then(|x| unsafe { x.as_x11() }),
        };

        let command = match should_use().ok_or(Error::NoImplementation)? {
            UseCommand::KDialog(cmd) => call_kdialog(cmd, params),
            UseCommand::Zenity(cmd) => call_zenity(cmd, params),
        };

        Ok(command)
    }
}

impl DialogImpl for MessageAlert {
    fn show(self) -> Result<Self::Output> {
        let command = self.create()?;
        execute_command(command)?;
        Ok(())
    }

    #[cfg(feature = "async")]
    async fn spawn(self) -> crate::Result<Self::Output> {
        let command = self.create()?;
        spawn_command(command).await?;
        Ok(())
    }
}

impl MessageConfirm {
    fn create(&self) -> Result<Command> {
        let params = Params {
            title: &self.title,
            text: &self.text,
            level: self.level,
            ask: true,
            attach: self.owner.and_then(|x| unsafe { x.as_x11() }),
        };

        let command = match should_use().ok_or(Error::NoImplementation)? {
            UseCommand::KDialog(cmd) => call_kdialog(cmd, params),
            UseCommand::Zenity(cmd) => call_zenity(cmd, params),
        };

        Ok(command)
    }
}

impl DialogImpl for MessageConfirm {
    fn show(self) -> Result<Self::Output> {
        let command = self.create()?;
        let output = execute_command(command)?;
        Ok(output.is_some())
    }

    #[cfg(feature = "async")]
    async fn spawn(self) -> crate::Result<Self::Output> {
        let command = self.create()?;
        let output = spawn_command(command).await?;
        Ok(output.is_some())
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
    level: MessageLevel,
    ask: bool,
    attach: Option<usize>,
}

fn call_kdialog(mut command: Command, params: Params) -> Command {
    if let Some(attach) = params.attach {
        command.arg("--attach");
        command.arg(attach.to_string());
    }

    if params.ask {
        command.arg("--yesno");
    } else {
        command.arg("--msgbox");
    }

    command.arg(convert_qt_text_document(params.text));

    command.arg("--title");
    command.arg(params.title);

    match params.level {
        MessageLevel::Info => command.arg("--icon=dialog-information"),
        MessageLevel::Warning => command.arg("--icon=dialog-warning"),
        MessageLevel::Error => command.arg("--icon=dialog-error"),
    };

    command
}

fn call_zenity(mut command: Command, params: Params) -> Command {
    if let Some(attach) = params.attach {
        command.arg("--attach");
        command.arg(attach.to_string());
    }

    command.arg("--width=400");

    if params.ask {
        command.arg("--question");

        // `--icon-name` was renamed to `--icon` at zenity 3.90.0
        match get_zenity_version() {
            Some(v) if v < (3, 90, 0) => command.arg("--icon-name"),
            _ => command.arg("--icon"),
        };
        match params.level {
            MessageLevel::Info => command.arg("dialog-information"),
            MessageLevel::Warning => command.arg("dialog-warning"),
            MessageLevel::Error => command.arg("dialog-error"),
        };
    } else {
        match params.level {
            MessageLevel::Info => command.arg("--info"),
            MessageLevel::Warning => command.arg("--warning"),
            MessageLevel::Error => command.arg("--error"),
        };
    }

    command.arg("--title");
    command.arg(params.title);

    command.arg("--text");
    command.arg(escape_pango_entities(params.text));

    command
}
