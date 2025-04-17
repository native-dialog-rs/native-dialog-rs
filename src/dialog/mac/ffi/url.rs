use objc2::rc::Retained as Id;
use objc2_foundation::{NSString, NSURL};
use std::path::{Path, PathBuf};

pub trait NSURLExt {
    fn from_path(s: &Path) -> Id<Self>;
    fn to_path_buf(&self) -> PathBuf;
}

impl NSURLExt for NSURL {
    fn from_path(s: &Path) -> Id<Self> {
        let s = NSString::from_str(&s.to_string_lossy());
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
