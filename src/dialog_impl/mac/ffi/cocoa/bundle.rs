use super::NSImage;
use objc_foundation::{INSObject, INSString, NSString};
use objc_id::Id;

pub trait INSBundle: INSObject {
    fn of_path(path: &str) -> Id<Self> {
        unsafe {
            let path = NSString::from_str(path);
            let ptr = msg_send![class!(NSBundle), bundleWithPath: path];
            Id::from_ptr(ptr)
        }
    }

    fn image_named(&self, name: &str) -> Id<NSImage> {
        unsafe {
            let name = NSString::from_str(name);
            let ptr = msg_send![self, imageForResource: name];
            Id::from_ptr(ptr)
        }
    }
}

object_struct!(NSBundle);

impl INSBundle for NSBundle {}
