use super::{INSImage, INSWindow, NSWindow};
use block::ConcreteBlock;
use cocoa::appkit::NSApp;
use cocoa::foundation::NSInteger;
use objc_foundation::{INSObject, INSString, NSString};
use objc_id::Id;

pub trait INSAlert: INSObject {
    fn set_informative_text(&self, text: &str) {
        let text = NSString::from_str(text);
        unsafe { msg_send![self, setInformativeText: text] }
    }

    fn set_message_text(&self, text: &str) {
        let text = NSString::from_str(text);
        unsafe { msg_send![self, setMessageText: text] }
    }

    fn set_icon(&self, icon: Id<impl INSImage>) {
        unsafe { msg_send![self, setIcon: icon] }
    }

    fn add_button(&self, title: &str) {
        let title = NSString::from_str(title);
        unsafe { msg_send![self, addButtonWithTitle: title] }
    }

    fn window(&self) -> Id<NSWindow> {
        unsafe {
            let ptr = msg_send![self, window];
            Id::from_ptr(ptr)
        }
    }

    fn run_modal(&self, owner: Option<Id<NSWindow>>) -> NSInteger {
        match owner {
            Some(window) => unsafe {
                let window = window.share();

                let owner_window = window.clone();
                let alert_window = self.window();
                let handler = ConcreteBlock::new(move |response: NSInteger| {
                    let () = msg_send![NSApp(), stopModalWithCode: response];
                    let () = msg_send![owner_window, endSheet:&*alert_window returnCode:response];
                    alert_window.order_out();
                });

                let () = msg_send![
                    self,
                    beginSheetModalForWindow:window
                    completionHandler:handler.copy()
                ];

                msg_send![NSApp(), runModalForWindow:self.window()]
            },
            None => unsafe { super::with_activation(|| msg_send![self, runModal]) },
        }
    }
}

object_struct!(NSAlert);

impl INSAlert for NSAlert {}
