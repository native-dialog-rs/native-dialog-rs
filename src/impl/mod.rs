#[cfg(target_os = "macos")]
pub(crate) mod mac;

#[cfg(target_os = "linux")]
pub(crate) mod gnu;
