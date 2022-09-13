use super::ffi::user_notification::CFUserNotification;
use crate::dialog::{DialogImpl, MessageAlert, MessageConfirm};
use crate::{MessageType, Result};

impl DialogImpl for MessageAlert<'_> {
    fn show(&mut self) -> Result<Self::Output> {
        let alert = CFUserNotification {
            header: self.title,
            message: self.text,
            icon: get_dialog_icon(self.typ),
            default_button_title: None,
            alternate_button_title: None,
        };
        alert.display_alert();
        Ok(())
    }
}

impl DialogImpl for MessageConfirm<'_> {
    fn show(&mut self) -> Result<Self::Output> {
        let alert = CFUserNotification {
            header: self.title,
            message: self.text,
            icon: get_dialog_icon(self.typ),
            default_button_title: Some("Yes"),
            alternate_button_title: Some("No"),
        };

        let res = alert.display_alert();

        // kCFUserNotificationDefaultResponse = 0
        Ok(res == 0)
    }
}

fn get_dialog_icon(typ: MessageType) -> usize {
    match typ {
        MessageType::Info => 1,
        MessageType::Warning => 2,
        MessageType::Error => 0,
    }
}
