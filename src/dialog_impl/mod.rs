use crate::Result;

#[cfg(target_os = "macos")]
pub(crate) mod mac;

#[cfg(all(
    unix,
    not(target_os = "macos"),
    not(target_os = "ios"),
    not(target_os = "android")
))]
pub(crate) mod gnu;

#[cfg(target_os = "windows")]
pub(crate) mod win;

pub trait DialogImpl: crate::dialog::Dialog {
    type Impl;

    fn create(&self) -> Self::Impl;

    fn show(&mut self) -> Result<Self::Output>;

    #[cfg(feature = "async")]
    async fn spawn(&mut self) -> Result<Self::Output>;
}
