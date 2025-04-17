use std::cell::Cell;
use std::path::PathBuf;

use block2::RcBlock;
use futures_channel::oneshot::Receiver;
use objc2::{MainThreadOnly, Message};
use objc2_app_kit::{NSModalResponse, NSModalResponseOK, NSSavePanel, NSWindow};

use crate::dialog::UnsafeWindowHandle;

use super::{NSURLExt, NSWindowExt};

pub trait NSSavePanelAsyncExt {
    fn spawn(&self, owner: Option<UnsafeWindowHandle>) -> Receiver<Option<PathBuf>>;

    fn begin<T, F>(&self, owner: Option<&NSWindow>, callback: F) -> Receiver<T>
    where
        T: Send + 'static,
        F: Fn(&NSSavePanel, NSModalResponse) -> T + Send + 'static;
}

impl NSSavePanelAsyncExt for NSSavePanel {
    fn spawn(&self, owner: Option<UnsafeWindowHandle>) -> Receiver<Option<PathBuf>> {
        let owner = NSWindow::from_handle(self.mtm(), owner);

        self.begin(owner.as_deref(), move |panel, response| {
            (response == NSModalResponseOK)
                .then(|| unsafe { panel.URL() })
                .flatten()
                .map(|url| url.to_path_buf())
        })
    }

    fn begin<T, F>(&self, owner: Option<&NSWindow>, callback: F) -> Receiver<T>
    where
        T: Send + 'static,
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

        recv
    }
}
