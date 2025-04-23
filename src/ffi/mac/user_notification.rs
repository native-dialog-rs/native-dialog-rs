use objc2_core_foundation::{
    kCFUserNotificationCautionAlertLevel, kCFUserNotificationNoteAlertLevel,
    kCFUserNotificationStopAlertLevel, CFOptionFlags, CFRetained, CFString, CFUserNotification,
};

use crate::dialog::MessageLevel;

pub struct UserNotification {
    header: CFRetained<CFString>,
    message: CFRetained<CFString>,
    flag: CFOptionFlags,
    confirm: bool,
}

impl UserNotification {
    pub fn new(title: &str, message: &str, level: MessageLevel, confirm: bool) -> Self {
        UserNotification {
            header: CFString::from_str(title),
            message: CFString::from_str(message),
            flag: match level {
                MessageLevel::Info => kCFUserNotificationNoteAlertLevel,
                MessageLevel::Warning => kCFUserNotificationCautionAlertLevel,
                MessageLevel::Error => kCFUserNotificationStopAlertLevel,
            },
            confirm,
        }
    }

    pub fn alert(&self) -> usize {
        let default = CFString::from_static_str("Yes");
        let alternate = CFString::from_static_str("No");

        let mut response = 0;

        unsafe {
            CFUserNotification::display_alert(
                0f64,
                self.flag,
                None,
                None,
                None,
                Some(&self.header),
                Some(&self.message),
                self.confirm.then_some(&default),
                self.confirm.then_some(&alternate),
                None,
                &mut response,
            );
        }

        response
    }
}
