use crate::dialog::{DialogImpl, OpenMultipleFile, OpenSingleDir, OpenSingleFile};
use crate::{Error, Filter, Result};
use std::path::{Component, Path, PathBuf};
use wfd::{
    DialogError, DialogParams, OpenDialogResult, FOS_ALLOWMULTISELECT, FOS_FILEMUSTEXIST,
    FOS_NOREADONLYRETURN, FOS_OVERWRITEPROMPT, FOS_PATHMUSTEXIST, FOS_PICKFOLDERS,
};

impl DialogImpl for OpenSingleFile<'_> {
    fn show(&mut self) -> Result<Self::Output> {
        super::process_init();

        let result = open_dialog(OpenDialogParams {
            location: self.location,
            filters: &self.filters,
            multiple: false,
            open_dir: false,
        })?;

        Ok(result.map(|x| x.selected_file_path))
    }
}

impl DialogImpl for OpenMultipleFile<'_> {
    fn show(&mut self) -> Result<Self::Output> {
        super::process_init();

        let result = open_dialog(OpenDialogParams {
            location: self.location,
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

impl DialogImpl for OpenSingleDir<'_> {
    fn show(&mut self) -> Result<Self::Output> {
        super::process_init();

        let result = open_dialog(OpenDialogParams {
            location: self.location,
            filters: &[],
            multiple: false,
            open_dir: true,
        })?;

        Ok(result.map(|x| x.selected_file_path))
    }
}

fn resolve_tilde<P: AsRef<Path> + ?Sized>(path: &P) -> Option<PathBuf> {
    let mut result = PathBuf::new();

    let mut components = path.as_ref().components();
    match components.next() {
        Some(Component::Normal(c)) if c == "~" => result.push(dirs::home_dir()?),
        Some(c) => result.push(c),
        None => {}
    };
    result.extend(components);

    Some(result)
}

struct OpenDialogParams<'a> {
    location: Option<&'a Path>,
    filters: &'a [Filter<'a>],
    multiple: bool,
    open_dir: bool,
}

fn open_dialog(params: OpenDialogParams) -> Result<Option<OpenDialogResult>> {
    let location = params.location.and_then(resolve_tilde);

    let types: Vec<_> = params
        .filters
        .iter()
        .map(|filter| {
            let extensions = filter.extensions.iter().map(|x| format!("*.{}", x));
            (filter.description, extensions.collect::<Vec<_>>().join(";"))
        })
        .collect();

    let mut options = FOS_PATHMUSTEXIST | FOS_FILEMUSTEXIST;
    if params.multiple {
        options |= FOS_ALLOWMULTISELECT;
    }
    if params.open_dir {
        options |= FOS_PICKFOLDERS;
    }

    let params = DialogParams {
        folder: location.as_deref().and_then(Path::to_str).unwrap_or(""),
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
            DialogError::UnsupportedFilepath => Err(Error::ImplementationError(
                "Unsupported filepath".to_string(),
            )),
        },
    }
}

#[allow(dead_code)]
fn save_dialog() {
    let mut _options = FOS_OVERWRITEPROMPT | FOS_PATHMUSTEXIST | FOS_NOREADONLYRETURN;
}
