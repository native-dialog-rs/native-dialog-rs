use std::cell::Cell;
use std::ptr;

use crate::Filter;
use objc2::mutability::InteriorMutable;
use objc2::rc::Id;
use objc2::runtime::AnyObject;
use objc2::{declare_class, msg_send, msg_send_id, ClassType, DeclaredClass};
use objc2_app_kit::NSSavePanel;
use objc2_foundation::{NSInteger, NSObject};

use super::cocoa::INSSavePanel;

#[derive(Debug)]
pub struct Ivars {
    panel: Id<NSSavePanel>,
    filters: Cell<*const Vec<Filter<'static>>>,
}

declare_class!(
    #[derive(Debug)]
    pub struct DropdownAction;

    unsafe impl ClassType for DropdownAction {
        type Super = NSObject;
        type Mutability = InteriorMutable;
        const NAME: &'static str = "__RustNativeDialogSavePanelDropdownAction";
    }

    impl DeclaredClass for DropdownAction {
        type Ivars = Ivars;
    }

    unsafe impl DropdownAction {
        #[method(onItemSelected:)]
        fn on_item_selected(&self, sender: &AnyObject) {
            let filters = self.ivars().filters.get();
            if let Some(filters) = unsafe { filters.as_ref() } {
                let index: NSInteger = unsafe { msg_send![sender, indexOfSelectedItem] };
                let filter = filters.get(index as usize).unwrap();

                self.ivars().panel.set_allowed_extensions(filter.extensions);
            }
        }
    }
);

impl DropdownAction {
    pub fn new_with_save_panel(panel: &NSSavePanel) -> Id<Self> {
        let this = Self::alloc().set_ivars(Ivars {
            panel: panel.retain(),
            filters: Cell::new(ptr::null()),
        });
        unsafe { msg_send_id![super(this), init] }
    }

    /// The caller has the responsibility to reset the pointer before the pointed value is dropped.
    pub unsafe fn set_filters(&self, filters: *const Vec<Filter<'_>>) {
        let filters: *const Vec<Filter<'static>> = filters.cast();
        self.ivars().filters.set(filters);
    }
}
