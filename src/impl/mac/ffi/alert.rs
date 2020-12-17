use super::INSImage;
use cocoa::foundation::NSInteger;
use objc_foundation::{INSObject, INSString, NSString};
use objc_id::Id;

pub trait INSAlert: INSObject {
    fn alert() -> Id<Self> {
        unsafe {
            let ptr = msg_send![class!(NSAlert), new];
            Id::from_retained_ptr(ptr)
        }
    }

    fn set_informative_text(&self, text: &str) {
        let text = NSString::from_str(text);
        unsafe { msg_send![self, setInformativeText: text] }
    }

    fn set_message_text(&self, text: &str) {
        let text = NSString::from_str(text);
        unsafe { msg_send![self, setMessageText: text] }
    }

    fn set_icon(&self, icon: Id<impl INSImage>) {
        unsafe { msg_send![self, setIcon: icon] }
    }

    fn add_button(&self, title: &str) {
        let title = NSString::from_str(title);
        unsafe { msg_send![self, addButtonWithTitle: title] }
    }

    fn run_modal(&self) -> NSInteger {
        unsafe { super::with_activation(|| msg_send![self, runModal]) }
    }
}

object_struct!(NSAlert);

impl INSAlert for NSAlert {}
