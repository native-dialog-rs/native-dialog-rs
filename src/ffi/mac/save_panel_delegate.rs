use std::cell::Cell;

use objc2::rc::Retained as Id;
use objc2::{define_class, msg_send, sel, DefinedClass, MainThreadOnly, Message};
use objc2_app_kit::{
    NSAlert, NSColor, NSImage, NSImageNameCaution, NSLayoutConstraintOrientation,
    NSOpenSavePanelDelegate, NSPopUpButton, NSSavePanel, NSStackView, NSStackViewGravity,
    NSTextField, NSView,
};
use objc2_foundation::{
    NSArray, NSEdgeInsets, NSObject, NSObjectProtocol, NSPoint, NSRect, NSSize, NSString,
};

use super::{NSColorExt, NSPopUpButtonExt, NSSavePanelExt, NSStackViewExt, NSTextFieldExt};
use crate::dialog::{FileFilter, FileFiltersBag};

pub struct SavePanelDelegateIvars {
    panel: Id<NSSavePanel>,
    accessory: Cell<Option<Id<NSView>>>,
    filters: FileFiltersBag,
    selected: Cell<usize>,
}

define_class! {
    #[unsafe(super = NSObject)]
    #[thread_kind = MainThreadOnly]
    #[ivars = SavePanelDelegateIvars]
    pub struct SavePanelDelegate;

    unsafe impl NSObjectProtocol for SavePanelDelegate {}

    unsafe impl NSOpenSavePanelDelegate for SavePanelDelegate {
        #[unsafe(method(panel:userEnteredFilename:confirmed:))]
        unsafe fn check_type(&self, sender: &NSSavePanel, filename: &NSString, _: bool) -> *mut NSString {
            let Some(filter) = self.selected_filter() else {
                return Id::into_raw(filename.retain());
            };

            if filter.accepts(filename.to_string()) {
                return Id::into_raw(filename.retain());
            }

            let explain = format!("File \"{}\" is not of type {}.", filename, filter.description);

            let alert = NSAlert::new(self.mtm());
            alert.setMessageText(&NSString::from_str("Unrecognized File Type"));
            alert.setInformativeText(&NSString::from_str(&explain));
            alert.setIcon(NSImage::imageNamed(NSImageNameCaution).as_deref());
            alert.beginSheetModalForWindow_completionHandler(sender, None);

            std::ptr::null_mut()
        }
    }

    impl SavePanelDelegate {
        #[unsafe(method(onItemSelected:))]
        fn on_item_selected(&self, sender: &NSPopUpButton) {
            let index = unsafe { sender.indexOfSelectedItem() };
            self.ivars().selected.set(index as usize);
        }
    }
}

impl SavePanelDelegate {
    pub fn attach(panel: &NSSavePanel, filters: &FileFiltersBag) -> Id<Self> {
        let ivars = SavePanelDelegateIvars {
            panel: panel.retain(),
            accessory: Cell::new(None),
            filters: filters.to_owned(),
            selected: Cell::new(0),
        };

        let this = Self::alloc(panel.mtm()).set_ivars(ivars);
        let this: Id<Self> = unsafe { msg_send![super(this), init] };

        panel.set_delegate(&this);

        // If there are filters specified, show a dropdown on the panel
        if !filters.is_empty() {
            let accessory = this.create_accessory(filters);
            panel.set_accessory_view(Some(&accessory));
            this.ivars().accessory.set(Some(accessory));
        }

        this
    }

    fn selected_filter(&self) -> Option<&FileFilter> {
        let ivars = self.ivars();
        ivars.filters.get(ivars.selected.get())
    }

    fn create_accessory(&self, filters: &FileFiltersBag) -> Id<NSView> {
        let titles = self.format_titles(filters);

        let frame = NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(480.0, 0.0));
        let dropdown = NSPopUpButton::new_with_frame(self.mtm(), frame);
        dropdown.add_items_with_titles(&titles);
        dropdown.select_item_at(0);
        dropdown.set_action(sel!(onItemSelected:));
        dropdown.set_target(self);

        let label = NSTextField::label_with_string(self.mtm(), "File Type: ");
        label.set_text_color(&NSColor::secondary_label_color());

        let stack = NSStackView::new_empty(self.mtm());
        // Edge insets in specific axis are only enforced when hugging priority >= 500
        // See https://stackoverflow.com/questions/54533509/nsstackview-edgeinsets-gets-ignored
        stack.set_hugging_priority(500.0, NSLayoutConstraintOrientation::Vertical);
        stack.set_hugging_priority(500.0, NSLayoutConstraintOrientation::Horizontal);
        stack.set_edge_insets(NSEdgeInsets {
            top: 16.0,
            left: 20.0,
            bottom: 16.0,
            right: 20.0,
        });
        stack.add_view_in_gravity(&label, NSStackViewGravity::Center);
        stack.add_view_in_gravity(&dropdown, NSStackViewGravity::Center);

        stack.into_super()
    }

    fn format_titles(&self, filters: &FileFiltersBag) -> Id<NSArray<NSString>> {
        filters
            .iter()
            .map(|filter| filter.format("{desc} ({types})", "*{ext}", " "))
            .map(|title| NSString::from_str(&title))
            .collect()
    }
}

#[cfg(feature = "async")]
impl super::AsyncDelegate for SavePanelDelegate {}
