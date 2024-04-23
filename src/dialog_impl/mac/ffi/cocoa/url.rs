use objc2::rc::Id;
use objc2_foundation::{NSString, NSURL};
use std::path::PathBuf;

pub trait INSURL {
    fn from_str(s: &str) -> Id<Self>;

    fn from_path(s: &str) -> Id<Self>;

    fn to_path_buf(&self) -> PathBuf;
}

impl INSURL for NSURL {
    fn from_str(s: &str) -> Id<Self> {
        let s = NSString::from_str(s);
        unsafe { NSURL::URLWithString(&s).unwrap() }
    }

    fn from_path(s: &str) -> Id<Self> {
        let s = NSString::from_str(s);
        unsafe { NSURL::fileURLWithPath(&s) }
    }

    fn to_path_buf(&self) -> PathBuf {
        unsafe {
            self.absoluteURL()
                .unwrap()
                .path()
                .unwrap()
                .to_string()
                .into()
        }
    }
}
