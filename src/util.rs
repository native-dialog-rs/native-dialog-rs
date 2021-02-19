#[cfg(not(target_os = "macos"))]
mod resolve_tilde {
    use dirs_next::home_dir;
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
