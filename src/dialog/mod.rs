pub use file::*;
pub use message::*;
pub use progress::*;

pub trait Dialog {
    type Output;
}

pub trait DialogImpl: Dialog {
    fn show(&mut self) -> crate::Result<Self::Output>;
}

mod file;
mod message;
mod progress;
