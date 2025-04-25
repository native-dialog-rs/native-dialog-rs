use block2::RcBlock;
use objc2::rc::Retained as Id;
use objc2::Message;
use objc2_app_kit::{NSAlert, NSApplication, NSSavePanel};
use objc2_app_kit::{NSApplicationActivationPolicy, NSModalResponse, NSWindow};

pub trait NSApplicationExt {
    fn run_modal<T: SheetModal>(&self, modal: &T) -> NSModalResponse;
    fn run_sheet<T: SheetModal>(&self, window: &NSWindow, sheet: &T) -> NSModalResponse;
}

impl NSApplicationExt for NSApplication {
    fn run_modal<T: SheetModal>(&self, modal: &T) -> NSModalResponse {
        let policy = unsafe { self.activationPolicy() };

        self.setActivationPolicy(NSApplicationActivationPolicy::Accessory);
        let response = modal.modal(self);
        self.setActivationPolicy(policy);

        response
    }

    fn run_sheet<T: SheetModal>(&self, window: &NSWindow, sheet: &T) -> NSModalResponse {
        let this = self.retain();
        let handler = RcBlock::new(move |response| unsafe {
            // This is like... using NSApp as a channel that is synchronous
            // but doesn't block the main dispatcher (event loop). Magical!
            // Really I don't have a clue how it works.
            this.stopModalWithCode(response)
        });

        let sheet = sheet.sheet(window, handler);

        unsafe { self.runModalForWindow(&sheet) }
    }
}

type SheetHandler = RcBlock<dyn Fn(NSModalResponse)>;

pub trait SheetModal {
    fn sheet(&self, window: &NSWindow, handler: SheetHandler) -> Id<NSWindow>;
    fn modal(&self, app: &NSApplication) -> NSModalResponse;
}

impl SheetModal for NSAlert {
    fn sheet(&self, window: &NSWindow, handler: SheetHandler) -> Id<NSWindow> {
        unsafe {
            self.beginSheetModalForWindow_completionHandler(window, Some(&handler));
            self.window()
        }
    }

    fn modal(&self, _: &NSApplication) -> NSModalResponse {
        unsafe { self.runModal() }
    }
}

impl SheetModal for NSSavePanel {
    fn sheet(&self, window: &NSWindow, handler: SheetHandler) -> Id<NSWindow> {
        unsafe { self.beginSheetModalForWindow_completionHandler(window, &handler) };
        NSWindow::retain(self)
    }

    fn modal(&self, app: &NSApplication) -> NSModalResponse {
        unsafe { app.runModalForWindow(self) }
    }
}
