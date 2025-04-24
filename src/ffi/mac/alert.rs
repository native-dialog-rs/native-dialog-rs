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
        let icon = NSBundle::from_path(bundle)
            .and_then(|bundle| {
                bundle.image(match level {
                    MessageLevel::Info => "AlertNoteIcon",
                    MessageLevel::Warning => "AlertCautionIcon",
                    MessageLevel::Error => "AlertStopIcon",
                })
            })
            .unwrap_or_else(|| match level {
                MessageLevel::Info => NSImage::stack(
                    &NSImage::text("⚪", 1.0, true),
                    &NSImage::text("𝒊", 0.667, false).etched(),
                    (0.0, 0.667),
                ),
                MessageLevel::Warning => NSImage::text("⚠️", 1.0, true),
                MessageLevel::Error => NSImage::stack(
                    &NSImage::text("🛑", 1.0, true),
                    &NSImage::text("❕", 0.6, true),
                    (1.0, -0.5),
                ),
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
