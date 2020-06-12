use crate::{Dialog, Error, MessageAlert, MessageConfirm, MessageType, Result};
use osascript::JavaScript;
use serde::de::DeserializeOwned;
use serde::Serialize;

impl Dialog for MessageAlert<'_> {
    type Output = ();

    fn show(self) -> Result<Self::Output> {
        display_alert(DisplayAlertParams {
            title: self.title,
            text: self.text,
            icon: &get_dialog_icon(self.typ),
            buttons: &["OK"],
        })
    }
}

impl Dialog for MessageConfirm<'_> {
    type Output = bool;

    fn show(self) -> Result<Self::Output> {
        let button = display_alert(DisplayAlertParams {
            title: self.title,
            text: self.text,
            icon: &get_dialog_icon(self.typ),
            buttons: &["No", "Yes"],
        })?;

        match button {
            Some(t) => Ok(String::eq(&t, "Yes")),
            None => Ok(false),
        }
    }
}

#[derive(Serialize)]
struct DisplayAlertParams<'a> {
    title: &'a str,
    text: &'a str,
    icon: &'a str,
    buttons: &'a [&'a str],
}

fn get_dialog_icon(typ: MessageType) -> String {
    match typ {
        MessageType::Info => "note".into(),
        MessageType::Warning => "caution".into(),
        MessageType::Error => "stop".into(),
    }
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
            return app.displayDialog($params.message, options).buttonReturned;
        } catch (e) {
            return null;
        }
        ",
    );

    script.execute_with_params(params).map_err(Error::from)
}
