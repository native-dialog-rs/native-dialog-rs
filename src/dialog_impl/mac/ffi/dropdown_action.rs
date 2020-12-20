use super::cocoa::{INSSavePanel, NSSavePanel};
use crate::Filter;
use cocoa::base::id;
use cocoa::foundation::NSInteger;
use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Sel};
use objc::Message;
use objc_foundation::INSObject;
use objc_id::Id;
use once_cell::sync::OnceCell;

static CLASS: OnceCell<&Class> = OnceCell::new();

fn declare_class() -> &'static Class {
    let classname = "__RustNativeDialogSavePanelDropdownAction";
    let mut decl = ClassDecl::new(classname, class!(NSObject)).unwrap();

    decl.add_ivar::<id>("_panel");
    decl.add_ivar::<usize>("_filters");

    extern "C" fn set_save_panel(this: &mut Object, _sel: Sel, panel: id) {
        unsafe { this.set_ivar("_panel", panel) };
    }

    extern "C" fn set_filters(this: &mut Object, _sel: Sel, filters: usize) {
        unsafe { this.set_ivar("_filters", filters) };
    }

    extern "C" fn on_item_selected(this: &Object, _sel: Sel, sender: id) {
        unsafe {
            let ptr_filters = *this.get_ivar::<usize>("_filters") as *const Vec<Filter>;
            if ptr_filters.is_null() {
                return;
            }

            let index: NSInteger = msg_send![sender, indexOfSelectedItem];
            let filter = (*ptr_filters).get(index as usize).unwrap();

            let panel = *this.get_ivar::<id>("_panel") as *mut NSSavePanel;
            let panel: Id<NSSavePanel> = Id::from_ptr(panel);
            panel.set_allowed_file_types(filter.extensions);
        }
    }

    unsafe {
        decl.add_method(
            sel!(setSavePanel:),
            set_save_panel as extern "C" fn(&mut Object, Sel, id),
        );
        decl.add_method(
            sel!(setFilters:),
            set_filters as extern "C" fn(&mut Object, Sel, usize),
        );
        decl.add_method(
            sel!(onItemSelected:),
            on_item_selected as extern "C" fn(&Object, Sel, id),
        );
    }

    decl.register()
}

pub trait IDropdownAction: INSObject {
    fn set_save_panel<T>(&self, panel: Id<NSSavePanel, T>) {
        unsafe { msg_send![self, setSavePanel: panel] }
    }

    /// The caller has the responsibility to reset the pointer before the pointed value is dropped.
    unsafe fn set_filters(&self, filters: *const Vec<Filter>) {
        msg_send![self, setFilters: filters as usize]
    }
}

pub struct DropdownAction {
    _private: (),
}

unsafe impl Message for DropdownAction {}

impl INSObject for DropdownAction {
    fn class() -> &'static Class {
        CLASS.get_or_init(declare_class)
    }
}

impl IDropdownAction for DropdownAction {}
