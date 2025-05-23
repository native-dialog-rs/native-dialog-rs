use std::path::{Path, PathBuf};

use objc2::rc::Retained as Id;
use objc2::runtime::ProtocolObject;
use objc2::MainThreadOnly;
use objc2_app_kit::{NSApp, NSModalResponse, NSModalResponseOK, NSSavePanel, NSView, NSWindow};
use objc2_foundation::{MainThreadMarker, NSString, NSURL};

use super::{NSApplicationExt, NSURLExt, SavePanelDelegate};
use crate::utils::UnsafeWindowHandle;

pub trait NSSavePanelExt {
    fn save_panel(mtm: MainThreadMarker) -> Id<Self>;

    fn show(&self, owner: UnsafeWindowHandle) -> Option<PathBuf>;
    fn run(&self, owner: Option<&NSWindow>) -> NSModalResponse;

    fn set_delegate(&self, delegate: &SavePanelDelegate);
    fn set_title(&self, title: &str);
    fn set_name_field_string_value(&self, value: &str);
    fn set_can_create_directories(&self, flag: bool);
    fn set_directory_url(&self, url: &Path);
    fn set_extension_hidden(&self, flag: bool);
    fn set_accessory_view(&self, view: Option<&NSView>);
}

impl NSSavePanelExt for NSSavePanel {
    fn save_panel(mtm: MainThreadMarker) -> Id<Self> {
        unsafe { NSSavePanel::savePanel(mtm) }
    }

    fn show(&self, owner: UnsafeWindowHandle) -> Option<PathBuf> {
        let owner = unsafe { owner.as_appkit() };
        let response = self.run(owner.as_deref());

        (response == NSModalResponseOK)
            .then(|| unsafe { self.URL() })
            .flatten()
            .and_then(|url| url.to_path_buf())
    }

    fn run(&self, owner: Option<&NSWindow>) -> NSModalResponse {
        let app = NSApp(self.mtm());
        match owner {
            Some(window) => app.run_sheet(window, self),
            None => app.run_modal(self),
        }
    }

    fn set_delegate(&self, delegate: &SavePanelDelegate) {
        unsafe { self.setDelegate(Some(ProtocolObject::from_ref(delegate))) };
    }

    fn set_title(&self, title: &str) {
        let title = NSString::from_str(title);
        unsafe { self.setTitle(Some(&title)) }
    }

    fn set_name_field_string_value(&self, value: &str) {
        let value = NSString::from_str(value);
        unsafe { self.setNameFieldStringValue(&value) }
    }

    fn set_can_create_directories(&self, flag: bool) {
        unsafe { self.setCanCreateDirectories(flag) }
    }

    fn set_directory_url(&self, url: &Path) {
        let url = NSURL::new_path(url);
        unsafe { self.setDirectoryURL(Some(&url)) }
    }

    fn set_extension_hidden(&self, flag: bool) {
        unsafe { self.setExtensionHidden(flag) }
    }

    fn set_accessory_view(&self, view: Option<&NSView>) {
        unsafe { self.setAccessoryView(view) }
    }
}
