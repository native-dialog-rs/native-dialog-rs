use std::cell::RefCell;

use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};

use crate::dialog::DialogImpl;
use crate::Result;

pub struct ProgressDialog<'a> {
    pub(crate) title: &'a str,
    pub(crate) text: &'a str,
    pub(crate) owner: Option<RawWindowHandle>,
}

pub trait ProgressHandle {
    fn set_progress(&mut self, percent: f32) -> Result<()>;
    fn set_text(&mut self, text: &str) -> Result<()>;
    fn check_cancelled(&mut self) -> Result<bool>;
    fn close(&mut self) -> Result<()>;
}

impl<'a> ProgressDialog<'a> {
    pub fn new() -> Self {
        ProgressDialog {
            title: "",
            text: "",
            owner: None,
        }
    }

    /// Set the title of the dialog.
    pub fn set_title(mut self, title: &'a str) -> Self {
        self.title = title;
        self
    }

    /// Set the message text of the dialog.
    pub fn set_text(mut self, text: &'a str) -> Self {
        self.text = text;
        self
    }

    /// Sets the owner of the dialog. On Unix and GNU/Linux, this is a no-op.
    pub fn set_owner<W: HasRawWindowHandle>(mut self, window: &W) -> Self {
        self.owner = Some(window.raw_window_handle());
        self
    }

    /// Sets the owner of the dialog by raw handle. On Unix and GNU/Linux, this is a no-op.
    ///pub(crate)
    /// # Safety
    ///
    /// It's the caller's responsibility that ensuring the handle is valid.
    pub unsafe fn set_owner_handle(mut self, handle: RawWindowHandle) -> Self {
        self.owner = Some(handle);
        self
    }

    /// Resets the owner of the dialog to nothing.
    pub fn reset_owner(mut self) -> Self {
        self.owner = None;
        self
    }

    pub fn show(&mut self) -> Result<Box<RefCell<dyn ProgressHandle>>> {
        DialogImpl::show(self)
    }
}

impl Default for ProgressDialog<'_> {
    fn default() -> Self {
        Self::new()
    }
}
