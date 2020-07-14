#[cfg(target_os = "macos")]
pub(crate) mod mac;

#[cfg(target_os = "linux")]
pub(crate) mod gnu;

#[cfg(target_os = "windows")]
pub(crate) mod win;

#[derive(PartialEq)]
pub(crate) enum OpenDialogTarget {
    File,
    Directory,
}
