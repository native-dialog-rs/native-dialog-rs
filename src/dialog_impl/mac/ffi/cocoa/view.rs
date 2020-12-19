use objc_foundation::INSObject;
use objc_id::Id;

pub trait INSView: INSObject {
    fn add_subview(&self, view: Id<impl INSView>) {
        unsafe { msg_send![self, addSubview: view] }
    }
}

object_struct!(NSView);

impl INSView for NSView {}
