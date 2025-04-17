use std::cell::Cell;
use std::marker::PhantomData;

use objc2::rc::Retained as Id;
use objc2::{define_class, msg_send, sel, ClassType, DefinedClass, MainThreadOnly, Message};
use objc2_app_kit::{
    NSColor, NSLayoutConstraintOrientation, NSPopUpButton, NSSavePanel, NSStackView,
    NSStackViewGravity, NSTextField, NSView,
};
use objc2_foundation::{NSArray, NSEdgeInsets, NSObject, NSPoint, NSRect, NSSize, NSString};

use super::{NSColorExt, NSPopUpButtonExt, NSSavePanelExt, NSStackViewExt, NSTextFieldExt};
use crate::dialog::Filter;

pub struct SavePanelFiltersIvars {
    panel: Id<NSSavePanel>,
    accessory: Cell<Option<Id<NSView>>>,
    filters: Cell<*const Vec<Filter>>,
}

define_class! {
    #[unsafe(super = NSObject)]
    #[thread_kind = MainThreadOnly]
    #[name = "__RustNativeDialogSavePanelFilters"]
    #[ivars = SavePanelFiltersIvars]
    pub struct SavePanelFilters;

    impl SavePanelFilters {
        #[unsafe(method(onItemSelected:))]
        fn on_item_selected(&self, sender: &NSPopUpButton) {
            let ivars = self.ivars();
            if let Some(filters) = unsafe { ivars.filters.get().as_ref() } {
                let index = unsafe { sender.indexOfSelectedItem() };
                if let Some(filter) = filters.get(index as usize) {
                    ivars.panel.set_filters(&[filter.clone()]);
                }
            }
        }
    }
}

impl SavePanelFilters {
    pub fn attach<'a>(panel: &NSSavePanel, filters: &'a Vec<Filter>) -> Guard<'a> {
        let ivars = SavePanelFiltersIvars {
            panel: panel.retain(),
            accessory: Cell::new(None),
            filters: Cell::new(filters as *const Vec<Filter> as *const _),
        };

        let target = Self::alloc(panel.mtm()).set_ivars(ivars);
        let target: Id<Self> = unsafe { msg_send![super(target), init] };

        // If there are filters specified, show a dropdown on the panel
        if let Some(first) = filters.first() {
            let accessory = target.create_accessory(filters);
            panel.set_accessory_view(Some(&accessory));
            panel.set_filters(&[first.clone()]);
            target.ivars().accessory.set(Some(accessory));
        }

        Guard {
            target,
            _marker: PhantomData,
        }
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

        stack.as_super().retain()
    }

    fn format_titles(&self, filters: &[Filter]) -> Id<NSArray<NSString>> {
        filters
            .iter()
            .map(|filter| filter.format("{desc} ({types})", "*.{ext}", " "))
            .map(|title| NSString::from_str(&title))
            .collect()
    }
}

pub struct Guard<'a> {
    target: Id<SavePanelFilters>,
    _marker: PhantomData<&'a Vec<Filter>>,
}

impl Drop for Guard<'_> {
    fn drop(&mut self) {
        self.target.ivars().filters.set(std::ptr::null());
    }
}
