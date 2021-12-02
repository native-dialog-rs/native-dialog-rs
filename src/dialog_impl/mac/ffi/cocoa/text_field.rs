use super::{INSColor, INSView};
use objc_foundation::{INSString, NSString};
use objc_id::Id;

pub trait INSTextField: INSView {
    fn label_with_string(string: &str) -> Id<Self> {
        let string = NSString::from_str(string);
        unsafe {
            let ptr = msg_send![class!(NSTextField), labelWithString: string];
            Id::from_ptr(ptr)
        }
    }

    fn set_text_color(&self, color: Id<impl INSColor>) {
        unsafe { msg_send![self, setTextColor: color] }
    }
}

object_struct!(NSTextField);

impl INSView for NSTextField {}
impl INSTextField for NSTextField {}
