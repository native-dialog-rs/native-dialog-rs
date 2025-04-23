use objc2::rc::Retained as Id;
use objc2::{define_class, msg_send, MainThreadOnly, Message};
use objc2_app_kit::{NSOpenPanel, NSOpenSavePanelDelegate};
use objc2_foundation::{NSError, NSObject, NSObjectProtocol, NSString, NSURL};

use crate::dialog::Filter;

use super::NSOpenPanelExt;

pub struct OpenPanelDelegateIvars {
    _panel: Id<NSOpenPanel>,
    _filters: Vec<Filter>,
}

impl Drop for OpenPanelDelegateIvars {
    fn drop(&mut self) {
        println!("OpenPanelDelegateIvars dropped");
    }
}

define_class! {
    #[unsafe(super = NSObject)]
    #[thread_kind = MainThreadOnly]
    #[ivars = OpenPanelDelegateIvars]
    pub struct OpenPanelDelegate;

    unsafe impl NSObjectProtocol for OpenPanelDelegate {}

    unsafe impl NSOpenSavePanelDelegate for OpenPanelDelegate {
        // Not supported yet.
        // #[unsafe(method(panel:validateURL:error:_))]
        // fn panel_validateURL_error(&self, sender: &AnyObject, url: &NSURL) -> Result<(), Id<NSError>> {
        //     Ok(())
        // }
    }

    // Workaround for the above
    impl OpenPanelDelegate {
        #[unsafe(method(panel:validateURL:error:))]
        fn validate_url(&self, sender: &NSOpenPanel, url: &NSURL, error: Option<&mut *mut NSError>) {
            // TODO: custom open panel filter
            println!("TODO: {sender:?} {url:?}");

            if let Some(out) = error {
                let msg = format!("TODO: {url:?}");
                *out = Id::autorelease_ptr(NSError::new(1, &NSString::from_str(&msg)));
            }
        }
    }
}

impl OpenPanelDelegate {
    pub fn attach(panel: &NSOpenPanel, filters: &[Filter]) -> Id<Self> {
        let ivars = OpenPanelDelegateIvars {
            _panel: panel.retain(),
            _filters: filters.to_owned(),
        };

        let this = Self::alloc(panel.mtm()).set_ivars(ivars);
        let this: Id<Self> = unsafe { msg_send![super(this), init] };

        panel.set_delegate(&this);

        this
    }
}
