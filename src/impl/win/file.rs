use crate::{Dialog, Error, Filter, OpenMultipleFile, OpenSingleDir, OpenSingleFile, Result};
use std::path::PathBuf;
use wfd::{
    DialogError, DialogParams, OpenDialogResult, FOS_ALLOWMULTISELECT, FOS_FILEMUSTEXIST,
    FOS_NOREADONLYRETURN, FOS_OVERWRITEPROMPT, FOS_PATHMUSTEXIST, FOS_PICKFOLDERS,
};

impl Dialog for OpenSingleFile<'_> {
    type Output = Option<PathBuf>;

    fn show(&mut self) -> Result<Self::Output> {
        super::process_init();

        let result = open_dialog(OpenDialogParams {
            dir: self.location,
            filters: &self.filters,
            multiple: false,
            open_dir: false,
        })?;

        Ok(result.map(|x| x.selected_file_path))
    }
}

impl Dialog for OpenMultipleFile<'_> {
    type Output = Vec<PathBuf>;

    fn show(&mut self) -> Result<Self::Output> {
        super::process_init();

        let result = open_dialog(OpenDialogParams {
            dir: self.location,
            filters: &self.filters,
            multiple: true,
            open_dir: false,
        })?;

        match result {
            Some(t) => Ok(t.selected_file_paths),
            None => Ok(vec![]),
        }
    }
}

impl Dialog for OpenSingleDir<'_> {
    type Output = Option<PathBuf>;

    fn show(&mut self) -> Result<Self::Output> {
        super::process_init();

        let result = open_dialog(OpenDialogParams {
            dir: self.location,
            filters: &[],
            multiple: false,
            open_dir: true,
        })?;

        Ok(result.map(|x| x.selected_file_path))
    }
}

struct OpenDialogParams<'a> {
    dir: Option<&'a str>,
    filters: &'a [Filter<'a>],
    multiple: bool,
    open_dir: bool,
}

fn open_dialog(params: OpenDialogParams) -> Result<Option<OpenDialogResult>> {
    let types: Vec<_> = params
        .filters
        .iter()
        .map(|filter| (filter.description, filter.extensions.join(";")))
        .collect();

    let mut options = FOS_PATHMUSTEXIST | FOS_FILEMUSTEXIST;
    if params.multiple {
        options |= FOS_ALLOWMULTISELECT;
    }
    if params.open_dir {
        options |= FOS_PICKFOLDERS;
    }

    let params = DialogParams {
        default_folder: params.dir.unwrap_or(""),
        file_types: types.iter().map(|t| (t.0, &*t.1)).collect(),
        options,
        ..Default::default()
    };

    let result = wfd::open_dialog(params);

    match result {
        Ok(t) => Ok(Some(t)),
        Err(e) => match e {
            DialogError::UserCancelled => Ok(None),
            DialogError::HResultFailed { error_method, .. } => {
                Err(Error::ImplementationError(error_method))
            }
        },
    }
}

#[allow(dead_code)]
fn save_dialog() {
    let mut _options = FOS_OVERWRITEPROMPT | FOS_PATHMUSTEXIST | FOS_NOREADONLYRETURN;
}
