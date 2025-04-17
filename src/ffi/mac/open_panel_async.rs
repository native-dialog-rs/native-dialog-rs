use std::path::PathBuf;

use futures_channel::oneshot::Receiver;
use objc2_app_kit::{NSModalResponseOK, NSOpenPanel};

use super::{NSSavePanelAsyncExt, NSURLExt};
use crate::utils::UnsafeWindowHandle;

pub trait NSOpenPanelAsyncExt {
    fn spawn(&self, owner: UnsafeWindowHandle) -> Receiver<Vec<PathBuf>>;
}

impl NSOpenPanelAsyncExt for NSOpenPanel {
    fn spawn(&self, owner: UnsafeWindowHandle) -> Receiver<Vec<PathBuf>> {
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
