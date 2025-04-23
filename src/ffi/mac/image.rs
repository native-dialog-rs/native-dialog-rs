use block2::RcBlock;
use objc2::rc::Retained as Id;
use objc2_app_kit::{
    NSColor, NSFont, NSFontAttributeName, NSImage, NSMutableParagraphStyle,
    NSParagraphStyleAttributeName, NSShadow, NSShadowAttributeName, NSStringDrawing,
    NSTextAlignment,
};
use objc2_core_foundation::{CGPoint, CGRect, CGSize};
use objc2_foundation::{NSDictionary, NSString};

pub trait NSImageExt {
    fn emoji(text: &str) -> Id<Self>;
}

impl NSImageExt for NSImage {
    fn emoji(text: &str) -> Id<Self> {
        let text = NSString::from_str(text);

        unsafe {
            let font = NSFont::systemFontOfSize(256.0);

            let style = NSMutableParagraphStyle::new();
            style.setAlignment(NSTextAlignment::Center);

            let shadow = NSShadow::new();
            shadow.setShadowColor(Some(&NSColor::blackColor().colorWithAlphaComponent(0.4)));
            shadow.setShadowBlurRadius(2.0);
            shadow.setShadowOffset(CGSize::new(0.0, -0.75));

            let attrs = NSDictionary::from_retained_objects(
                &[
                    NSFontAttributeName,
                    NSParagraphStyleAttributeName,
                    NSShadowAttributeName,
                ],
                &[font.into(), style.into(), shadow.into()],
            );

            let size = text.sizeWithAttributes(Some(&attrs));
            let padding = 1.0;

            NSImage::imageWithSize_flipped_drawingHandler(
                CGSize::new(size.width + padding * 2.0, size.height + padding * 2.0),
                false,
                &RcBlock::new(move |_| {
                    let rect = CGRect::new(CGPoint::new(padding, -padding), size);
                    text.drawInRect_withAttributes(rect, Some(&attrs));
                    true.into()
                }),
            )
        }
    }
}
