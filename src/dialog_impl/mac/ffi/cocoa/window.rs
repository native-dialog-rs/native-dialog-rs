use objc2::rc::Id;
use objc2_app_kit::{NSModalResponse, NSPanel, NSWindow};
use raw_window_handle::RawWindowHandle;

pub trait INSWindow {
    fn from_raw_handle(handle: RawWindowHandle) -> Option<Id<Self>>;

    fn begin_sheet(&self, sheet: &NSPanel);

    fn end_sheet(&self, sheet: &NSPanel, response: NSModalResponse);
}

impl INSWindow for NSWindow {
    fn from_raw_handle(handle: RawWindowHandle) -> Option<Id<Self>> {
        match handle {
            RawWindowHandle::AppKit(h) if !h.ns_window.is_null() => unsafe {
                Some(Id::retain(h.ns_window as _).unwrap())
            },
            _ => None,
        }
    }

    fn begin_sheet(&self, sheet: &NSPanel) {
        unsafe { self.beginSheet_completionHandler(sheet, None) }
    }

    fn end_sheet(&self, sheet: &NSPanel, response: NSModalResponse) {
        unsafe { self.endSheet_returnCode(sheet, response) }
    }
}
