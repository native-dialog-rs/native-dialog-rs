use block2::RcBlock;
use objc2::rc::Retained as Id;
use objc2::runtime::AnyObject;
use objc2::{AnyThread, Message};
use objc2_app_kit::{
    NSColor, NSFont, NSFontAttributeName, NSGradient, NSGraphicsContext, NSImage, NSShadow,
    NSShadowAttributeName, NSStringDrawing,
};
use objc2_core_foundation::{CGFloat, CGPoint, CGRect, CGSize};
use objc2_core_graphics::{kCGColorBlack, CGBlendMode, CGColor, CGContext, CGImage};
use objc2_foundation::{NSMutableDictionary, NSRect, NSString};

pub trait NSImageExt {
    fn draw<F>(size: CGSize, draw: F) -> Id<Self>
    where
        F: Fn(CGRect) -> Option<()> + 'static;

    fn text(text: &str, scale: CGFloat, shadow: bool) -> Id<Self>;
    fn stack(back: &Self, front: &Self, stagger: (CGFloat, CGFloat)) -> Id<Self>;
    fn rect(&self) -> NSRect;
    fn etched(&self) -> Id<Self>;
}

impl NSImageExt for NSImage {
    fn draw<F>(size: CGSize, draw: F) -> Id<Self>
    where
        F: Fn(CGRect) -> Option<()> + 'static,
    {
        unsafe {
            let block = RcBlock::new(move |rect| draw(rect).is_some().into());
            NSImage::imageWithSize_flipped_drawingHandler(size, false, &block)
        }
    }

    fn text(text: &str, scale: CGFloat, shadow: bool) -> Id<Self> {
        let base = 256.0;
        let margin = base / 256.0 * scale;

        unsafe {
            let attrs = NSMutableDictionary::<NSString, AnyObject>::new();

            let font = NSFont::systemFontOfSize(base * scale);
            attrs.insert(NSFontAttributeName, &font);

            if shadow {
                let shadow = NSShadow::new();
                shadow.setShadowColor(Some(&NSColor::blackColor().colorWithAlphaComponent(0.4)));
                shadow.setShadowBlurRadius(2.0);
                shadow.setShadowOffset(CGSize::new(0.0, 0.75));
                attrs.insert(NSShadowAttributeName, &shadow);
            }

            let text = NSString::from_str(text);
            let size = text.sizeWithAttributes(Some(&attrs));

            let outer = CGSize::new(size.width + margin * 2.0, size.height + margin * 2.0);
            Self::draw(outer, move |rect| {
                let origin = CGPoint::new(margin, margin);
                let inner = CGRect::new(origin, rect.size);
                text.drawInRect_withAttributes(inner, Some(&attrs));
                Some(())
            })
        }
    }

    fn stack(back: &Self, front: &Self, stagger: (CGFloat, CGFloat)) -> Id<Self> {
        let back = back.retain();
        let front = front.retain();
        let size = unsafe { back.size() };

        Self::draw(size, move |rect| unsafe {
            back.drawInRect(rect);

            let outer = rect.size;
            let inner = front.size();
            let origin = CGPoint::new(
                (outer.width - inner.width) / 2.0 + outer.width * stagger.0 / 100.0,
                (outer.height - inner.height) / 2.0 + outer.height * stagger.1 / 100.0,
            );
            front.drawInRect(CGRect::new(origin, inner));

            Some(())
        })
    }

    fn rect(&self) -> NSRect {
        unsafe { NSRect::new(CGPoint::ZERO, self.size()) }
    }

    // Greatly inspired by https://stackoverflow.com/a/7138497
    fn etched(&self) -> Id<Self> {
        let this = self.retain();
        let size = unsafe { self.size() };
        Self::draw(size, move |rect: CGRect| unsafe {
            let mask = CGImage::from_ns_image(&this)?;
            let ctx = NSGraphicsContext::currentContext().unwrap().CGContext();

            let physical = CGContext::convert_rect_to_device_space(Some(&ctx), rect);

            // Draw white drop shadow (by moving the original image)
            let moved = mask.make_moved_mask(rect, CGPoint::new(0.0, -2.0))?;
            CGContext::clip_to_mask(Some(&ctx), rect, Some(&moved));
            CGContext::set_rgb_fill_color(Some(&ctx), 1.0, 1.0, 1.0, 1.0);
            CGContext::fill_rect(Some(&ctx), rect);

            // Draw gradient that is clipped to mask
            CGContext::clip_to_mask(Some(&ctx), rect, Some(&mask));
            let gradient = NSGradient::initWithStartingColor_endingColor(
                NSGradient::alloc(),
                &NSColor::colorWithDeviceWhite_alpha(0.5, 1.0),
                &NSColor::colorWithDeviceWhite_alpha(0.25, 1.0),
            )?;
            gradient.drawInRect_angle(rect, 90.0);

            // Draw inner shadow with inverted mask
            let offset = CGSize::new(0.0, physical.size.height / 1024.0);
            let blur = physical.size.height / 128.0;
            let color = CGColor::constant_color(Some(kCGColorBlack))?;
            let inverted = mask.make_inverted_mask(rect)?;
            CGContext::set_shadow_with_color(Some(&ctx), offset, blur, Some(&color));
            CGContext::draw_image(Some(&ctx), rect, Some(&inverted));

            Some(())
        })
    }
}

trait CGImageExt {
    fn from_ns_image(image: &NSImage) -> Option<Id<Self>>;
    fn make_inverted_mask(&self, rect: NSRect) -> Option<Id<Self>>;
    fn make_moved_mask(&self, rect: NSRect, delta: CGPoint) -> Option<Id<Self>>;
}

impl CGImageExt for CGImage {
    fn from_ns_image(image: &NSImage) -> Option<Id<Self>> {
        unsafe { image.CGImageForProposedRect_context_hints(&mut image.rect(), None, None) }
    }

    fn make_inverted_mask(&self, rect: NSRect) -> Option<Id<Self>> {
        let this = self.retain();
        let image = NSImage::draw(rect.size, move |rect: CGRect| unsafe {
            let ctx = NSGraphicsContext::currentContext().unwrap().CGContext();

            CGContext::set_blend_mode(Some(&ctx), CGBlendMode::XOR);
            CGContext::draw_image(Some(&ctx), rect, Some(&this));
            CGContext::set_rgb_fill_color(Some(&ctx), 1.0, 1.0, 1.0, 1.0);
            CGContext::fill_rect(Some(&ctx), rect);

            Some(())
        });

        Self::from_ns_image(&image)
    }

    fn make_moved_mask(&self, rect: NSRect, delta: CGPoint) -> Option<Id<Self>> {
        let this = self.retain();
        let image = NSImage::draw(rect.size, move |rect: CGRect| unsafe {
            let ctx = NSGraphicsContext::currentContext().unwrap().CGContext();
            let rect = CGRect::new(delta, rect.size);
            CGContext::draw_image(Some(&ctx), rect, Some(&this));
            Some(())
        });

        Self::from_ns_image(&image)
    }
}
