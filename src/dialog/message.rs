use super::Dialog;
use crate::MessageType;
use raw_window_handle::RawWindowHandle;

pub struct MessageAlert<'a> {
    pub(crate) title: &'a str,
    pub(crate) text: &'a str,
    pub(crate) typ: MessageType,
    #[cfg_attr(not(any(target_os = "macos", target_os = "windows")), allow(dead_code))]
    pub(crate) owner: Option<RawWindowHandle>,
}

impl Dialog for MessageAlert<'_> {
    type Output = ();
}

pub struct MessageConfirm<'a> {
    pub(crate) title: &'a str,
    pub(crate) text: &'a str,
    pub(crate) typ: MessageType,
    #[cfg_attr(not(any(target_os = "macos", target_os = "windows")), allow(dead_code))]
    pub(crate) owner: Option<RawWindowHandle>,
}

impl Dialog for MessageConfirm<'_> {
    type Output = bool;
}
