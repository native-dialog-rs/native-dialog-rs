use super::Dialog;
use crate::utils::UnsafeWindowHandle;

/// Represents the type of the message in the dialog.
#[derive(Debug, Clone, Copy)]
pub enum MessageLevel {
    Info,
    Warning,
    Error,
}

impl Default for MessageLevel {
    fn default() -> Self {
        Self::Info
    }
}

pub struct MessageAlert {
    pub title: String,
    pub text: String,
    pub level: MessageLevel,
    pub owner: UnsafeWindowHandle,
}

impl Dialog for MessageAlert {
    type Output = ();
}

impl MessageAlert {
    super::dialog_delegate!();
}

pub struct MessageConfirm {
    pub title: String,
    pub text: String,
    pub level: MessageLevel,
    pub owner: UnsafeWindowHandle,
}

impl Dialog for MessageConfirm {
    type Output = bool;
}

impl MessageConfirm {
    super::dialog_delegate!();
}
