use super::Dialog;
use crate::MessageType;
use raw_window_handle::RawWindowHandle;

pub struct MessageAlert<'a> {
    pub(crate) title: &'a str,
    pub(crate) text: &'a str,
    pub(crate) typ: MessageType,
    #[cfg_attr(not(target_os = "windows"), allow(dead_code))]
    pub(crate) owner: Option<RawWindowHandle>,
}

impl Dialog for MessageAlert<'_> {
    type Output = ();
}

pub struct FallbackMessageAlert<'a> {
    pub(crate) title: &'a str,
    pub(crate) text: &'a str,
    pub(crate) typ: MessageType,
}

impl Dialog for FallbackMessageAlert<'_> {
    type Output = ();
}

pub struct MessageConfirm<'a> {
    pub(crate) title: &'a str,
    pub(crate) text: &'a str,
    pub(crate) typ: MessageType,
    #[cfg_attr(not(target_os = "windows"), allow(dead_code))]
    pub(crate) owner: Option<RawWindowHandle>,
}

impl Dialog for MessageConfirm<'_> {
    type Output = bool;
}

pub struct FallbackMessageConfirm<'a> {
    pub(crate) title: &'a str,
    pub(crate) text: &'a str,
    pub(crate) typ: MessageType,
}

impl Dialog for FallbackMessageConfirm<'_> {
    type Output = bool;
}
