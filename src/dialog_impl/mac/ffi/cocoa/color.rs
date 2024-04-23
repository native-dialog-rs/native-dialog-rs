use objc2::rc::Id;
use objc2_app_kit::NSColor;

pub trait INSColor {
    fn secondary_label_color() -> Id<NSColor> {
        unsafe { NSColor::secondaryLabelColor() }
    }
}

impl INSColor for NSColor {}
