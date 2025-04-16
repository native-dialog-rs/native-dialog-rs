use native_dialog::{FileDialog, MessageDialogBuilder, MessageType};

fn main() {
    let path = FileDialog::new().open_single_file().show();

    let result = MessageDialogBuilder::new()
        .set_title("What is happening?")
        .set_text(&format!("Shit is on fire!\n\n{:?}", path))
        .set_type(MessageType::Warning)
        .confirm()
        .show();

    println!("{:?}", result);
}
