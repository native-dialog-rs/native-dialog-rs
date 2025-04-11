use objc2::rc::Id;
use objc2_app_kit::{NSModalResponse, NSOpenPanel, NSWindow};
use objc2_foundation::{MainThreadMarker, NSArray, NSURL};

use super::INSSavePanel;

pub trait INSOpenPanel {
    fn open_panel() -> Id<NSOpenPanel>;

    fn run_modal(&self, owner: Option<Id<NSWindow>>)
        -> Result<Id<NSArray<NSURL>>, NSModalResponse>;

    fn set_can_choose_files(&self, flag: bool);

    fn set_can_choose_directories(&self, flag: bool);

    fn set_allows_multiple_selection(&self, flag: bool);

    fn set_accessory_view_disclosed(&self, flag: bool);
}

impl INSOpenPanel for NSOpenPanel {
    fn open_panel() -> Id<Self> {
        // TODO: Main Thread Safety
        let mtm = unsafe { MainThreadMarker::new_unchecked() };
        unsafe { NSOpenPanel::openPanel(mtm) }
    }

    fn run_modal(
        &self,
        owner: Option<Id<NSWindow>>,
    ) -> Result<Id<NSArray<NSURL>>, NSModalResponse> {
        match self.run_sheet_or_modal(owner) {
            1 => unsafe { Ok(self.URLs()) },
            x => Err(x),
        }
    }

    fn set_can_choose_files(&self, flag: bool) {
        unsafe { self.setCanChooseFiles(flag) };
    }

    fn set_can_choose_directories(&self, flag: bool) {
        unsafe { self.setCanChooseDirectories(flag) };
    }

    fn set_allows_multiple_selection(&self, flag: bool) {
        unsafe { self.setAllowsMultipleSelection(flag) };
    }

    fn set_accessory_view_disclosed(&self, flag: bool) {
        unsafe { self.setAccessoryViewDisclosed(flag) }
    }
}
