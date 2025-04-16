use objc2::rc::Retained as Id;
use objc2_app_kit::NSOpenPanel;
use objc2_foundation::{MainThreadMarker, NSArray, NSURL};
use raw_window_handle::RawWindowHandle;

use super::NSSavePanelExt;

pub trait NSOpenPanelExt {
    fn open_panel() -> Id<NSOpenPanel>;

    #[cfg(feature = "async")]
    async fn spawn(&self, owner: Option<RawWindowHandle>) -> Option<Id<NSArray<NSURL>>>;
    fn show(&self, owner: Option<RawWindowHandle>) -> Option<Id<NSArray<NSURL>>>;

    fn set_can_choose_files(&self, flag: bool);
    fn set_can_choose_directories(&self, flag: bool);
    fn set_allows_multiple_selection(&self, flag: bool);
    fn set_accessory_view_disclosed(&self, flag: bool);
}

impl NSOpenPanelExt for NSOpenPanel {
    fn open_panel() -> Id<Self> {
        // TODO: Main Thread Safety
        let mtm = unsafe { MainThreadMarker::new_unchecked() };
        unsafe { NSOpenPanel::openPanel(mtm) }
    }

    #[cfg(feature = "async")]
    async fn spawn(&self, owner: Option<RawWindowHandle>) -> Option<Id<NSArray<NSURL>>> {
        match self.run_completion(owner).await {
            1 => unsafe { Some(self.URLs()) },
            _ => None,
        }
    }

    fn show(&self, owner: Option<RawWindowHandle>) -> Option<Id<NSArray<NSURL>>> {
        match self.run_blocking(owner) {
            1 => unsafe { Some(self.URLs()) },
            _ => None,
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
