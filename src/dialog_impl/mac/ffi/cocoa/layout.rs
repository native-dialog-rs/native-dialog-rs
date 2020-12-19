use cocoa::appkit::CGFloat;

#[repr(C)]
pub struct NSEdgeInsets {
    top: CGFloat,
    left: CGFloat,
    bottom: CGFloat,
    right: CGFloat,
}

impl NSEdgeInsets {
    pub fn new(top: CGFloat, left: CGFloat, bottom: CGFloat, right: CGFloat) -> Self {
        NSEdgeInsets {
            top,
            left,
            bottom,
            right,
        }
    }
}

#[allow(dead_code)]
#[repr(usize)]
pub enum NSUserInterfaceLayoutOrientation {
    Horizontal,
    Vertical,
}
