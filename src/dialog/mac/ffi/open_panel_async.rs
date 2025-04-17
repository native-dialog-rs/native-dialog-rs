use std::path::PathBuf;

use futures_channel::oneshot::Receiver;
use objc2::MainThreadOnly;
use objc2_app_kit::{NSModalResponseOK, NSOpenPanel, NSWindow};

use crate::dialog::UnsafeWindowHandle;

use super::{NSSavePanelAsyncExt, NSURLExt, NSWindowExt};

pub trait NSOpenPanelAsyncExt {
    fn spawn(&self, owner: Option<UnsafeWindowHandle>) -> Receiver<Vec<PathBuf>>;
}

impl NSOpenPanelAsyncExt for NSOpenPanel {
    fn spawn(&self, owner: Option<UnsafeWindowHandle>) -> Receiver<Vec<PathBuf>> {
        let owner = NSWindow::from_handle(self.mtm(), owner);

        self.begin(owner.as_deref(), move |panel, response| {
            let panel = panel.downcast_ref::<NSOpenPanel>().unwrap();

            (response == NSModalResponseOK)
                .then(|| unsafe { panel.URLs() })
                .map(|urls| urls.into_iter().map(|x| x.to_path_buf()).collect())
                .unwrap_or_default()
        })
    }
}
