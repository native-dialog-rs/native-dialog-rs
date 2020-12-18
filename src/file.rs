use crate::dialog::{
    Dialog, DialogImpl, OpenMultipleFile, OpenSingleDir, OpenSingleFile, SaveSingleFile,
};
use crate::Result;
use std::path::Path;

/// Represent a set of file extensions and their description.
#[derive(Debug, Clone)]
pub struct Filter<'a> {
    #[cfg_attr(target_os = "macos", allow(dead_code))]
    pub(crate) description: &'a str,
    pub(crate) extensions: &'a [&'a str],
}

/// The builder of file dialogs.
#[derive(Debug, Clone)]
pub struct FileDialog<'a> {
    pub(crate) location: Option<&'a Path>,
    pub(crate) filters: Vec<Filter<'a>>,
}

impl<'a> FileDialog<'a> {
    pub fn new() -> Self {
        FileDialog {
            location: None,
            filters: vec![],
        }
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

    pub fn open_single_file(self) -> Result<<OpenSingleFile<'a> as Dialog>::Output> {
        let mut dialog = OpenSingleFile {
            location: self.location,
            filters: self.filters,
        };
        dialog.show()
    }

    pub fn open_multiple_file(self) -> Result<<OpenMultipleFile<'a> as Dialog>::Output> {
        let mut dialog = OpenMultipleFile {
            location: self.location,
            filters: self.filters,
        };
        dialog.show()
    }

    pub fn open_single_dir(self) -> Result<<OpenSingleDir<'a> as Dialog>::Output> {
        let mut dialog = OpenSingleDir {
            location: self.location,
        };
        dialog.show()
    }

    pub fn save_single_file(self) -> SaveSingleFile<'a> {
        SaveSingleFile {
            location: self.location,
            filters: self.filters,
        }
    }
}

impl Default for FileDialog<'_> {
    fn default() -> Self {
        Self::new()
    }
}
