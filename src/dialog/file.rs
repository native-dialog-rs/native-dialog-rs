use std::path::PathBuf;

use formatx::formatx;

use super::Dialog;
use crate::util::UnsafeWindowHandle;

/// Represents a set of file extensions and their description.
#[derive(Debug, Clone)]
pub struct Filter {
    pub description: String,
    pub extensions: Vec<String>,
}

impl Filter {
    pub fn format(&self, fmt_line: &str, fmt_type: &str, delimeter: &str) -> String {
        let exts: Vec<String> = self
            .extensions
            .iter()
            .map(|ext| formatx!(fmt_type, ext = ext).unwrap())
            .collect();

        formatx!(
            fmt_line,
            desc = &self.description,
            types = exts.join(delimeter)
        )
        .unwrap()
    }
}

pub struct OpenSingleFile {
    pub filename: Option<String>,
    pub location: Option<PathBuf>,
    pub title: String,
    pub filters: Vec<Filter>,
    pub owner: Option<UnsafeWindowHandle>,
}

impl Dialog for OpenSingleFile {
    type Output = Option<PathBuf>;
}

impl OpenSingleFile {
    super::dialog_delegate!();
}

pub struct OpenMultipleFile {
    pub filename: Option<String>,
    pub location: Option<PathBuf>,
    pub title: String,
    pub filters: Vec<Filter>,
    pub owner: Option<UnsafeWindowHandle>,
}

impl Dialog for OpenMultipleFile {
    type Output = Vec<PathBuf>;
}

impl OpenMultipleFile {
    super::dialog_delegate!();
}

pub struct OpenSingleDir {
    pub filename: Option<String>,
    pub location: Option<PathBuf>,
    pub title: String,
    pub owner: Option<UnsafeWindowHandle>,
}

impl Dialog for OpenSingleDir {
    type Output = Option<PathBuf>;
}

impl OpenSingleDir {
    super::dialog_delegate!();
}

pub struct SaveSingleFile {
    pub filename: Option<String>,
    pub location: Option<PathBuf>,
    pub title: String,
    pub filters: Vec<Filter>,
    pub owner: Option<UnsafeWindowHandle>,
}

impl Dialog for SaveSingleFile {
    type Output = Option<PathBuf>;
}

impl SaveSingleFile {
    super::dialog_delegate!();
}
