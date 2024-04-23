use objc2::{msg_send, rc::Id};
use objc2_app_kit::{
    NSAppKitVersionNumber, NSAppKitVersionNumber11_0, NSApplication, NSApplicationActivationPolicy,
    NSModalResponse, NSSavePanel, NSView, NSWindow,
};
use objc2_foundation::{MainThreadMarker, NSArray, NSMutableArray, NSString, NSURL};
use objc2_uniform_type_identifiers::UTType;

use super::{INSWindow, INSURL};

pub trait INSSavePanel {
    fn save_panel() -> Id<Self>;

    fn run_sheet_or_modal(&self, owner: Option<Id<NSWindow>>) -> NSModalResponse;

    fn run_modal(&self, owner: Option<Id<NSWindow>>) -> Result<Id<NSURL>, NSModalResponse>;

    fn set_title(&self, title: &str);

    fn set_name_field_string_value(&self, value: &str);

    fn set_shows_tag_field(&self, flag: bool);

    fn set_can_create_directories(&self, flag: bool);

    fn set_directory_url(&self, url: &str);

    fn set_extension_hidden(&self, flag: bool);

    fn set_accessory_view(&self, view: &NSView);

    fn set_allowed_content_types(&self, value: &NSArray<UTType>);

    fn set_allowed_extensions(&self, extensions: &[&str]);
}

impl INSSavePanel for NSSavePanel {
    fn save_panel() -> Id<Self> {
        // TODO: Main Thread Safety
        let mtm = unsafe { MainThreadMarker::new_unchecked() };
        unsafe { NSSavePanel::savePanel(mtm) }
    }

    fn run_sheet_or_modal(&self, owner: Option<Id<NSWindow>>) -> NSModalResponse {
        match owner {
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

                if policy == NSApplicationActivationPolicy::Prohibited {
                    app.setActivationPolicy(NSApplicationActivationPolicy::Accessory);
                    let ret = unsafe { self.runModal() };
                    app.setActivationPolicy(NSApplicationActivationPolicy::Prohibited);
                    ret
                } else {
                    unsafe { self.runModal() }
                }
            }
        }
    }

    fn run_modal(&self, owner: Option<Id<NSWindow>>) -> Result<Id<NSURL>, NSModalResponse> {
        match self.run_sheet_or_modal(owner) {
            1 => unsafe { Ok(self.URL().unwrap()) },
            x => Err(x),
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

    fn set_accessory_view(&self, view: &NSView) {
        unsafe { self.setAccessoryView(Some(view)) }
    }

    fn set_allowed_content_types(&self, value: &NSArray<UTType>) {
        unsafe { msg_send![self, setAllowedContentTypes: value] }
    }

    fn set_allowed_extensions(&self, extensions: &[&str]) {
        if unsafe { NSAppKitVersionNumber > NSAppKitVersionNumber11_0 } {
            let mut types = NSMutableArray::new();
            for ext in extensions {
                let t =
                    unsafe { UTType::typeWithFilenameExtension(&NSString::from_str(ext)) }.unwrap();
                types.push(t);
            }
            // Available from macOS 11
            self.set_allowed_content_types(&types)
        } else {
            let mut types = NSMutableArray::new();
            for ext in extensions {
                let t = NSString::from_str(ext);
                types.push(t);
            }
            // Removed at macOS 13
            #[allow(deprecated)]
            unsafe {
                self.setAllowedFileTypes(Some(&types))
            }
        }
    }
}
