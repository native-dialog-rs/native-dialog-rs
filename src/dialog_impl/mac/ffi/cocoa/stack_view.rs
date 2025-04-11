use objc2::rc::Id;
use objc2_app_kit::{NSLayoutConstraintOrientation, NSStackView, NSStackViewGravity, NSView};
use objc2_foundation::{MainThreadMarker, NSEdgeInsets, NSRect};

pub trait INSStackView {
    fn new_empty() -> Id<Self>;

    fn new_with_frame(frame: NSRect) -> Id<Self>;

    fn set_hugging_priority(&self, priority: f32, orientation: NSLayoutConstraintOrientation);

    fn set_edge_insets(&self, insets: NSEdgeInsets);

    fn add_view_in_gravity(&self, view: &NSView, gravity: NSStackViewGravity);
}

impl INSStackView for NSStackView {
    fn new_empty() -> Id<Self> {
        // TODO: Main Thread Safety
        let mtm = unsafe { MainThreadMarker::new_unchecked() };
        unsafe { NSStackView::new(mtm) }
    }

    fn new_with_frame(frame: NSRect) -> Id<Self> {
        // TODO: Main Thread Safety
        let mtm = unsafe { MainThreadMarker::new_unchecked() };
        unsafe { NSStackView::initWithFrame(mtm.alloc(), frame) }
    }

    fn set_hugging_priority(&self, priority: f32, orientation: NSLayoutConstraintOrientation) {
        unsafe { self.setHuggingPriority_forOrientation(priority, orientation) }
    }

    fn set_edge_insets(&self, insets: NSEdgeInsets) {
        unsafe { self.setEdgeInsets(insets) }
    }

    fn add_view_in_gravity(&self, view: &NSView, gravity: NSStackViewGravity) {
        unsafe { self.addView_inGravity(&view, gravity) }
    }
}
