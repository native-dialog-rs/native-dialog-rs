use super::{INSUrl, NSUrl};
use cocoa::foundation::NSInteger;
use objc_foundation::{INSArray, INSObject, INSString, NSArray, NSString};
use objc_id::Id;

pub trait INSOpenPanel: INSObject {
    fn open_panel() -> Id<Self> {
        unsafe {
            let ptr = msg_send![class!(NSOpenPanel), openPanel];
            Id::from_ptr(ptr)
        }
    }

    fn set_name_field_string_value(&self, value: &str) {
        let value = NSString::from_str(value);
        unsafe { msg_send![self, setNameFieldStringValue: value] }
    }

    fn set_can_choose_files(&self, flag: bool) {
        let flag = super::objc_bool(flag);
        unsafe { msg_send![self, setCanChooseFiles: flag] }
    }

    fn set_can_choose_directories(&self, flag: bool) {
        let flag = super::objc_bool(flag);
        unsafe { msg_send![self, setCanChooseDirectories: flag] }
    }

    fn set_allows_multiple_selection(&self, flag: bool) {
        let flag = super::objc_bool(flag);
        unsafe { msg_send![self, setAllowsMultipleSelection: flag] }
    }

    fn set_directory_url(&self, url: &str) {
        let url = NSUrl::from_str(url);
        unsafe { msg_send![self, setDirectoryURL: url] }
    }

    fn set_allowed_file_types(&self, types: Id<impl INSArray>) {
        unsafe { msg_send![self, setAllowedFileTypes: types] }
    }

    fn run_modal(&self) -> Result<Id<NSArray<NSUrl>>, NSInteger> {
        let response: NSInteger = unsafe { super::with_activation(|| msg_send![self, runModal]) };
        match response {
            1 => unsafe {
                let urls = msg_send![self, URLs];
                Ok(Id::from_ptr(urls))
            },
            x => Err(x),
        }
    }
}

object_struct!(NSOpenPanel);

impl INSOpenPanel for NSOpenPanel {}
