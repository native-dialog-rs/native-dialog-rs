use std::marker::PhantomData;

pub mod message;
pub use message::*;

pub mod file;
pub use file::*;

/// Builder for dialogs.
#[derive(Debug, Clone)]
pub struct DialogBuilder(PhantomData<()>);

impl DialogBuilder {
    pub fn file() -> FileDialogBuilder {
        FileDialogBuilder::default()
    }

    pub fn message() -> MessageDialogBuilder {
        MessageDialogBuilder::default()
    }
}
