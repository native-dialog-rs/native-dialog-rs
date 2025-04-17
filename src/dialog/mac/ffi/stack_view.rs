use objc2::rc::Retained as Id;
use objc2_app_kit::{NSLayoutConstraintOrientation, NSStackView, NSStackViewGravity, NSView};
use objc2_foundation::{MainThreadMarker, NSEdgeInsets};

pub trait NSStackViewExt {
    fn new_empty(mtm: MainThreadMarker) -> Id<Self>;
    fn set_hugging_priority(&self, priority: f32, orientation: NSLayoutConstraintOrientation);
    fn set_edge_insets(&self, insets: NSEdgeInsets);
    fn add_view_in_gravity(&self, view: &NSView, gravity: NSStackViewGravity);
}

impl NSStackViewExt for NSStackView {
    fn new_empty(mtm: MainThreadMarker) -> Id<Self> {
        unsafe { NSStackView::new(mtm) }
    }

    fn set_hugging_priority(&self, priority: f32, orientation: NSLayoutConstraintOrientation) {
        unsafe { self.setHuggingPriority_forOrientation(priority, orientation) }
    }

    fn set_edge_insets(&self, insets: NSEdgeInsets) {
        unsafe { self.setEdgeInsets(insets) }
    }

    fn add_view_in_gravity(&self, view: &NSView, gravity: NSStackViewGravity) {
        unsafe { self.addView_inGravity(view, gravity) }
    }
}
