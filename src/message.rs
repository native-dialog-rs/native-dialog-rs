use crate::dialog::{Dialog, DialogImpl, MessageAlert, MessageConfirm};
use crate::Result;

#[derive(Copy, Clone)]
pub enum MessageType {
    Info,
    Warning,
    Error,
}

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

    pub fn set_title(mut self, title: &'a str) -> Self {
        self.title = title;
        self
    }

    pub fn set_text(mut self, text: &'a str) -> Self {
        self.text = text;
        self
    }

    pub fn set_type(mut self, typ: MessageType) -> Self {
        self.typ = typ;
        self
    }

    pub fn alert(self) -> Result<<MessageAlert<'a> as Dialog>::Output> {
        let mut dialog = MessageAlert {
            title: self.title,
            text: self.text,
            typ: self.typ,
        };
        dialog.show()
    }

    pub fn confirm(self) -> Result<<MessageConfirm<'a> as Dialog>::Output> {
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
