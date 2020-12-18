use super::Dialog;
use crate::MessageType;

pub struct MessageAlert<'a> {
    pub(crate) title: &'a str,
    pub(crate) text: &'a str,
    pub(crate) typ: MessageType,
}

impl Dialog for MessageAlert<'_> {
    type Output = ();
}

pub struct MessageConfirm<'a> {
    pub(crate) title: &'a str,
    pub(crate) text: &'a str,
    pub(crate) typ: MessageType,
}

impl Dialog for MessageConfirm<'_> {
    type Output = bool;
}
