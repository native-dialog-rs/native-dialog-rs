use crate::r#impl::DialogImpl;
use crate::{Error, OpenMultipleFile, OpenSingleDir, OpenSingleFile, Result};
use osascript::JavaScript;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::path::{Path, PathBuf};

impl DialogImpl for OpenSingleFile<'_> {
    type Output = Option<PathBuf>;

    fn show(&mut self) -> Result<Self::Output> {
        choose_file(ChooseFileParams {
            multiple: false,
            location: self.location,
            filters: self
                .filters
                .iter()
                .flat_map(|x| x.extensions)
                .cloned()
                .collect(),
            choose_folder: false,
        })
    }
}

impl DialogImpl for OpenMultipleFile<'_> {
    type Output = Vec<PathBuf>;

    fn show(&mut self) -> Result<Self::Output> {
        choose_file::<Option<_>>(ChooseFileParams {
            multiple: true,
            location: self.location,
            filters: self
                .filters
                .iter()
                .flat_map(|x| x.extensions)
                .cloned()
                .collect(),
            choose_folder: false,
        })
        .map(|opt| opt.unwrap_or_else(Vec::new))
    }
}

impl DialogImpl for OpenSingleDir<'_> {
    type Output = Option<PathBuf>;

    fn show(&mut self) -> Result<Self::Output> {
        choose_file(ChooseFileParams {
            multiple: false,
            location: self.location,
            filters: vec![],
            choose_folder: true,
        })
    }
}

#[derive(Serialize)]
struct ChooseFileParams<'a> {
    multiple: bool,
    location: Option<&'a Path>,
    filters: Vec<&'a str>,
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

        if ($params.location)
            options.defaultLocation = Path($params.location.replace(/^\~/, app.pathTo('home folder')));

        if ($params.filters.length)
            options.ofType = $params.filters;

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
