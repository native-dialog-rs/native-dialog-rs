use super::ffi::UserNotificationAlert;
use crate::dialog::{MessageAlert, MessageConfirm};
use crate::dialog_impl::DialogImpl;
use crate::{MessageType, Result};

impl<'a> DialogImpl for MessageAlert<'a> {
    type Impl = UserNotificationAlert<'a>;

    fn create(&self) -> Self::Impl {
        UserNotificationAlert {
            header: self.title,
            message: self.text,
            icon: get_dialog_icon(self.typ),
            confirm: false,
        }
    }

    fn show(&mut self) -> Result<Self::Output> {
        self.create().display();
        Ok(())
    }

    #[cfg(feature = "async")]
    async fn spawn(&mut self) -> Result<Self::Output> {
        self.show()
    }
}

impl<'a> DialogImpl for MessageConfirm<'a> {
    type Impl = UserNotificationAlert<'a>;

    fn create(&self) -> Self::Impl {
        UserNotificationAlert {
            header: self.title,
            message: self.text,
            icon: get_dialog_icon(self.typ),
            confirm: true,
        }
    }

    fn show(&mut self) -> Result<Self::Output> {
        let res = self.create().display();

        // kCFUserNotificationDefaultResponse = 0
        Ok(res == 0)
    }

    #[cfg(feature = "async")]
    async fn spawn(&mut self) -> Result<Self::Output> {
        self.show()
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
