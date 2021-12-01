use super::ffi::cocoa::{INSAlert, INSBundle, NSAlert, NSBundle, NSImage};
use crate::dialog::{DialogImpl, MessageAlert, MessageConfirm};
use crate::{MessageType, Result};
use objc_foundation::INSObject;
use objc_id::Id;

impl DialogImpl for MessageAlert<'_> {
    fn show(&mut self) -> Result<Self::Output> {
        let panel = NSAlert::new();

        panel.set_informative_text(self.text);
        panel.set_message_text(self.title);
        panel.set_icon(get_dialog_icon(self.typ));

        panel.run_modal();

        Ok(())
    }
}

impl DialogImpl for MessageConfirm<'_> {
    fn show(&mut self) -> Result<Self::Output> {
        let panel = NSAlert::new();

        panel.set_informative_text(self.text);
        panel.set_message_text(self.title);
        panel.set_icon(get_dialog_icon(self.typ));

        panel.add_button("Yes");
        panel.add_button("No");

        let res = panel.run_modal();

        // NSAlertFirstButtonReturn = 1000
        Ok(res == 1000)
    }
}

fn get_dialog_icon(typ: MessageType) -> Id<NSImage> {
    let bundle = NSBundle::of_path("/System/Library/CoreServices/CoreTypes.bundle");

    let name = match typ {
        MessageType::Info => "AlertNoteIcon",
        MessageType::Warning => "AlertCautionIcon",
        MessageType::Error => "AlertStopIcon",
    };

    bundle.image_named(name)
}
