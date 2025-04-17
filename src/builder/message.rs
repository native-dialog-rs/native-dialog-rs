use raw_window_handle::HasWindowHandle;

use crate::dialog::{MessageAlert, MessageConfirm};
use crate::utils::UnsafeWindowHandle;

pub use crate::dialog::MessageLevel;

/// Builds and shows message dialogs.
#[derive(Debug, Clone, Default)]
pub struct MessageDialogBuilder {
    pub title: String,
    pub text: String,
    pub level: MessageLevel,
    pub owner: UnsafeWindowHandle,
}

impl MessageDialogBuilder {
    /// Set the title of the dialog.
    pub fn set_title(mut self, title: impl ToString) -> Self {
        self.title = title.to_string();
        self
    }

    /// Set the message text of the dialog.
    pub fn set_text(mut self, text: impl ToString) -> Self {
        self.text = text.to_string();
        self
    }

    /// Set the level of the message. This usually affects the icon shown in the dialog.
    pub fn set_level(mut self, level: MessageLevel) -> Self {
        self.level = level;
        self
    }

    /// Sets the owner of the dialog.
    pub fn set_owner<W: HasWindowHandle>(mut self, window: &W) -> Self {
        self.owner = UnsafeWindowHandle::new(window);
        self
    }

    /// Resets the owner of the dialog to nothing.
    pub fn reset_owner(mut self) -> Self {
        self.owner = UnsafeWindowHandle::default();
        self
    }

    /// Shows a dialog that alert users with some message.
    pub fn alert(self) -> MessageAlert {
        MessageAlert {
            title: self.title,
            text: self.text,
            level: self.level,
            owner: self.owner,
        }
    }

    /// Shows a dialog that let users to choose Yes/No.
    pub fn confirm(self) -> MessageConfirm {
        MessageConfirm {
            title: self.title,
            text: self.text,
            level: self.level,
            owner: self.owner,
        }
    }
}
