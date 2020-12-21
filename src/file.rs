use crate::dialog::{
    Dialog, DialogImpl, OpenMultipleFile, OpenSingleDir, OpenSingleFile, SaveSingleFile,
};
use crate::Result;
use std::path::Path;

/// Represents a set of file extensions and their description.
#[derive(Debug, Clone)]
pub struct Filter<'a> {
    #[cfg_attr(target_os = "macos", allow(dead_code))]
    pub(crate) description: &'a str,
    pub(crate) extensions: &'a [&'a str],
}

/// Builds and shows file dialogs.
#[derive(Debug, Clone)]
pub struct FileDialog<'a> {
    pub(crate) filename: Option<&'a str>,
    pub(crate) location: Option<&'a Path>,
    pub(crate) filters: Vec<Filter<'a>>,
}

impl<'a> FileDialog<'a> {
    pub fn new() -> Self {
        FileDialog {
            filename: None,
            location: None,
            filters: vec![],
        }
    }

    pub fn set_filename(mut self, filename: &'a str) -> Self {
        self.filename = Some(filename);
        self
    }

    pub fn reset_filename(mut self) -> Self {
        self.filename = None;
        self
    }

    pub fn set_location<P: AsRef<Path> + ?Sized>(mut self, path: &'a P) -> Self {
        self.location = Some(path.as_ref());
        self
    }

    pub fn reset_location(mut self) -> Self {
        self.location = None;
        self
    }

    pub fn add_filter(mut self, description: &'a str, extensions: &'a [&'a str]) -> Self {
        if extensions.is_empty() {
            panic!("The file extensions of a filter must be specified.")
        }
        self.filters.push(Filter {
            description,
            extensions,
        });
        self
    }

    pub fn remove_all_filters(mut self) -> Self {
        self.filters = vec![];
        self
    }

    pub fn show_open_single_file(self) -> Result<<OpenSingleFile<'a> as Dialog>::Output> {
        let mut dialog = OpenSingleFile {
            filename: self.filename,
            location: self.location,
            filters: self.filters,
        };
        dialog.show()
    }

    pub fn show_open_multiple_file(self) -> Result<<OpenMultipleFile<'a> as Dialog>::Output> {
        let mut dialog = OpenMultipleFile {
            filename: self.filename,
            location: self.location,
            filters: self.filters,
        };
        dialog.show()
    }

    pub fn show_open_single_dir(self) -> Result<<OpenSingleDir<'a> as Dialog>::Output> {
        let mut dialog = OpenSingleDir {
            filename: self.filename,
            location: self.location,
        };
        dialog.show()
    }

    pub fn show_save_single_file(self) -> Result<<SaveSingleFile<'a> as Dialog>::Output> {
        let mut dialog = SaveSingleFile {
            filename: self.filename,
            location: self.location,
            filters: self.filters,
        };
        dialog.show()
    }
}

impl Default for FileDialog<'_> {
    fn default() -> Self {
        Self::new()
    }
}
