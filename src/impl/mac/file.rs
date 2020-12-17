use super::ffi::{INSOpenPanel, NSOpenPanel, INSURL, NSURL};
use crate::r#impl::DialogImpl;
use crate::{Filter, OpenMultipleFile, OpenSingleDir, OpenSingleFile, Result};
use objc_foundation::{INSArray, INSMutableArray, INSObject, INSString, NSMutableArray, NSString};
use objc_id::Id;
use std::path::PathBuf;

impl DialogImpl for OpenSingleFile<'_> {
    type Output = Option<PathBuf>;

    fn show(&mut self) -> Result<Self::Output> {
        let panel = NSOpenPanel::open_panel();

        panel.set_can_choose_files(true);
        panel.set_can_choose_directories(false);
        panel.set_allows_multiple_selection(false);

        if let Some(location) = self.location {
            panel.set_directory_url(&location.to_string_lossy());
        }

        panel.set_allowed_file_types(get_allowed_types(&self.filters));

        match panel.run_modal() {
            Ok(urls) => {
                let url = urls.first_object().unwrap();
                Ok(Some(to_path_buf(&url)))
            }
            Err(_) => Ok(None),
        }
    }
}

impl DialogImpl for OpenMultipleFile<'_> {
    type Output = Vec<PathBuf>;

    fn show(&mut self) -> Result<Self::Output> {
        let panel = NSOpenPanel::open_panel();

        panel.set_can_choose_files(true);
        panel.set_can_choose_directories(false);
        panel.set_allows_multiple_selection(true);

        if let Some(location) = self.location {
            panel.set_directory_url(&location.to_string_lossy());
        }

        panel.set_allowed_file_types(get_allowed_types(&self.filters));

        match panel.run_modal() {
            Ok(urls) => Ok(urls.to_vec().into_iter().map(to_path_buf).collect()),
            Err(_) => Ok(vec![]),
        }
    }
}

impl DialogImpl for OpenSingleDir<'_> {
    type Output = Option<PathBuf>;

    fn show(&mut self) -> Result<Self::Output> {
        let panel = NSOpenPanel::open_panel();

        panel.set_can_choose_files(false);
        panel.set_can_choose_directories(true);
        panel.set_allows_multiple_selection(false);

        if let Some(location) = self.location {
            panel.set_directory_url(&location.to_string_lossy());
        }

        match panel.run_modal() {
            Ok(urls) => {
                let url = urls.first_object().unwrap();
                Ok(Some(to_path_buf(&url)))
            }
            Err(_) => Ok(None),
        }
    }
}

fn get_allowed_types(filters: &[Filter<'_>]) -> Id<impl INSArray<Item = NSString>> {
    let mut extensions = NSMutableArray::new();
    for filter in filters {
        for ext in filter.extensions {
            let s = NSString::from_str(ext);
            extensions.add_object(s);
        }
    }
    extensions
}

fn to_path_buf(url: &NSURL) -> PathBuf {
    url.absolute_string().as_str().into()
}
