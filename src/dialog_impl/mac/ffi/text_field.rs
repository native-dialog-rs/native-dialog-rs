use objc2::rc::{Retained as Id};
use objc2_app_kit::{NSColor, NSTextField};
use objc2_foundation::{MainThreadMarker, NSString};

pub trait NSTextFieldExt {
    fn label_with_string(string: &str) -> Id<Self>;

    fn set_text_color(&self, color: &NSColor);
}

impl NSTextFieldExt for NSTextField {
    fn label_with_string(string: &str) -> Id<Self> {
        // TODO: Main Thread Safety
        let mtm = unsafe { MainThreadMarker::new_unchecked() };
        let string = NSString::from_str(string);
        unsafe { NSTextField::labelWithString(&string, mtm) }
    }

    fn set_text_color(&self, color: &NSColor) {
        unsafe { self.setTextColor(Some(color)) }
    }
}
