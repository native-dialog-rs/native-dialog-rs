pub trait Dialog {
    type Output;
}

pub trait DialogImpl: Dialog {
    fn show(&mut self) -> crate::Result<Self::Output>;
}

mod file;
pub use file::*;

mod message;
pub use message::*;
