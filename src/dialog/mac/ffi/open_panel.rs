use std::path::PathBuf;

use objc2::rc::Retained as Id;
use objc2::MainThreadOnly;
use objc2_app_kit::{NSModalResponseOK, NSOpenPanel, NSWindow};
use objc2_foundation::MainThreadMarker;

use crate::dialog::UnsafeWindowHandle;

use super::{NSSavePanelExt, NSURLExt, NSWindowExt};

pub trait NSOpenPanelExt {
    fn open_panel(mtm: MainThreadMarker) -> Id<NSOpenPanel>;

    fn show(&self, owner: Option<UnsafeWindowHandle>) -> Vec<PathBuf>;

    fn set_can_choose_files(&self, flag: bool);
    fn set_can_choose_directories(&self, flag: bool);
    fn set_allows_multiple_selection(&self, flag: bool);
}

impl NSOpenPanelExt for NSOpenPanel {
    fn open_panel(mtm: MainThreadMarker) -> Id<Self> {
        unsafe { NSOpenPanel::openPanel(mtm) }
    }

    fn show(&self, owner: Option<UnsafeWindowHandle>) -> Vec<PathBuf> {
        let owner = NSWindow::from_handle(self.mtm(), owner);
        let response = self.run(owner.as_deref());

        (response == NSModalResponseOK)
            .then(|| unsafe { self.URLs() })
            .map(|urls| urls.into_iter().map(|x| x.to_path_buf()).collect())
            .unwrap_or_default()
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
}
