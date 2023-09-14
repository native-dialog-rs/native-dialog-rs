use objc_foundation::{INSObject, INSString, NSString};
use objc_id::Id;
use std::path::PathBuf;

pub trait INSUrl: INSObject {
    fn from_str(s: &str) -> Id<Self> {
        unsafe {
            let s = NSString::from_str(s);
            let ptr = msg_send![class!(NSURL), URLWithString: s];
            Id::from_ptr(ptr)
        }
    }

    fn from_path(s: &str) -> Id<Self> {
        unsafe {
            let s = NSString::from_str(s);
            let ptr = msg_send![class!(NSURL), fileURLWithPath: s];
            Id::from_ptr(ptr)
        }
    }

    fn absolute_url(&self) -> Id<Self> {
        unsafe {
            let s = msg_send![self, absoluteURL];
            Id::from_ptr(s)
        }
    }

    fn path(&self) -> Id<NSString> {
        unsafe {
            let s = msg_send![self, path];
            Id::from_ptr(s)
        }
    }

    fn to_path_buf(&self) -> PathBuf {
        self.absolute_url().path().as_str().into()
    }
}

object_struct!(NSUrl);

impl INSUrl for NSUrl {}
