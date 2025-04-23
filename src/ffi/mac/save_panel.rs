use std::path::{Path, PathBuf};

use objc2::rc::Retained as Id;
use objc2::MainThreadOnly;
use objc2_app_kit::{
    NSAppKitVersionNumber, NSAppKitVersionNumber11_0, NSApplication, NSApplicationActivationPolicy,
    NSModalResponse, NSModalResponseOK, NSSavePanel, NSView, NSWindow,
};
use objc2_foundation::{MainThreadMarker, NSString, NSURL};
use objc2_uniform_type_identifiers::UTType;

use super::{NSURLExt, NSWindowExt};
use crate::dialog::Filter;
use crate::utils::UnsafeWindowHandle;

pub trait NSSavePanelExt {
    fn save_panel(mtm: MainThreadMarker) -> Id<Self>;

    fn show(&self, owner: UnsafeWindowHandle) -> Option<PathBuf>;
    fn run(&self, owner: Option<&NSWindow>) -> NSModalResponse;

    fn set_title(&self, title: &str);
    fn set_name_field_string_value(&self, value: &str);
    fn set_can_create_directories(&self, flag: bool);
    fn set_directory_url(&self, url: &Path);
    fn set_extension_hidden(&self, flag: bool);
    fn set_accessory_view(&self, view: Option<&NSView>);
    fn set_filters(&self, filters: &[Filter]);
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
            .map(|url| url.to_path_buf())
    }

    fn run(&self, owner: Option<&NSWindow>) -> NSModalResponse {
        match owner {
            Some(window) => {
                window.begin_sheet(self);
                let response = unsafe { self.runModal() };
                window.end_sheet(self, response);

                response
            }
            None => {
                let app = NSApplication::sharedApplication(self.mtm());
                let policy = unsafe { app.activationPolicy() };

                app.setActivationPolicy(NSApplicationActivationPolicy::Accessory);
                let response = unsafe { self.runModal() };
                app.setActivationPolicy(policy);

                response
            }
        }
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

    fn set_filters(&self, filters: &[Filter]) {
        let extensions = filters.iter().flat_map(|x| &x.extensions);

        if unsafe { NSAppKitVersionNumber > NSAppKitVersionNumber11_0 } {
            let types = extensions
                .map(|x| NSString::from_str(x))
                // TODO: will panic here if extension contains period
                .map(|x| unsafe { UTType::typeWithFilenameExtension(&x) }.unwrap())
                .collect::<Id<_>>();

            // Available from macOS 11
            unsafe { self.setAllowedContentTypes(&types) }
        } else {
            let types = extensions.map(|x| NSString::from_str(x)).collect::<Id<_>>();

            // Removed at macOS 13
            unsafe {
                #[allow(deprecated)]
                self.setAllowedFileTypes(Some(&types))
            }
        }
    }
}
