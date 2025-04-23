use objc2_app_kit::{NSAlert, NSImage};
use objc2_foundation::{NSBundle, NSString};

use super::{NSBundleExt, NSImageExt};
use crate::MessageLevel;

pub trait NSAlertExt {
    fn set_level_icon(&self, level: MessageLevel);

    fn set_informative_text(&self, text: &str);
    fn set_message_text(&self, text: &str);
    fn add_button(&self, title: &str);
}

impl NSAlertExt for NSAlert {
    fn set_level_icon(&self, level: MessageLevel) {
        let bundle = "/System/Library/CoreServices/CoreTypes.bundle";
        let name = match level {
            MessageLevel::Info => "AlertNoteIcon",
            MessageLevel::Warning => "AlertCautionIcon",
            MessageLevel::Error => "AlertStopIcon",
        };

        if let Some(icon) = NSBundle::from_path(bundle).and_then(|x| x.image(name)) {
            return unsafe { self.setIcon(Some(&icon)) };
        };

        let icon = NSImage::emoji(match level {
            MessageLevel::Info => "💡",
            MessageLevel::Warning => "⚠️",
            MessageLevel::Error => "🛑",
        });

        unsafe { self.setIcon(Some(&icon)) };
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
