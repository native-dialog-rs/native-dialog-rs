use objc2::rc::Retained as Id;
use objc2::runtime::Sel;
use objc2_app_kit::NSPopUpButton;
use objc2_foundation::{MainThreadMarker, NSArray, NSInteger, NSObject, NSRect, NSString};

pub trait NSPopUpButtonExt {
    fn new_with_frame(mtm: MainThreadMarker, frame: NSRect) -> Id<Self>;
    fn add_items_with_titles(&self, items: &NSArray<NSString>);
    fn select_item_at(&self, index: NSInteger);
    fn set_action(&self, sel: Sel);
    fn set_target(&self, target: &NSObject);
}

impl NSPopUpButtonExt for NSPopUpButton {
    fn new_with_frame(mtm: MainThreadMarker, frame: NSRect) -> Id<Self> {
        unsafe { NSPopUpButton::initWithFrame_pullsDown(mtm.alloc(), frame, false) }
    }

    fn add_items_with_titles(&self, items: &NSArray<NSString>) {
        unsafe { self.addItemsWithTitles(items) }
    }

    fn select_item_at(&self, index: NSInteger) {
        unsafe { self.selectItemAtIndex(index) }
    }

    fn set_action(&self, sel: Sel) {
        unsafe { self.setAction(Some(sel)) }
    }

    fn set_target(&self, target: &NSObject) {
        unsafe { self.setTarget(Some(target)) }
    }
}
