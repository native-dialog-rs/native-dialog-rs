use dispatch2::run_on_main;
use objc2::rc::Retained as Id;
use objc2::MainThreadMarker;
use objc2_app_kit::{NSOpenPanel, NSSavePanel};

use crate::dialog::{DialogImpl, OpenMultipleFile, OpenSingleDir, OpenSingleFile, SaveSingleFile};
use crate::ffi::mac::{NSOpenPanelExt, NSSavePanelExt, OpenPanelDelegate, SavePanelDelegate};
use crate::Result;

impl OpenSingleFile {
    fn create(&self, mtm: MainThreadMarker) -> Id<NSOpenPanel> {
        let panel = NSOpenPanel::open_panel(mtm);

        panel.set_title(&self.title);

        panel.set_can_choose_files(true);
        panel.set_can_choose_directories(false);
        panel.set_allows_multiple_selection(false);

        if let Some(filename) = &self.filename {
            panel.set_name_field_string_value(filename);
        }

        if let Some(location) = &self.location {
            panel.set_directory_url(location);
        }

        panel
    }
}

impl DialogImpl for OpenSingleFile {
    fn show(self) -> Result<Self::Output> {
        let res = run_on_main(|mtm| {
            let panel = self.create(mtm);
            let _delegate = OpenPanelDelegate::attach(&panel, &self.filters);
            panel.show(self.owner)
        });

        Ok(res.into_iter().next())
    }

    #[cfg(feature = "async")]
    async fn spawn(self) -> Result<Self::Output> {
        use crate::ffi::mac::NSOpenPanelAsyncExt;

        let res = run_on_main(|mtm| {
            let panel = self.create(mtm);
            let delegate = OpenPanelDelegate::attach(&panel, &self.filters);
            panel.spawn(self.owner).retain(delegate)
        });

        Ok(res.await.into_iter().next())
    }
}

impl OpenMultipleFile {
    fn create(&self, mtm: MainThreadMarker) -> Id<NSOpenPanel> {
        let panel = NSOpenPanel::open_panel(mtm);

        panel.set_title(&self.title);

        panel.set_can_choose_files(true);
        panel.set_can_choose_directories(false);
        panel.set_allows_multiple_selection(true);

        if let Some(filename) = &self.filename {
            panel.set_name_field_string_value(filename);
        }

        if let Some(location) = &self.location {
            panel.set_directory_url(location);
        }

        panel
    }
}

impl DialogImpl for OpenMultipleFile {
    fn show(self) -> Result<Self::Output> {
        let res = run_on_main(|mtm| {
            let panel = self.create(mtm);
            let _delegate = OpenPanelDelegate::attach(&panel, &self.filters);
            panel.show(self.owner)
        });

        Ok(res)
    }

    #[cfg(feature = "async")]
    async fn spawn(self) -> Result<Self::Output> {
        use crate::ffi::mac::NSOpenPanelAsyncExt;

        let res = run_on_main(|mtm| {
            let panel = self.create(mtm);
            let delegate = OpenPanelDelegate::attach(&panel, &self.filters);
            panel.spawn(self.owner).retain(delegate)
        });

        Ok(res.await)
    }
}

impl OpenSingleDir {
    fn create(&self, mtm: MainThreadMarker) -> Id<NSOpenPanel> {
        let panel = NSOpenPanel::open_panel(mtm);

        panel.set_title(&self.title);
        panel.set_can_choose_files(false);
        panel.set_can_choose_directories(true);
        panel.set_allows_multiple_selection(false);

        if let Some(filename) = &self.filename {
            panel.set_name_field_string_value(filename);
        }

        if let Some(location) = &self.location {
            panel.set_directory_url(location);
        }

        panel
    }
}

impl DialogImpl for OpenSingleDir {
    fn show(self) -> Result<Self::Output> {
        let res = run_on_main(|mtm| {
            let panel = self.create(mtm);
            panel.show(self.owner)
        });

        Ok(res.into_iter().next())
    }

    #[cfg(feature = "async")]
    async fn spawn(self) -> Result<Self::Output> {
        use crate::ffi::mac::NSOpenPanelAsyncExt;

        let res = run_on_main(|mtm| {
            let panel = self.create(mtm);
            panel.spawn(self.owner)
        });

        Ok(res.await.into_iter().next())
    }
}

impl SaveSingleFile {
    fn create(&self, mtm: MainThreadMarker) -> Id<NSSavePanel> {
        let panel = NSSavePanel::save_panel(mtm);

        panel.set_title(&self.title);
        panel.set_can_create_directories(false);
        panel.set_extension_hidden(false);

        if let Some(filename) = &self.filename {
            panel.set_name_field_string_value(filename);
        }

        if let Some(location) = &self.location {
            panel.set_directory_url(location);
        }

        panel
    }
}

impl DialogImpl for SaveSingleFile {
    fn show(self) -> Result<Self::Output> {
        let res = run_on_main(|mtm| {
            let panel = self.create(mtm);
            let _delegate = SavePanelDelegate::attach(&panel, &self.filters);
            panel.show(self.owner)
        });

        Ok(res)
    }

    #[cfg(feature = "async")]
    async fn spawn(self) -> Result<Self::Output> {
        use crate::ffi::mac::NSSavePanelAsyncExt;

        let res = run_on_main(|mtm| {
            let panel = self.create(mtm);
            let delegate = SavePanelDelegate::attach(&panel, &self.filters);
            panel.spawn(self.owner).retain(delegate)
        });

        Ok(res.await)
    }
}
