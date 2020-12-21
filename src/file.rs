use crate::dialog::{
    Dialog, DialogImpl, OpenMultipleFile, OpenSingleDir, OpenSingleFile, SaveSingleFile,
};
use crate::Result;
use std::path::Path;

/// Represents a set of file extensions and their description.
#[derive(Debug, Clone)]
pub struct Filter<'a> {
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
    /// Creates a file dialog builder.
    pub fn new() -> Self {
        FileDialog {
            filename: None,
            location: None,
            filters: vec![],
        }
    }

    /// Sets the default value of the filename text field in the dialog. For open dialogs of macOS
    /// and zenity, this is a no-op because there's no such text field on the dialog.
    pub fn set_filename(mut self, filename: &'a str) -> Self {
        self.filename = Some(filename);
        self
    }

    /// Resets the default value of the filename field in the dialog.
    pub fn reset_filename(mut self) -> Self {
        self.filename = None;
        self
    }

    /// Sets the default location that the dialog shows at open.
    pub fn set_location<P: AsRef<Path> + ?Sized>(mut self, path: &'a P) -> Self {
        self.location = Some(path.as_ref());
        self
    }

    /// Resets the default location that the dialog shows at open. Without a default location set,
    /// the dialog will probably use the current working directory as default location.
    pub fn reset_location(mut self) -> Self {
        self.location = None;
        self
    }

    /// Adds a file type filter. The filter must contains at least one extension, otherwise this
    /// method will panic. For dialogs that open directories, this is a no-op.
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

    /// Removes all file type filters.
    pub fn remove_all_filters(mut self) -> Self {
        self.filters = vec![];
        self
    }

    /// Shows a dialog that let users to open one file.
    pub fn show_open_single_file(self) -> Result<<OpenSingleFile<'a> as Dialog>::Output> {
        let mut dialog = OpenSingleFile {
            filename: self.filename,
            location: self.location,
            filters: self.filters,
        };
        dialog.show()
    }

    /// Shows a dialog that let users to open multiple files.
    pub fn show_open_multiple_file(self) -> Result<<OpenMultipleFile<'a> as Dialog>::Output> {
        let mut dialog = OpenMultipleFile {
            filename: self.filename,
            location: self.location,
            filters: self.filters,
        };
        dialog.show()
    }

    /// Shows a dialog that let users to open one directory.
    pub fn show_open_single_dir(self) -> Result<<OpenSingleDir<'a> as Dialog>::Output> {
        let mut dialog = OpenSingleDir {
            filename: self.filename,
            location: self.location,
        };
        dialog.show()
    }

    /// Shows a dialog that let users to save one file.
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
