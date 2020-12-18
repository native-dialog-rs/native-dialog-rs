use cocoa::base::nil;
use cocoa::foundation::{NSInteger, NSUInteger};
use objc::runtime::{Object, BOOL, NO, YES};
use objc_id::{Id, Shared};
use std::os::raw::c_char;

pub trait NSSavePanel {
    fn save_panel() -> Id<Self> {
        unsafe {
            let ptr = msg_send![class!(NSSavePanel), savePanel];
            Id::from_retained_ptr(ptr)
        }
    }

    fn run_modal(mut self) -> NSInteger {
        unsafe { msg_send![self.0, runModal] }
    }
}
