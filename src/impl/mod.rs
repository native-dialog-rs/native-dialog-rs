pub trait DialogImpl {
    type Output;

    fn show(&mut self) -> crate::Result<Self::Output>;
}

macro_rules! show_impl {
    () => {
        pub fn show(&mut self) -> crate::Result<<Self as crate::r#impl::DialogImpl>::Output> {
            crate::r#impl::DialogImpl::show(self)
        }
    };
}

#[cfg(target_os = "macos")]
pub(crate) mod mac;

#[cfg(all(unix, not(target_os = "macos"), not(target_os = "ios"), not(target_os = "android")))]
pub(crate) mod gnu;

#[cfg(target_os = "windows")]
pub(crate) mod win;
