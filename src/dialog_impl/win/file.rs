use crate::dialog::{DialogImpl, OpenMultipleFile, OpenSingleDir, OpenSingleFile, SaveSingleFile};
use crate::util::resolve_tilde;
use crate::{Error, Filter, Result};
use raw_window_handle::RawWindowHandle;
use std::path::Path;
use wfd::{
    DialogError, DialogParams, OpenDialogResult, SaveDialogResult, FOS_ALLOWMULTISELECT,
    FOS_FILEMUSTEXIST, FOS_NOREADONLYRETURN, FOS_OVERWRITEPROMPT, FOS_PATHMUSTEXIST,
    FOS_PICKFOLDERS, FOS_STRICTFILETYPES, HWND,
};

impl DialogImpl for OpenSingleFile<'_> {
    fn show(&mut self) -> Result<Self::Output> {
        super::process_init();

        let result = open_dialog(OpenDialogParams {
            filename: self.filename,
            location: self.location,
            filters: &self.filters,
            owner: self.owner,
            multiple: false,
            dir: false,
        })?;

        Ok(result.map(|x| x.selected_file_path))
    }
}

impl DialogImpl for OpenMultipleFile<'_> {
    fn show(&mut self) -> Result<Self::Output> {
        super::process_init();

        let result = open_dialog(OpenDialogParams {
            filename: self.filename,
            location: self.location,
            filters: &self.filters,
            owner: self.owner,
            multiple: true,
            dir: false,
        })?;

        match result {
            Some(t) => Ok(t.selected_file_paths),
            None => Ok(vec![]),
        }
    }
}

impl DialogImpl for OpenSingleDir<'_> {
    fn show(&mut self) -> Result<Self::Output> {
        super::process_init();

        let result = open_dialog(OpenDialogParams {
            filename: self.filename,
            location: self.location,
            filters: &[],
            owner: self.owner,
            multiple: false,
            dir: true,
        })?;

        Ok(result.map(|x| x.selected_file_path))
    }
}

impl DialogImpl for SaveSingleFile<'_> {
    fn show(&mut self) -> Result<Self::Output> {
        super::process_init();

        let result = save_dialog(SaveDialogParams {
            filename: self.filename,
            location: self.location,
            filters: &self.filters,
            owner: self.owner,
        })?;

        Ok(result.map(|x| x.selected_file_path))
    }
}

struct OpenDialogParams<'a> {
    filename: Option<&'a str>,
    location: Option<&'a Path>,
    filters: &'a [Filter<'a>],
    owner: Option<RawWindowHandle>,
    multiple: bool,
    dir: bool,
}

fn open_dialog(params: OpenDialogParams) -> Result<Option<OpenDialogResult>> {
    let folder = params.location.and_then(resolve_tilde);
    let folder = folder.as_deref().and_then(Path::to_str).unwrap_or("");

    let file_types: Vec<_> = get_dialog_file_types(params.filters);
    let file_types = file_types.iter().map(|t| (t.0, &*t.1)).collect();

    let file_name = params.filename.unwrap_or("");

    let mut options = FOS_PATHMUSTEXIST | FOS_FILEMUSTEXIST | FOS_STRICTFILETYPES;
    if params.multiple {
        options |= FOS_ALLOWMULTISELECT;
    }
    if params.dir {
        options |= FOS_PICKFOLDERS;
    }

    let owner = match params.owner {
        Some(RawWindowHandle::Win32(handle)) => Some(handle.hwnd as HWND),
        _ => None,
    };

    let params = DialogParams {
        folder,
        file_types,
        file_name,
        options,
        owner,
        ..Default::default()
    };

    let result = wfd::open_dialog(params);

    convert_result(result)
}

struct SaveDialogParams<'a> {
    filename: Option<&'a str>,
    location: Option<&'a Path>,
    filters: &'a [Filter<'a>],
    owner: Option<RawWindowHandle>,
}

fn save_dialog(params: SaveDialogParams) -> Result<Option<SaveDialogResult>> {
    let folder = params.location.and_then(resolve_tilde);
    let folder = folder.as_deref().and_then(Path::to_str).unwrap_or("");

    let file_types: Vec<_> = get_dialog_file_types(params.filters);
    let file_types = file_types.iter().map(|t| (t.0, &*t.1)).collect();

    let file_name = params.filename.unwrap_or("");

    let default_extension = match params.filters {
        [first_filter, ..] => first_filter.extensions[0],
        _ => "",
    };

    let options =
        FOS_OVERWRITEPROMPT | FOS_PATHMUSTEXIST | FOS_NOREADONLYRETURN | FOS_STRICTFILETYPES;

    let owner = match params.owner {
        Some(RawWindowHandle::Win32(handle)) => Some(handle.hwnd as HWND),
        _ => None,
    };

    let params = DialogParams {
        folder,
        file_types,
        file_name,
        default_extension,
        options,
        owner,
        ..Default::default()
    };

    let result = wfd::save_dialog(params);

    convert_result(result)
}

fn get_dialog_file_types<'a>(filters: &'a [Filter<'a>]) -> Vec<(&'a str, String)> {
    filters
        .iter()
        .map(|filter| {
            let extensions = filter.extensions.iter().map(|x| format!("*.{}", x));
            (filter.description, extensions.collect::<Vec<_>>().join(";"))
        })
        .collect()
}

fn convert_result<T>(result: std::result::Result<T, DialogError>) -> Result<Option<T>> {
    match result {
        Ok(t) => Ok(Some(t)),
        Err(e) => match e {
            DialogError::UserCancelled => Ok(None),
            DialogError::HResultFailed { error_method, .. } => {
                Err(Error::ImplementationError(error_method))
            }
            DialogError::UnsupportedFilepath => Err(Error::ImplementationError(
                "Unsupported filepath".to_string(),
            )),
        },
    }
}
