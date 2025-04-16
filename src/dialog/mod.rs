pub trait Dialog {
    type Output;
}

macro_rules! dialog_delegate {
    () => {
        pub fn show(mut self) -> crate::Result<<Self as crate::dialog::Dialog>::Output> {
            <Self as crate::dialog_impl::DialogImpl>::show(&mut self)
        }

        #[cfg(feature = "async")]
        pub async fn spawn(mut self) -> crate::Result<<Self as crate::dialog::Dialog>::Output> {
            <Self as crate::dialog_impl::DialogImpl>::spawn(&mut self).await
        }
    };
}

mod file;
pub use file::*;

mod message;
pub use message::*;
