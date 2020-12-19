use super::{INSView, NSEdgeInsets, NSUserInterfaceLayoutOrientation};
use cocoa::base::id;
use cocoa::foundation::NSRect;
use objc_id::Id;

#[allow(dead_code)]
#[repr(usize)]
pub enum NSStackViewGravity {
    Leading = 1,
    Center = 2,
    Trailing = 3,
}

pub trait INSStackView: INSView {
    fn new_with_frame(frame: NSRect) -> Id<Self> {
        unsafe {
            let ptr: id = msg_send![class!(NSStackView), alloc];
            let ptr = msg_send![ptr, initWithFrame: frame];
            Id::from_retained_ptr(ptr)
        }
    }

    fn set_hugging_priority(&self, priority: f32, orientation: NSUserInterfaceLayoutOrientation) {
        unsafe { msg_send![self, setHuggingPriority:priority forOrientation:orientation] }
    }

    fn set_edge_insets(&self, insets: NSEdgeInsets) {
        unsafe { msg_send![self, setEdgeInsets: insets] }
    }

    fn add_view_in_gravity(&self, view: Id<impl INSView>, gravity: NSStackViewGravity) {
        unsafe { msg_send![self, addView:view inGravity:gravity] }
    }
}

object_struct!(NSStackView);

impl INSView for NSStackView {}
impl INSStackView for NSStackView {}
