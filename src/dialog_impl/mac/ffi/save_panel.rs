use objc2::rc::Retained as Id;
use objc2_app_kit::{
    NSAppKitVersionNumber, NSAppKitVersionNumber11_0, NSApplication, NSApplicationActivationPolicy,
    NSModalResponse, NSSavePanel, NSView, NSWindow,
};
use objc2_foundation::{MainThreadMarker, NSString, NSURL};
use objc2_uniform_type_identifiers::UTType;
use raw_window_handle::RawWindowHandle;

use super::{NSURLExt, NSWindowExt};
use crate::Filter;

pub trait NSSavePanelExt {
    fn save_panel() -> Id<Self>;

    #[cfg(feature = "async")]
    async fn run_completion(&self, owner: Option<RawWindowHandle>) -> NSModalResponse;
    fn run_blocking(&self, owner: Option<RawWindowHandle>) -> NSModalResponse;

    #[cfg(feature = "async")]
    async fn spawn(&self, owner: Option<RawWindowHandle>) -> Option<Id<NSURL>>;
    fn show(&self, owner: Option<RawWindowHandle>) -> Option<Id<NSURL>>;

    fn set_title(&self, title: &str);
    fn set_name_field_string_value(&self, value: &str);
    fn set_shows_tag_field(&self, flag: bool);
    fn set_can_create_directories(&self, flag: bool);
    fn set_directory_url(&self, url: &str);
    fn set_extension_hidden(&self, flag: bool);
    fn set_accessory_view(&self, view: Option<&NSView>);
    fn set_filters(&self, filters: &[Filter]);
}

impl NSSavePanelExt for NSSavePanel {
    fn save_panel() -> Id<Self> {
        // TODO: Main Thread Safety
        let mtm = unsafe { MainThreadMarker::new_unchecked() };
        MainThreadMarker::new().expect("main thread");
        unsafe { NSSavePanel::savePanel(mtm) }
    }

    #[cfg(feature = "async")]
    async fn spawn(&self, owner: Option<RawWindowHandle>) -> Option<Id<NSURL>> {
        match self.run_completion(owner).await {
            1 => unsafe { Some(self.URL().unwrap()) },
            _ => None,
        }
    }

    fn show(&self, owner: Option<RawWindowHandle>) -> Option<Id<NSURL>> {
        match self.run_blocking(owner) {
            1 => unsafe { Some(self.URL().unwrap()) },
            _ => None,
        }
    }

    #[cfg(feature = "async")]
    async fn run_completion(&self, owner: Option<RawWindowHandle>) -> NSModalResponse {
        use block2::RcBlock;
        use objc2_app_kit::NSModalResponseAbort;
        use std::cell::Cell;

        let (send, recv) = futures_channel::oneshot::channel();

        let cell = Cell::new(Some(send));
        let handler = RcBlock::new(move |response: NSModalResponse| {
            if let Some(send) = cell.take() {
                let _ = send.send(response);
            }
        });

        match owner.and_then(NSWindow::from_raw_handle) {
            Some(window) => unsafe {
                self.beginSheetModalForWindow_completionHandler(&window, &handler);
            },
            None => unsafe {
                self.beginWithCompletionHandler(&handler);
            },
        }

        recv.await.unwrap_or(NSModalResponseAbort)
    }

    fn run_blocking(&self, owner: Option<RawWindowHandle>) -> NSModalResponse {
        match owner.and_then(NSWindow::from_raw_handle) {
            Some(window) => {
                window.begin_sheet(self);
                let response = unsafe { self.runModal() };
                window.end_sheet(self, response);

                response
            }
            None => {
                // TODO: Main Thread Safety
                let mtm = unsafe { MainThreadMarker::new_unchecked() };
                let app = NSApplication::sharedApplication(mtm);
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

    fn set_shows_tag_field(&self, flag: bool) {
        unsafe { self.setShowsTagField(flag) }
    }

    fn set_can_create_directories(&self, flag: bool) {
        unsafe { self.setCanCreateDirectories(flag) }
    }

    fn set_directory_url(&self, url: &str) {
        let url = NSURL::from_path(url);
        unsafe { self.setDirectoryURL(Some(&url)) }
    }

    fn set_extension_hidden(&self, flag: bool) {
        unsafe { self.setExtensionHidden(flag) }
    }

    fn set_accessory_view(&self, view: Option<&NSView>) {
        unsafe { self.setAccessoryView(view) }
    }

    fn set_filters(&self, filters: &[Filter]) {
        let extensions = filters.iter().flat_map(|x| x.extensions);

        if unsafe { NSAppKitVersionNumber > NSAppKitVersionNumber11_0 } {
            let types = extensions
                .map(|x| NSString::from_str(x))
                .map(|x| unsafe { UTType::typeWithFilenameExtension(&x) }.unwrap())
                .collect::<Id<_>>();

            // Available from macOS 11
            unsafe { self.setAllowedContentTypes(&types) }
        } else {
            let types = extensions.map(|x| NSString::from_str(x)).collect::<Id<_>>();

            // Removed at macOS 13
            #[allow(deprecated)]
            unsafe {
                self.setAllowedFileTypes(Some(&types))
            }
        }
    }
}
