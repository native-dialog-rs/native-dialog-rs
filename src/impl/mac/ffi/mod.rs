use cocoa::base::id;
use cocoa::foundation::NSInteger;

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

mod url;
pub use url::*;

mod alert;
pub use alert::*;

mod image;
pub use image::*;

mod open_panel;
pub use open_panel::*;

/*
mod save_panel;
pub use save_panel::*;
*/
