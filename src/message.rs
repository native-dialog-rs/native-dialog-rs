use crate::dialog::{DialogImpl, MessageAlert, MessageConfirm};
use crate::Result;
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};

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
    pub(crate) owner: Option<RawWindowHandle>,
}

impl<'a> MessageDialog<'a> {
    pub fn new() -> Self {
        MessageDialog {
            title: "",
            text: "",
            typ: MessageType::Info,
            owner: None,
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

    /// Sets the owner of the dialog. On Unix and GNU/Linux, this is a no-op.
    pub fn set_owner<W: HasRawWindowHandle>(mut self, window: &W) -> Self {
        self.owner = Some(window.raw_window_handle());
        self
    }

    /// Sets the owner of the dialog by raw handle. On Unix and GNU/Linux, this is a no-op.
    ///
    /// # Safety
    ///
    /// It's the caller's responsibility that ensuring the handle is valid.
    pub unsafe fn set_owner_handle(mut self, handle: RawWindowHandle) -> Self {
        self.owner = Some(handle);
        self
    }

    /// Resets the owner of the dialog to nothing.
    pub fn reset_owner(mut self) -> Self {
        self.owner = None;
        self
    }

    /// Shows a dialog that alert users with some message.
    pub fn show_alert(self) -> Result<()> {
        let mut dialog = MessageAlert {
            title: self.title,
            text: self.text,
            typ: self.typ,
            owner: self.owner,
        };
        dialog.show()
    }

    /// Shows a dialog that let users to choose Yes/No.
    pub fn show_confirm(self) -> Result<bool> {
        let mut dialog = MessageConfirm {
            title: self.title,
            text: self.text,
            typ: self.typ,
            owner: self.owner,
        };
        dialog.show()
    }
}

impl Default for MessageDialog<'_> {
    fn default() -> Self {
        Self::new()
    }
}
