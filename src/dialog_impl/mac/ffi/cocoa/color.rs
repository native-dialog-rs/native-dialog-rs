use objc_foundation::INSObject;
use objc_id::Id;

pub trait INSColor: INSObject {
    fn secondary_label_color() -> Id<Self> {
        unsafe {
            let ptr = msg_send![class!(NSColor), secondaryLabelColor];
            Id::from_ptr(ptr)
        }
    }
}

object_struct!(NSColor);

impl INSColor for NSColor {}
