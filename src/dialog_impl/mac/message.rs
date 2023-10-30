use super::ffi::UserNotificationAlert;
use crate::dialog::{DialogImpl, MessageAlert, MessageConfirm};
use crate::{MessageType, Result};

impl DialogImpl for MessageAlert<'_> {
    fn show(&mut self) -> Result<Self::Output> {
        let alert = UserNotificationAlert {
            header: self.title,
            message: self.text,
            icon: get_dialog_icon(self.typ),
            confirm: false,
        };
        alert.display();

        Ok(())
    }
}

impl DialogImpl for MessageConfirm<'_> {
    fn show(&mut self) -> Result<Self::Output> {
        let alert = UserNotificationAlert {
            header: self.title,
            message: self.text,
            icon: get_dialog_icon(self.typ),
            confirm: true,
        };
        let res = alert.display();

        // kCFUserNotificationDefaultResponse = 0
        Ok(res == 0)
    }
}

fn get_dialog_icon(typ: MessageType) -> usize {
    match typ {
        // kCFUserNotificationNoteAlertLevel = 1
        MessageType::Info => 1,
        // kCFUserNotificationCautionAlertLevel = 2
        MessageType::Warning => 2,
        // kCFUserNotificationStopAlertLevel = 0
        MessageType::Error => 0,
    }
}
