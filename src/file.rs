use crate::dialog::{DialogImpl, OpenMultipleFile, OpenSingleDir, OpenSingleFile, SaveSingleFile};
use crate::Result;
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use std::path::{Path, PathBuf};

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
    pub(crate) owner: Option<RawWindowHandle>,
}

impl<'a> FileDialog<'a> {
    /// Creates a file dialog builder.
    pub fn new() -> Self {
        FileDialog {
            filename: None,
            location: None,
            filters: vec![],
            owner: None,
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

    /// Sets the owner of the dialog. On Unix and GNU/Linux, this is a no-op.
    pub fn set_owner<W: HasRawWindowHandle>(mut self, window: &W) -> Self {
        self.owner = Some(window.raw_window_handle());
        self
    }

    /// Sets the owner of the dialog by raw handle. On Unix and GNU/Linux, this is a no-op.
    ///
    /// # Safety
    ///
    /// It's the caller's responsibility that ensuring the handle is valid.
    pub unsafe fn set_owner_handle(mut self, handle: RawWindowHandle) -> Self {
        self.owner = Some(handle);
        self
    }

    /// Resets the owner of the dialog to nothing.
    pub fn reset_owner(mut self) -> Self {
        self.owner = None;
        self
    }

    /// Shows a dialog that let users to open one file.
    pub fn show_open_single_file(self) -> Result<Option<PathBuf>> {
        let mut dialog = OpenSingleFile {
            filename: self.filename,
            location: self.location,
            filters: self.filters,
            owner: self.owner,
        };
        dialog.show()
    }

    /// Shows a dialog that let users to open multiple files.
    pub fn show_open_multiple_file(self) -> Result<Vec<PathBuf>> {
        let mut dialog = OpenMultipleFile {
            filename: self.filename,
            location: self.location,
            filters: self.filters,
            owner: self.owner,
        };
        dialog.show()
    }

    /// Shows a dialog that let users to open one directory.
    pub fn show_open_single_dir(self) -> Result<Option<PathBuf>> {
        let mut dialog = OpenSingleDir {
            filename: self.filename,
            location: self.location,
            owner: self.owner,
        };
        dialog.show()
    }

    /// Shows a dialog that let users to save one file.
    pub fn show_save_single_file(self) -> Result<Option<PathBuf>> {
        let mut dialog = SaveSingleFile {
            filename: self.filename,
            location: self.location,
            filters: self.filters,
            owner: self.owner,
        };
        dialog.show()
    }
}

impl Default for FileDialog<'_> {
    fn default() -> Self {
        Self::new()
    }
}
