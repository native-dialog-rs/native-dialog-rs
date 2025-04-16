#[cfg(feature = "async")]
pub async fn spawn_thread<T, F>(f: F) -> Option<T>
where
    T: Send + 'static,
    F: FnOnce() -> T + Send + 'static,
{
    let (tx, rx) = futures_channel::oneshot::channel();

    std::thread::spawn(move || {
        let _ = tx.send(f());
    });

    rx.await.ok()
}

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
