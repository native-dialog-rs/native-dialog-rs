use super::INSView;
use cocoa::base::id;
use cocoa::foundation::{NSInteger, NSRect};
use objc::runtime::Sel;
use objc_foundation::{INSArray, INSObject, NSString};
use objc_id::Id;

pub trait INSPopUpButton: INSView {
    fn new_with_frame(frame: NSRect, pulls_down: bool) -> Id<Self> {
        let pulls_down = super::objc_bool(pulls_down);
        unsafe {
            let ptr: id = msg_send![class!(NSPopUpButton), alloc];
            let ptr = msg_send![ptr, initWithFrame:frame pullsDown:pulls_down];
            Id::from_retained_ptr(ptr)
        }
    }

    fn add_items_with_titles(&self, items: Id<impl INSArray<Item = NSString>>) {
        unsafe { msg_send![self, addItemsWithTitles: items] }
    }

    fn select_item_at(&self, index: NSInteger) {
        unsafe { msg_send![self, selectItemAtIndex: index] }
    }

    fn set_action(&self, sel: Sel) {
        unsafe { msg_send![self, setAction: sel] }
    }

    fn set_target<O>(&self, target: Id<impl INSObject, O>) {
        unsafe { msg_send![self, setTarget: target] }
    }
}

object_struct!(NSPopUpButton);

impl INSView for NSPopUpButton {}
impl INSPopUpButton for NSPopUpButton {}
