use super::{INSWindow, NSWindow};
use cocoa::foundation::NSInteger;
use objc_foundation::INSObject;
use objc_id::Id;

pub trait INSPanel: INSObject {
    fn run_sheet_or_modal(&self, owner: Option<Id<NSWindow>>) -> NSInteger {
        match owner {
            Some(window) => {
                window.begin_sheet(self);
                let response = unsafe { msg_send![self, runModal] };
                window.end_sheet(self, response);
                response
            }
            None => unsafe { super::with_activation(|| msg_send![self, runModal]) },
        }
    }
}

object_struct!(NSPanel);

impl INSPanel for NSPanel {}
