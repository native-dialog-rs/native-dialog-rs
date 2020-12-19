use native_dialog::{FileDialog, MessageDialog, MessageType};

fn main() {
    let path = FileDialog::new().show_open_single_file();

    let result = MessageDialog::new()
        .set_title("What is happening?")
        .set_text(&format!("Shit is on fire!\n\n{:?}", path))
        .set_type(MessageType::Warning)
        .show_confirm();

    println!("{:?}", result);
}
