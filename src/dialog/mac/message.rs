use objc2_core_foundation::kCFUserNotificationDefaultResponse;

use crate::dialog::{DialogImpl, MessageAlert, MessageConfirm};
use crate::ffi::mac::UserNotification;
use crate::Result;

impl DialogImpl for MessageAlert {
    fn show(self) -> Result<Self::Output> {
        UserNotification::new(&self.title, &self.text, self.level, false).alert();
        Ok(())
    }

    #[cfg(feature = "async")]
    async fn spawn(self) -> Result<Self::Output> {
        use crate::ffi::mac::{NSAlertAsyncExt, NSAlertExt};
        use dispatch2::run_on_main;
        use objc2_app_kit::NSAlert;

        let res = run_on_main(|mtm| {
            let alert = unsafe { NSAlert::new(mtm) };

            alert.set_informative_text(&self.text);
            alert.set_message_text(&self.title);
            alert.set_level_icon(self.level);

            alert.spawn(self.owner)
        });

        res.await;
        Ok(())
    }
}

impl DialogImpl for MessageConfirm {
    fn show(self) -> Result<Self::Output> {
        let res = UserNotification::new(&self.title, &self.text, self.level, true).alert();
        Ok(res == kCFUserNotificationDefaultResponse)
    }

    #[cfg(feature = "async")]
    async fn spawn(self) -> Result<Self::Output> {
        use crate::ffi::mac::{NSAlertAsyncExt, NSAlertExt};
        use dispatch2::run_on_main;
        use objc2_app_kit::{NSAlert, NSAlertFirstButtonReturn};

        let res = run_on_main(|mtm| {
            let alert = unsafe { NSAlert::new(mtm) };

            alert.set_informative_text(&self.text);
            alert.set_message_text(&self.title);
            alert.set_level_icon(self.level);

            alert.add_button("Yes");
            alert.add_button("No");

            alert.spawn(self.owner)
        });

        Ok(res.await == NSAlertFirstButtonReturn)
    }
}
