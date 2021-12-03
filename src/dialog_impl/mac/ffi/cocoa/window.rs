use super::INSPanel;
use cocoa::base::nil;
use cocoa::foundation::NSInteger;
use objc_foundation::INSObject;
use objc_id::Id;
use raw_window_handle::RawWindowHandle;

pub trait INSWindow: INSObject {
    fn from_raw_handle(handle: RawWindowHandle) -> Option<Id<Self>> {
        match handle {
            RawWindowHandle::AppKit(h) if !h.ns_window.is_null() => unsafe {
                Some(Id::from_ptr(h.ns_window as _))
            },
            _ => None,
        }
    }

    fn begin_sheet(&self, sheet: &impl INSPanel) {
        unsafe { msg_send![self, beginSheet:sheet completionHandler:nil] }
    }

    fn end_sheet(&self, sheet: &impl INSPanel, response: NSInteger) {
        unsafe { msg_send![self, endSheet:sheet returnCode:response] }
    }

    fn order_out(&self) {
        unsafe { msg_send![self, orderOut: nil] }
    }
}

object_struct!(NSWindow);

impl INSWindow for NSWindow {}
