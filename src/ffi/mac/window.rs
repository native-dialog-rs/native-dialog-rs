use objc2::rc::Retained as Id;
use objc2_app_kit::{NSModalResponse, NSPanel, NSView, NSWindow};
use raw_window_handle::AppKitWindowHandle;

pub trait NSWindowExt {
    fn from_raw(handle: AppKitWindowHandle) -> Option<Id<Self>>;
    fn begin_sheet(&self, sheet: &NSPanel);
    fn end_sheet(&self, sheet: &NSPanel, response: NSModalResponse);
}

impl NSWindowExt for NSWindow {
    fn from_raw(handle: AppKitWindowHandle) -> Option<Id<Self>> {
        let view = handle.ns_view.as_ptr();
        let view = unsafe { Id::<NSView>::retain(view.cast()) };
        view.and_then(|x| x.window())
    }

    fn begin_sheet(&self, sheet: &NSPanel) {
        unsafe { self.beginSheet_completionHandler(sheet, None) }
    }

    fn end_sheet(&self, sheet: &NSPanel, response: NSModalResponse) {
        unsafe { self.endSheet_returnCode(sheet, response) }
    }
}
