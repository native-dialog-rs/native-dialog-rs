use std::os::unix::ffi::OsStrExt;
use std::path::Path;

use formatx::formatx;

#[derive(Debug, Clone, Default)]
pub struct FileFiltersBag {
    pub filters: Vec<FileFilter>,
}

impl FileFiltersBag {
    pub fn add(&mut self, description: impl ToString, extensions: &[impl ToString]) {
        if let Some(filter) = FileFilter::new(description, extensions) {
            self.filters.push(filter);
        }
    }

    pub fn clear(&mut self) {
        self.filters.clear();
    }

    pub fn first(&self) -> Option<&FileFilter> {
        self.filters.first()
    }

    pub fn is_empty(&self) -> bool {
        self.filters.is_empty()
    }

    pub fn get(&self, index: usize) -> Option<&FileFilter> {
        self.filters.get(index)
    }

    pub fn iter(&self) -> impl Iterator<Item = &FileFilter> {
        self.filters.iter()
    }

    pub fn accepts(&self, path: impl AsRef<Path>) -> bool {
        if self.filters.is_empty() {
            return true;
        }

        for filter in &self.filters {
            if filter.accepts(&path) {
                return true;
            }
        }

        false
    }
}

/// Represents a set of file extensions and their description.
#[derive(Debug, Clone)]
pub struct FileFilter {
    pub description: String,
    pub extensions: Vec<String>,
}

impl FileFilter {
    pub fn new(description: impl ToString, extensions: &[impl ToString]) -> Option<Self> {
        if extensions.is_empty() {
            return None;
        }

        Some(FileFilter {
            description: description.to_string(),
            extensions: extensions
                .iter()
                .map(ToString::to_string)
                .filter(|ext| !ext.is_empty())
                .map(|mut ext| {
                    if !ext.starts_with('.') {
                        ext.insert(0, '.');
                    }
                    ext
                })
                .collect(),
        })
    }

    pub fn accepts(&self, path: impl AsRef<Path>) -> bool {
        if let Some(name) = path.as_ref().file_name() {
            for accepting in &self.extensions {
                if name.as_bytes().ends_with(accepting.as_bytes()) {
                    return true;
                }
            }
        }

        false
    }

    pub fn format(&self, fmt_line: &str, fmt_type: &str, delimeter: &str) -> String {
        let types: Vec<String> = self
            .extensions
            .iter()
            .map(|ext| formatx!(fmt_type, ext = ext).unwrap())
            .collect();

        formatx!(
            fmt_line,
            desc = &self.description,
            types = types.join(delimeter)
        )
        .unwrap()
    }
}
