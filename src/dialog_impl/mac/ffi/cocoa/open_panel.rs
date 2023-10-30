use super::{INSPanel, INSSavePanel, NSUrl, NSWindow};
use cocoa::foundation::NSInteger;
use objc_foundation::NSArray;
use objc_id::Id;

pub trait INSOpenPanel: INSPanel {
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
}

object_struct!(NSOpenPanel);

impl INSPanel for NSOpenPanel {}

impl INSOpenPanel for NSOpenPanel {}

impl INSSavePanel for NSOpenPanel {}

impl NSOpenPanel {
    pub fn open_panel() -> Id<Self> {
        unsafe {
            let ptr = msg_send![class!(NSOpenPanel), openPanel];
            Id::from_ptr(ptr)
        }
    }

    pub fn run_modal(&self, owner: Option<Id<NSWindow>>) -> Result<Id<NSArray<NSUrl>>, NSInteger> {
        match self.run_sheet_or_modal(owner) {
            1 => unsafe {
                let urls = msg_send![self, URLs];
                Ok(Id::from_ptr(urls))
            },
            x => Err(x),
        }
    }
}
