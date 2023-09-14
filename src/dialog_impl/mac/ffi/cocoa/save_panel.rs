use super::{INSPanel, INSUrl, INSView, NSUrl, NSWindow};
use cocoa::foundation::NSInteger;
use objc_foundation::{INSMutableArray, INSObject, INSString, NSMutableArray, NSString};
use objc_id::Id;

pub trait INSSavePanel: INSPanel {
    fn save_panel() -> Id<Self> {
        unsafe {
            let ptr = msg_send![class!(NSSavePanel), savePanel];
            Id::from_ptr(ptr)
        }
    }

    fn set_name_field_string_value(&self, value: &str) {
        let value = NSString::from_str(value);
        unsafe { msg_send![self, setNameFieldStringValue: value] }
    }

    fn set_shows_tag_field(&self, flag: bool) {
        let flag = super::objc_bool(flag);
        unsafe { msg_send![self, setShowsTagField: flag] }
    }

    fn set_can_create_directories(&self, flag: bool) {
        let flag = super::objc_bool(flag);
        unsafe { msg_send![self, setCanCreateDirectories: flag] }
    }

    fn set_directory_url(&self, url: &str) {
        let url = NSUrl::from_path(url);
        unsafe { msg_send![self, setDirectoryURL: url] }
    }

    fn set_allowed_file_types(&self, types: &[&str]) {
        // We cannot use NSArray::from_vec or NSArray::from_slice because we need owned data.
        // Otherwise, a segfault will occur.
        let mut file_types = NSMutableArray::new();
        for ext in types {
            file_types.add_object(NSString::from_str(ext))
        }

        unsafe { msg_send![self, setAllowedFileTypes: file_types] }
    }

    fn set_extension_hidden(&self, flag: bool) {
        let flag = super::objc_bool(flag);
        unsafe { msg_send![self, setExtensionHidden: flag] }
    }

    fn set_accessory_view(&self, view: Id<impl INSView>) {
        unsafe { msg_send![self, setAccessoryView: view] }
    }

    fn set_accessory_view_disclosed(&self, flag: bool) {
        let flag = super::objc_bool(flag);
        unsafe { msg_send![self, setAccessoryViewDisclosed: flag] }
    }

    fn run_modal(&self, owner: Option<Id<NSWindow>>) -> Result<Id<NSUrl>, NSInteger> {
        match self.run_sheet_or_modal(owner) {
            1 => unsafe {
                let urls = msg_send![self, URL];
                Ok(Id::from_ptr(urls))
            },
            x => Err(x),
        }
    }
}

object_struct!(NSSavePanel);

impl INSPanel for NSSavePanel {}

impl INSSavePanel for NSSavePanel {}
