use objc2::rc::Retained as Id;
use objc2_app_kit::{NSOpenPanel, NSSavePanel};

use super::ffi::{NSOpenPanelExt, NSSavePanelExt, NSURLExt, SavePanelFilters};
use crate::dialog::{OpenMultipleFile, OpenSingleDir, OpenSingleFile, SaveSingleFile};
use crate::dialog_impl::DialogImpl;
use crate::Result;

impl DialogImpl for OpenSingleFile<'_> {
    type Impl = Id<NSOpenPanel>;

    fn create(&self) -> Self::Impl {
        let panel = NSOpenPanel::open_panel();

        panel.set_title(self.title);

        panel.set_can_choose_files(true);
        panel.set_can_choose_directories(false);
        panel.set_allows_multiple_selection(false);

        if let Some(filename) = self.filename {
            panel.set_name_field_string_value(filename);
        }

        if let Some(location) = self.location {
            panel.set_directory_url(&location.to_string_lossy());
        }

        panel.set_filters(&self.filters);

        panel
    }

    fn show(&mut self) -> Result<Self::Output> {
        match self.create().show(self.owner) {
            Some(urls) => {
                let url = urls.firstObject().unwrap();
                Ok(Some(url.to_path_buf()))
            }
            None => Ok(None),
        }
    }

    #[cfg(feature = "async")]
    async fn spawn(&mut self) -> Result<Self::Output> {
        match self.create().spawn(self.owner).await {
            Some(urls) => {
                let url = urls.firstObject().unwrap();
                Ok(Some(url.to_path_buf()))
            }
            None => Ok(None),
        }
    }
}

impl DialogImpl for OpenMultipleFile<'_> {
    type Impl = Id<NSOpenPanel>;

    fn create(&self) -> Self::Impl {
        let panel = NSOpenPanel::open_panel();

        panel.set_title(self.title);
        panel.set_can_choose_files(true);
        panel.set_can_choose_directories(false);
        panel.set_allows_multiple_selection(true);

        if let Some(filename) = self.filename {
            panel.set_name_field_string_value(filename);
        }

        if let Some(location) = self.location {
            panel.set_directory_url(&location.to_string_lossy());
        }

        panel.set_filters(&self.filters);

        panel
    }

    fn show(&mut self) -> Result<Self::Output> {
        match self.create().show(self.owner) {
            Some(urls) => Ok(urls.to_vec().into_iter().map(|x| x.to_path_buf()).collect()),
            None => Ok(vec![]),
        }
    }

    #[cfg(feature = "async")]
    async fn spawn(&mut self) -> crate::Result<Self::Output> {
        match self.create().spawn(self.owner).await {
            Some(urls) => Ok(urls.to_vec().into_iter().map(|x| x.to_path_buf()).collect()),
            None => Ok(vec![]),
        }
    }
}

impl DialogImpl for OpenSingleDir<'_> {
    type Impl = Id<NSOpenPanel>;

    fn create(&self) -> Self::Impl {
        let panel = NSOpenPanel::open_panel();

        panel.set_title(self.title);
        panel.set_can_choose_files(false);
        panel.set_can_choose_directories(true);
        panel.set_allows_multiple_selection(false);

        if let Some(filename) = self.filename {
            panel.set_name_field_string_value(filename);
        }

        if let Some(location) = self.location {
            panel.set_directory_url(&location.to_string_lossy());
        }

        panel
    }

    fn show(&mut self) -> Result<Self::Output> {
        match self.create().show(self.owner) {
            Some(urls) => {
                let url = urls.firstObject().unwrap();
                Ok(Some(url.to_path_buf()))
            }
            None => Ok(None),
        }
    }

    #[cfg(feature = "async")]
    async fn spawn(&mut self) -> Result<Self::Output> {
        match self.create().spawn(self.owner).await {
            Some(urls) => {
                let url = urls.firstObject().unwrap();
                Ok(Some(url.to_path_buf()))
            }
            None => Ok(None),
        }
    }
}

impl DialogImpl for SaveSingleFile<'_> {
    type Impl = Id<NSSavePanel>;

    fn create(&self) -> Self::Impl {
        let panel = NSSavePanel::save_panel();

        panel.set_title(self.title);
        panel.set_can_create_directories(false);
        panel.set_extension_hidden(false);

        if let Some(filename) = self.filename {
            panel.set_name_field_string_value(filename);
        }

        if let Some(location) = self.location {
            panel.set_directory_url(&location.to_string_lossy());
        }

        panel
    }

    fn show(&mut self) -> Result<Self::Output> {
        let panel = self.create();
        let _ = SavePanelFilters::attach(&panel, &self.filters);

        match panel.show(self.owner) {
            Some(url) => Ok(Some(url.to_path_buf())),
            None => Ok(None),
        }
    }

    #[cfg(feature = "async")]
    async fn spawn(&mut self) -> Result<Self::Output> {
        let panel = self.create();
        let _ = SavePanelFilters::attach(&panel, &self.filters);

        match panel.spawn(self.owner).await {
            Some(url) => Ok(Some(url.to_path_buf())),
            None => Ok(None),
        }
    }
}
