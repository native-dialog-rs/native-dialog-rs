use std::cell::Cell;
use std::path::PathBuf;

use block2::RcBlock;
use objc2::Message;
use objc2_app_kit::{NSModalResponse, NSModalResponseOK, NSSavePanel, NSWindow};

use super::{AppKitFuture, NSURLExt};
use crate::utils::UnsafeWindowHandle;

pub trait NSSavePanelAsyncExt {
    fn spawn(&self, owner: UnsafeWindowHandle) -> AppKitFuture<Option<PathBuf>>;

    fn begin<T, F>(&self, owner: Option<&NSWindow>, callback: F) -> AppKitFuture<T>
    where
        T: Default + Send + 'static,
        F: Fn(&NSSavePanel, NSModalResponse) -> T + Send + 'static;
}

impl NSSavePanelAsyncExt for NSSavePanel {
    fn spawn(&self, owner: UnsafeWindowHandle) -> AppKitFuture<Option<PathBuf>> {
        let owner = unsafe { owner.as_appkit() };

        self.begin(owner.as_deref(), move |panel, response| {
            (response == NSModalResponseOK)
                .then(|| unsafe { panel.URL() })
                .flatten()
                .map(|url| url.to_path_buf())
        })
    }

    fn begin<T, F>(&self, owner: Option<&NSWindow>, callback: F) -> AppKitFuture<T>
    where
        T: Default + Send + 'static,
        F: Fn(&NSSavePanel, NSModalResponse) -> T + Send + 'static,
    {
        let (send, recv) = futures_channel::oneshot::channel();

        let cell = Cell::new(Some(send));
        let panel = self.retain();
        let handler = RcBlock::new(move |response: NSModalResponse| {
            if let Some(send) = cell.take() {
                let _ = send.send(callback(&panel, response));
            }
        });

        unsafe {
            match owner {
                Some(window) => self.beginSheetModalForWindow_completionHandler(window, &handler),
                None => self.beginWithCompletionHandler(&handler),
            }
        }

        AppKitFuture::from_oneshot(recv)
    }
}
