use super::ffi::{INSAlert, INSImage, NSAlert, NSImage};
use crate::dialog::{DialogImpl, MessageAlert, MessageConfirm};
use crate::{MessageType, Result};
use objc_foundation::INSObject;

impl DialogImpl for MessageAlert<'_> {
    fn show(&mut self) -> Result<Self::Output> {
        let panel = NSAlert::new();

        panel.set_informative_text(self.text);
        panel.set_message_text(self.title);
        panel.set_icon(NSImage::of_file(&get_dialog_icon_path(self.typ)));

        panel.run_modal();

        Ok(())
    }
}

impl DialogImpl for MessageConfirm<'_> {
    fn show(&mut self) -> Result<Self::Output> {
        let panel = NSAlert::new();

        panel.set_informative_text(self.text);
        panel.set_message_text(self.title);
        panel.set_icon(NSImage::of_file(&get_dialog_icon_path(self.typ)));

        panel.add_button("Yes");
        panel.add_button("No");

        let res = panel.run_modal();

        // NSAlertFirstButtonReturn = 1000
        Ok(res == 1000)
    }
}

fn get_dialog_icon_path(typ: MessageType) -> String {
    let basename = match typ {
        MessageType::Info => "AlertNoteIcon",
        MessageType::Warning => "AlertCautionIcon",
        MessageType::Error => "AlertStopIcon",
    };

    format!(
        "/System/Library/CoreServices/CoreTypes.bundle/Contents/Resources/{}.icns",
        basename,
    )
}
