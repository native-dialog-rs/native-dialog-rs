use cocoa::base::id;
use objc_foundation::{INSObject, INSString, NSString};
use objc_id::Id;

pub trait INSImage: INSObject {
    fn named(name: &str) -> Id<Self> {
        unsafe {
            let name = NSString::from_str(name);
            let ptr = msg_send![class!(NSImage), imageNamed: name];
            Id::from_ptr(ptr)
        }
    }

    fn new_with_file(path: &str) -> Id<Self> {
        unsafe {
            let path = NSString::from_str(path);
            let ptr: id = msg_send![class!(NSImage), alloc];
            let ptr = msg_send![ptr, initWithContentsOfFile: path];
            Id::from_retained_ptr(ptr)
        }
    }
}

object_struct!(NSImage);

impl INSImage for NSImage {}
