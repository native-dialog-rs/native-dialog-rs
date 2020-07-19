use crate::{Dialog, Error, OpenMultipleFile, OpenSingleDir, OpenSingleFile, Result};
use osascript::JavaScript;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::path::PathBuf;

impl Dialog for OpenSingleFile<'_> {
    type Output = Option<PathBuf>;

    fn show(self) -> Result<Self::Output> {
        choose_file(ChooseFileParams {
            multiple: false,
            dir: self.dir,
            filter: self.filter,
            choose_folder: false,
        })
    }
}

impl Dialog for OpenMultipleFile<'_> {
    type Output = Vec<PathBuf>;

    fn show(self) -> Result<Self::Output> {
        choose_file::<Option<_>>(ChooseFileParams {
            multiple: true,
            dir: self.dir,
            filter: self.filter,
            choose_folder: false,
        })
        .map(|opt| opt.unwrap_or_else(|| vec![]))
    }
}

impl Dialog for OpenSingleDir<'_> {
    type Output = Option<PathBuf>;

    fn show(self) -> Result<Self::Output> {
        choose_file(ChooseFileParams {
            multiple: false,
            dir: self.dir,
            filter: None,
            choose_folder: true,
        })
    }
}

#[derive(Serialize)]
struct ChooseFileParams<'a> {
    multiple: bool,
    dir: Option<&'a str>,
    filter: Option<&'a [&'a str]>,
    choose_folder: bool,
}

fn choose_file<T: DeserializeOwned>(params: ChooseFileParams) -> Result<T> {
    let script = JavaScript::new(
        // language=js
        r"
        const app = Application.currentApplication();
        app.includeStandardAdditions = true;

        const options = {
            multipleSelectionsAllowed: $params.multiple,
        };

        if ($params.dir)
            options.defaultLocation = Path($params.dir.replace(/^\~/, app.pathTo('home folder')));

        if ($params.filter)
            options.ofType = $params.filter;

        try {
            let path = $params.choose_folder ? app.chooseFolder(options) : app.chooseFile(options);
            
            if ($params.multiple)
                return path.map(x => x.toString());
            else 
                return path.toString();
        } catch (e) {
            return null;
        }
        ",
    );

    script.execute_with_params(params).map_err(Error::from)
}
