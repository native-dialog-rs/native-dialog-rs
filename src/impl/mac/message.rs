use crate::r#impl::DialogImpl;
use crate::{Error, MessageAlert, MessageConfirm, MessageType, Result};
use osascript::JavaScript;
use serde::de::DeserializeOwned;
use serde::Serialize;

impl DialogImpl for MessageAlert<'_> {
    type Output = ();

    fn show(&mut self) -> Result<Self::Output> {
        display_alert(DisplayAlertParams {
            title: self.title,
            text: self.text,
            icon: get_dialog_icon(self.typ),
            buttons: &["OK"],
        })
        .map(|_: String| ())
    }
}

impl DialogImpl for MessageConfirm<'_> {
    type Output = bool;

    fn show(&mut self) -> Result<Self::Output> {
        let button = display_alert(DisplayAlertParams {
            title: self.title,
            text: self.text,
            icon: get_dialog_icon(self.typ),
            buttons: &["No", "Yes"],
        })?;

        match button {
            Some(t) => Ok(String::eq(&t, "Yes")),
            None => Ok(false),
        }
    }
}

fn get_dialog_icon(typ: MessageType) -> &'static str {
    match typ {
        MessageType::Info => "note",
        MessageType::Warning => "caution",
        MessageType::Error => "stop",
    }
}

#[derive(Serialize)]
struct DisplayAlertParams<'a> {
    title: &'a str,
    text: &'a str,
    icon: &'a str,
    buttons: &'a [&'a str],
}

fn display_alert<T: DeserializeOwned>(params: DisplayAlertParams) -> Result<T> {
    let script = JavaScript::new(
        // language=js
        r"
        const app = Application.currentApplication();
        app.includeStandardAdditions = true;

        const options = {
            buttons: $params.buttons,
            withTitle: $params.title,
            withIcon: $params.icon,
        };

        try {
            return app.displayDialog($params.text, options).buttonReturned;
        } catch (e) {
            return null;
        }
        ",
    );

    script.execute_with_params(params).map_err(Error::from)
}
