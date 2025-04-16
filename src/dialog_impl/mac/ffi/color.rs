use objc2::rc::Retained as Id;
use objc2_app_kit::NSColor;

pub trait NSColorExt {
    fn secondary_label_color() -> Id<NSColor>;
}

impl NSColorExt for NSColor {
    fn secondary_label_color() -> Id<NSColor> {
        unsafe { NSColor::secondaryLabelColor() }
    }
}
