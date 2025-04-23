use std::path::PathBuf;

use objc2::rc::Retained as Id;
use objc2::runtime::ProtocolObject;
use objc2_app_kit::{NSModalResponseOK, NSOpenPanel};
use objc2_foundation::MainThreadMarker;

use super::{NSSavePanelExt, NSURLExt, OpenPanelDelegate};
use crate::utils::UnsafeWindowHandle;

pub trait NSOpenPanelExt {
    fn open_panel(mtm: MainThreadMarker) -> Id<NSOpenPanel>;

    fn show(&self, owner: UnsafeWindowHandle) -> Vec<PathBuf>;

    fn set_delegate(&self, delegate: &OpenPanelDelegate);
    fn set_can_choose_files(&self, flag: bool);
    fn set_can_choose_directories(&self, flag: bool);
    fn set_allows_multiple_selection(&self, flag: bool);
}

impl NSOpenPanelExt for NSOpenPanel {
    fn open_panel(mtm: MainThreadMarker) -> Id<Self> {
        unsafe { NSOpenPanel::openPanel(mtm) }
    }

    fn show(&self, owner: UnsafeWindowHandle) -> Vec<PathBuf> {
        let owner = unsafe { owner.as_appkit() };
        let response = self.run(owner.as_deref());

        (response == NSModalResponseOK)
            .then(|| unsafe { self.URLs() })
            .map(|urls| urls.into_iter().filter_map(|x| x.to_path_buf()).collect())
            .unwrap_or_default()
    }

    fn set_delegate(&self, delegate: &OpenPanelDelegate) {
        unsafe { self.setDelegate(Some(ProtocolObject::from_ref(delegate))) };
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
