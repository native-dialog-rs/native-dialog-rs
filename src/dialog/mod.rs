mod file;
pub use file::*;

mod message;
pub use message::*;

pub trait Dialog {
    type Output;
}

macro_rules! dialog_delegate {
    () => {
        pub fn show(self) -> $crate::Result<<Self as $crate::dialog::Dialog>::Output> {
            <Self as $crate::dialog::DialogImpl>::show(self)
        }

        #[cfg(feature = "async")]
        pub async fn spawn(self) -> $crate::Result<<Self as $crate::dialog::Dialog>::Output> {
            <Self as $crate::dialog::DialogImpl>::spawn(self).await
        }
    };
}

use dialog_delegate;

pub trait DialogImpl: Dialog {
    fn show(self) -> crate::Result<Self::Output>;

    #[cfg(feature = "async")]
    async fn spawn(self) -> crate::Result<Self::Output>;
}

#[cfg(target_os = "macos")]
pub mod mac;

#[cfg(all(
    unix,
    not(target_os = "macos"),
    not(target_os = "ios"),
    not(target_os = "android")
))]
pub mod gnu;

#[cfg(target_os = "windows")]
pub mod win;
