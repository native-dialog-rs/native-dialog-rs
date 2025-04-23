use objc2_app_kit::NSAlert;
use objc2_foundation::{NSBundle, NSString};

use crate::MessageLevel;

use super::NSBundleExt;

pub trait NSAlertExt {
    fn set_level(&self, level: MessageLevel) -> bool;

    fn set_informative_text(&self, text: &str);
    fn set_message_text(&self, text: &str);
    fn add_button(&self, title: &str);
}

impl NSAlertExt for NSAlert {
    fn set_level(&self, level: MessageLevel) -> bool {
        let path = "/System/Library/CoreServices/CoreTypes.bundle";
        let Some(bundle) = NSBundle::of_path(path) else {
            return false;
        };

        let name = match level {
            MessageLevel::Info => "AlertNoteIcon",
            MessageLevel::Warning => "AlertCautionIcon",
            MessageLevel::Error => "AlertStopIcon",
        };

        let Some(image) = bundle.image_named(name) else {
            return false;
        };

        unsafe { self.setIcon(Some(&image)) };

        true
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
