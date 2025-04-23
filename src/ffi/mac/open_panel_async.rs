use std::path::PathBuf;

use objc2_app_kit::{NSModalResponseOK, NSOpenPanel};

use super::{AppKitFuture, NSSavePanelAsyncExt, NSURLExt};
use crate::utils::UnsafeWindowHandle;

pub trait NSOpenPanelAsyncExt {
    fn spawn(&self, owner: UnsafeWindowHandle) -> AppKitFuture<Vec<PathBuf>>;
}

impl NSOpenPanelAsyncExt for NSOpenPanel {
    fn spawn(&self, owner: UnsafeWindowHandle) -> AppKitFuture<Vec<PathBuf>> {
        let owner = unsafe { owner.as_appkit() };

        self.begin(owner.as_deref(), move |panel, response| {
            let panel = panel.downcast_ref::<NSOpenPanel>().unwrap();

            (response == NSModalResponseOK)
                .then(|| unsafe { panel.URLs() })
                .map(|urls| urls.into_iter().map(|x| x.to_path_buf()).collect())
                .unwrap_or_default()
        })
    }
}
