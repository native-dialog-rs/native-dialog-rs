use cocoa::base::id;
use cocoa::foundation::NSInteger;
use objc::runtime::{BOOL, NO, YES};

#[inline(always)]
unsafe fn with_activation<T>(mut f: impl FnMut() -> T) -> T {
    let app: id = msg_send![class!(NSApplication), sharedApplication];
    let policy: NSInteger = msg_send![app, activationPolicy];

    if policy == 2 {
        let _: () = msg_send![app, setActivationPolicy:1];
    }

    let ret = f();

    if policy == 2 {
        let _: () = msg_send![app, setActivationPolicy:2];
    }

    ret
}

#[inline(always)]
fn objc_bool(value: bool) -> BOOL {
    match value {
        true => YES,
        false => NO,
    }
}

mod url;
pub use url::*;

mod alert;
pub use alert::*;

mod image;
pub use image::*;

mod color;
pub use color::*;

mod layout;
pub use layout::*;

mod view;
pub use view::*;

mod stack_view;
pub use stack_view::*;

mod text_field;
pub use text_field::*;

mod pop_up_button;
pub use pop_up_button::*;

mod open_panel;
pub use open_panel::*;

mod save_panel;
pub use save_panel::*;
