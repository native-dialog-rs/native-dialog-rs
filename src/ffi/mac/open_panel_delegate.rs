use objc2::rc::Retained as Id;
use objc2::{define_class, msg_send, DefinedClass, MainThreadOnly};
use objc2_app_kit::{NSOpenPanel, NSOpenSavePanelDelegate};
use objc2_foundation::{NSObject, NSObjectProtocol, NSURL};

use crate::dialog::FileFiltersBag;

use super::{NSOpenPanelExt, NSURLExt};

pub struct OpenPanelDelegateIvars {
    filters: FileFiltersBag,
}

define_class! {
    #[unsafe(super = NSObject)]
    #[thread_kind = MainThreadOnly]
    #[ivars = OpenPanelDelegateIvars]
    pub struct OpenPanelDelegate;

    unsafe impl NSObjectProtocol for OpenPanelDelegate {}

    unsafe impl NSOpenSavePanelDelegate for OpenPanelDelegate {
        #[unsafe(method(panel:shouldEnableURL:))]
        unsafe fn should_enable_url(&self, _sender: &NSOpenPanel, url: &NSURL) -> bool {
            matches!(url.to_path_buf().map(|x| self.ivars().filters.accepts(&x)), Some(true))
        }
    }
}

impl OpenPanelDelegate {
    pub fn attach(panel: &NSOpenPanel, filters: &FileFiltersBag) -> Id<Self> {
        let ivars = OpenPanelDelegateIvars {
            filters: filters.to_owned(),
        };

        let this = Self::alloc(panel.mtm()).set_ivars(ivars);
        let this: Id<Self> = unsafe { msg_send![super(this), init] };

        panel.set_delegate(&this);

        this
    }
}

#[cfg(feature = "async")]
impl super::AsyncDelegate for OpenPanelDelegate {}
