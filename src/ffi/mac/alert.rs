use objc2_app_kit::NSAlert;
use objc2_foundation::{NSBundle, NSString};

use super::NSBundleExt;
use crate::MessageLevel;

pub trait NSAlertExt {
    fn set_level_icon(&self, level: MessageLevel) -> bool;

    fn set_informative_text(&self, text: &str);
    fn set_message_text(&self, text: &str);
    fn add_button(&self, title: &str);
}

impl NSAlertExt for NSAlert {
    fn set_level_icon(&self, level: MessageLevel) -> bool {
        let bundle = "/System/Library/CoreServices/CoreTypes.bundle";
        let name = match level {
            MessageLevel::Info => "AlertNoteIcon",
            MessageLevel::Warning => "AlertCautionIcon",
            MessageLevel::Error => "AlertStopIcon",
        };

        let icon = NSBundle::from_path(bundle).and_then(|x| x.image(name));
        unsafe { self.setIcon(icon.as_deref()) };

        icon.is_some()
    }

    fn set_informative_text(&self, text: &str) {
        let text = NSString::from_str(text);
        unsafe { self.setInformativeText(&text) };
    }

    fn set_message_text(&self, text: &str) {
        let text = NSString::from_str(text);
        unsafe { self.setMessageText(&text) };
    }

    fn add_button(&self, title: &str) {
        let title = NSString::from_str(title);
        unsafe { self.addButtonWithTitle(&title) };
    }
}
