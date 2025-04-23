use std::cell::Cell;

use objc2::rc::Retained as Id;
use objc2::{define_class, msg_send, sel, DefinedClass, MainThreadOnly, Message};
use objc2_app_kit::{
    NSColor, NSLayoutConstraintOrientation, NSPopUpButton, NSSavePanel, NSStackView,
    NSStackViewGravity, NSTextField, NSView,
};
use objc2_foundation::{NSArray, NSEdgeInsets, NSObject, NSPoint, NSRect, NSSize, NSString};

use super::{NSColorExt, NSPopUpButtonExt, NSSavePanelExt, NSStackViewExt, NSTextFieldExt};
use crate::dialog::Filter;

pub struct SavePanelDelegateIvars {
    panel: Id<NSSavePanel>,
    accessory: Cell<Option<Id<NSView>>>,
    filters: Vec<Filter>,
}

define_class! {
    #[unsafe(super = NSObject)]
    #[thread_kind = MainThreadOnly]
    #[ivars = SavePanelDelegateIvars]
    pub struct SavePanelDelegate;

    impl SavePanelDelegate {
        #[unsafe(method(onItemSelected:))]
        fn on_item_selected(&self, sender: &NSPopUpButton) {
            let ivars = self.ivars();
            let index = unsafe { sender.indexOfSelectedItem() };
            if let Some(filter) = ivars.filters.get(index as usize) {
                ivars.panel.set_filters(&[filter.clone()]);
            }
        }
    }
}

impl SavePanelDelegate {
    pub fn attach(panel: &NSSavePanel, filters: &[Filter]) -> Id<Self> {
        let ivars = SavePanelDelegateIvars {
            panel: panel.retain(),
            accessory: Cell::new(None),
            filters: filters.to_owned(),
        };

        let this = Self::alloc(panel.mtm()).set_ivars(ivars);
        let this: Id<Self> = unsafe { msg_send![super(this), init] };

        // If there are filters specified, show a dropdown on the panel
        if let Some(first) = filters.first() {
            let accessory = this.create_accessory(filters);
            panel.set_accessory_view(Some(&accessory));
            panel.set_filters(&[first.clone()]);
            this.ivars().accessory.set(Some(accessory));
        }

        this
    }

    fn create_accessory(&self, filters: &[Filter]) -> Id<NSView> {
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

    fn format_titles(&self, filters: &[Filter]) -> Id<NSArray<NSString>> {
        filters
            .iter()
            .map(|filter| filter.format("{desc} ({types})", "*.{ext}", " "))
            .map(|title| NSString::from_str(&title))
            .collect()
    }
}

#[cfg(feature = "async")]
impl super::AsyncDelegate for SavePanelDelegate {}
