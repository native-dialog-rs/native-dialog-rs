use crate::{
    r#impl::OpenDialogTarget, Dialog, Error, OpenMultipleFile, OpenSingleDirectory, OpenSingleFile,
    Result,
};
use std::path::PathBuf;
use wfd::{
    DialogError, DialogParams, OpenDialogResult, FOS_ALLOWMULTISELECT, FOS_FILEMUSTEXIST,
    FOS_NOREADONLYRETURN, FOS_OVERWRITEPROMPT, FOS_PATHMUSTEXIST, FOS_PICKFOLDERS,
};

impl Dialog for OpenSingleFile<'_> {
    type Output = Option<String>;

    fn show(self) -> Result<Self::Output> {
        super::process_init();

        open_dialog(OpenDialogParams {
            dir: self.dir,
            filter: self.filter,
            multiple: false,
            target: OpenDialogTarget::File,
        })
        .map(|ok| ok.map(|some| path_to_string(some.selected_file_path)))
    }
}

impl Dialog for OpenMultipleFile<'_> {
    type Output = Vec<String>;

    fn show(self) -> Result<Self::Output> {
        super::process_init();

        let result = open_dialog(OpenDialogParams {
            dir: self.dir,
            filter: self.filter,
            multiple: true,
            target: OpenDialogTarget::File,
        });

        match result {
            Ok(Some(t)) => {
                let paths = t.selected_file_paths;
                let strings = paths.into_iter().map(path_to_string).collect();
                Ok(strings)
            }
            Ok(None) => Ok(vec![]),
            Err(e) => Err(e),
        }
    }
}

impl Dialog for OpenSingleDirectory<'_> {
    type Output = Option<String>;

    fn show(self) -> Result<Self::Output> {
        super::process_init();

        open_dialog(OpenDialogParams {
            dir: self.dir,
            filter: None,
            multiple: false,
            target: OpenDialogTarget::Directory,
        })
        .map(|ok| ok.map(|some| path_to_string(some.selected_file_path)))
    }
}

fn path_to_string(path: PathBuf) -> String {
    path.to_string_lossy().to_string()
}

struct OpenDialogParams<'a> {
    dir: Option<&'a str>,
    filter: Option<&'a [&'a str]>,
    multiple: bool,
    target: OpenDialogTarget,
}

fn open_dialog(params: OpenDialogParams) -> Result<Option<OpenDialogResult>> {
    let file_types = match params.filter {
        Some(filter) => {
            let types: Vec<String> = filter.iter().map(|s| format!("*.{}", s)).collect();
            types.join(";")
        }
        None => String::new(),
    };
    let file_types = match params.filter {
        Some(_) => vec![("", file_types.as_str())],
        None => vec![],
    };

    let mut options = FOS_PATHMUSTEXIST | FOS_FILEMUSTEXIST;
    if params.multiple {
        options |= FOS_ALLOWMULTISELECT;
    }
    if params.target == OpenDialogTarget::Directory {
        options |= FOS_PICKFOLDERS;
    }

    let params = DialogParams {
        default_folder: params.dir.unwrap_or(""),
        file_types,
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
