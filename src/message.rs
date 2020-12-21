use crate::dialog::{Dialog, DialogImpl, MessageAlert, MessageConfirm};
use crate::Result;

/// Represents the type of the message in the dialog.
#[derive(Copy, Clone)]
pub enum MessageType {
    Info,
    Warning,
    Error,
}

/// Builds and shows message dialogs.
pub struct MessageDialog<'a> {
    pub(crate) title: &'a str,
    pub(crate) text: &'a str,
    pub(crate) typ: MessageType,
}

impl<'a> MessageDialog<'a> {
    pub fn new() -> Self {
        MessageDialog {
            title: "",
            text: "",
            typ: MessageType::Info,
        }
    }

    /// Set the title of the dialog.
    pub fn set_title(mut self, title: &'a str) -> Self {
        self.title = title;
        self
    }

    /// Set the message text of the dialog.
    pub fn set_text(mut self, text: &'a str) -> Self {
        self.text = text;
        self
    }

    /// Set the type of the message. This usually affects the icon shown in the dialog.
    pub fn set_type(mut self, typ: MessageType) -> Self {
        self.typ = typ;
        self
    }

    /// Shows a dialog that alert users with some message.
    pub fn show_alert(self) -> Result<<MessageAlert<'a> as Dialog>::Output> {
        let mut dialog = MessageAlert {
            title: self.title,
            text: self.text,
            typ: self.typ,
        };
        dialog.show()
    }

    /// Shows a dialog that let users to choose Yes/No.
    pub fn show_confirm(self) -> Result<<MessageConfirm<'a> as Dialog>::Output> {
        let mut dialog = MessageConfirm {
            title: self.title,
            text: self.text,
            typ: self.typ,
        };
        dialog.show()
    }
}

impl Default for MessageDialog<'_> {
    fn default() -> Self {
        Self::new()
    }
}
