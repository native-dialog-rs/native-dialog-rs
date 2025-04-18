use super::backend::{Backend, BackendKind};
use crate::dialog::{DialogImpl, MessageAlert, MessageConfirm};
use crate::{MessageLevel, Result};

impl MessageAlert {
    fn create(&self) -> Result<Backend> {
        let params = Params {
            title: &self.title,
            text: &self.text,
            level: self.level,
            ask: false,
            owner: unsafe { self.owner.as_x11() },
        };

        let mut backend = Backend::new()?;
        match backend.kind {
            BackendKind::KDialog => call_kdialog(&mut backend, params),
            BackendKind::Zenity => call_zenity(&mut backend, params),
        };

        Ok(backend)
    }
}

impl DialogImpl for MessageAlert {
    fn show(self) -> Result<Self::Output> {
        let backend = self.create()?;
        backend.exec()?;
        Ok(())
    }

    #[cfg(feature = "async")]
    async fn spawn(self) -> Result<Self::Output> {
        let backend = self.create()?;
        backend.spawn().await?;
        Ok(())
    }
}

impl MessageConfirm {
    fn create(&self) -> Result<Backend> {
        let params = Params {
            title: &self.title,
            text: &self.text,
            level: self.level,
            ask: true,
            owner: unsafe { self.owner.as_x11() },
        };

        let mut backend = Backend::new()?;
        match backend.kind {
            BackendKind::KDialog => call_kdialog(&mut backend, params),
            BackendKind::Zenity => call_zenity(&mut backend, params),
        };

        Ok(backend)
    }
}

impl DialogImpl for MessageConfirm {
    fn show(self) -> Result<Self::Output> {
        let backend = self.create()?;
        let output = backend.exec()?;
        Ok(output.is_some())
    }

    #[cfg(feature = "async")]
    async fn spawn(self) -> Result<Self::Output> {
        let backend = self.create()?;
        let output = backend.spawn().await?;
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
fn convert_qt_text_document(backend: &Backend, text: &str) -> String {
    if matches!(backend.version(), Some(v) if v < (19, 0, 0)) {
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
    owner: Option<u64>,
}

fn call_kdialog(backend: &mut Backend, params: Params) {
    if let Some(owner) = params.owner {
        backend.command.arg(format!("--attach=0x{:x}", owner));
    }

    if params.ask {
        backend.command.arg("--yesno");
    } else {
        backend.command.arg("--msgbox");
    }

    let text = convert_qt_text_document(backend, params.text);
    backend.command.arg(text);

    backend.command.arg("--title");
    backend.command.arg(params.title);

    match params.level {
        MessageLevel::Info => backend.command.arg("--icon=dialog-information"),
        MessageLevel::Warning => backend.command.arg("--icon=dialog-warning"),
        MessageLevel::Error => backend.command.arg("--icon=dialog-error"),
    };
}

fn call_zenity(backend: &mut Backend, params: Params) {
    backend.command.arg("--width=400");

    if params.ask {
        backend.command.arg("--question");

        // `--icon-name` was renamed to `--icon` at zenity 3.90.0
        match backend.version() {
            Some(v) if v < (3, 90, 0) => backend.command.arg("--icon-name"),
            _ => backend.command.arg("--icon"),
        };
        match params.level {
            MessageLevel::Info => backend.command.arg("dialog-information"),
            MessageLevel::Warning => backend.command.arg("dialog-warning"),
            MessageLevel::Error => backend.command.arg("dialog-error"),
        };
    } else {
        match params.level {
            MessageLevel::Info => backend.command.arg("--info"),
            MessageLevel::Warning => backend.command.arg("--warning"),
            MessageLevel::Error => backend.command.arg("--error"),
        };
    }

    backend.command.arg("--title");
    backend.command.arg(params.title);

    let text = escape_pango_entities(params.text);
    backend.command.arg("--text");
    backend.command.arg(text);
}
