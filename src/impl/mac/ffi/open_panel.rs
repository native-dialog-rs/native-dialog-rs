use super::{INSURL, NSURL};
use cocoa::foundation::NSInteger;
use objc::runtime::{NO, YES};
use objc_foundation::{INSArray, INSObject, NSArray};
use objc_id::Id;

pub trait INSOpenPanel: INSObject {
    fn open_panel() -> Id<Self> {
        unsafe {
            let ptr = msg_send![class!(NSOpenPanel), openPanel];
            Id::from_retained_ptr(ptr)
        }
    }

    fn set_can_choose_files(&self, value: bool) {
        let value = if value { YES } else { NO };
        unsafe { msg_send![self, setCanChooseFiles: value] }
    }

    fn set_can_choose_directories(&self, value: bool) {
        let value = if value { YES } else { NO };
        unsafe { msg_send![self, setCanChooseDirectories: value] }
    }

    fn set_allows_multiple_selection(&self, value: bool) {
        let value = if value { YES } else { NO };
        unsafe { msg_send![self, setAllowsMultipleSelection: value] }
    }

    fn set_directory_url(&self, url: &str) {
        let url = NSURL::from_str(url);
        unsafe { msg_send![self, setDirectoryURL: url] }
    }

    fn set_allowed_file_types(&self, types: Id<impl INSArray>) {
        unsafe { msg_send![self, setAllowedFileTypes: types] }
    }

    fn run_modal(&self) -> Result<Id<NSArray<NSURL>>, NSInteger> {
        let response: NSInteger = unsafe { super::with_activation(|| msg_send![self, runModal]) };
        match response {
            1 => unsafe {
                let urls = msg_send![self, URLs];
                Ok(Id::from_retained_ptr(urls))
            },
            x => Err(x),
        }
    }
}

object_struct!(NSOpenPanel);

impl INSOpenPanel for NSOpenPanel {}
