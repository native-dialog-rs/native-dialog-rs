use raw_window_handle::{AppKitWindowHandle, HasWindowHandle, RawWindowHandle, Win32WindowHandle};

#[cfg(not(target_os = "macos"))]
mod resolve_tilde {
    use dirs::home_dir;
    use std::path::{Component, Path, PathBuf};

    pub fn resolve_tilde<P: AsRef<Path> + ?Sized>(path: &P) -> Option<PathBuf> {
        let mut result = PathBuf::new();

        let mut components = path.as_ref().components();
        match components.next() {
            Some(Component::Normal(c)) if c == "~" => result.push(home_dir()?),
            Some(c) => result.push(c),
            None => {}
        };
        result.extend(components);

        Some(result)
    }
}

#[cfg(not(target_os = "macos"))]
pub use resolve_tilde::resolve_tilde;

#[derive(Debug, Clone, Copy)]
pub struct UnsafeWindowHandle {
    handle: RawWindowHandle,
}

unsafe impl Send for UnsafeWindowHandle {}
unsafe impl Sync for UnsafeWindowHandle {}

impl UnsafeWindowHandle {
    pub fn new<W: HasWindowHandle>(window: &W) -> Option<Self> {
        window.window_handle().ok().map(|handle| Self {
            handle: handle.as_raw(),
        })
    }

    /// SAFETY: must be called on the correct thread
    pub unsafe fn as_appkit(&self) -> Option<AppKitWindowHandle> {
        match self.handle {
            RawWindowHandle::AppKit(handle) => Some(handle),
            _ => None,
        }
    }

    pub unsafe fn as_win32(&self) -> Option<Win32WindowHandle> {
        match self.handle {
            RawWindowHandle::Win32(handle) => Some(handle),
            _ => None,
        }
    }

    pub unsafe fn as_x11(&self) -> Option<usize> {
        match self.handle {
            RawWindowHandle::Xlib(handle) => Some(handle.window as _),
            RawWindowHandle::Xcb(handle) => Some(handle.window.get() as _),
            _ => None,
        }
    }
}
