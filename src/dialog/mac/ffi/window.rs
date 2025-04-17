use objc2::rc::Retained as Id;
use objc2::MainThreadMarker;
use objc2_app_kit::{NSModalResponse, NSPanel, NSView, NSWindow};

use crate::dialog::UnsafeWindowHandle;

pub trait NSWindowExt {
    fn from_handle(mtm: MainThreadMarker, handle: Option<UnsafeWindowHandle>) -> Option<Id<Self>>;
    fn begin_sheet(&self, sheet: &NSPanel);
    fn end_sheet(&self, sheet: &NSPanel, response: NSModalResponse);
}

impl NSWindowExt for NSWindow {
    fn from_handle(_mtm: MainThreadMarker, handle: Option<UnsafeWindowHandle>) -> Option<Id<Self>> {
        handle
            .and_then(|handle| unsafe { handle.as_appkit() })
            .and_then(|handle| {
                let view = handle.ns_view.as_ptr();
                let view = unsafe { Id::<NSView>::retain(view.cast()) };
                view.unwrap().window()
            })
    }

    fn begin_sheet(&self, sheet: &NSPanel) {
        unsafe { self.beginSheet_completionHandler(sheet, None) }
    }

    fn end_sheet(&self, sheet: &NSPanel, response: NSModalResponse) {
        unsafe { self.endSheet_returnCode(sheet, response) }
    }
}
