use native_dialog::{FileDialog, MessageDialogBuilder, MessageType};

fn echo<T: std::fmt::Debug>(name: &str, value: &T) {
    MessageDialogBuilder::new()
        .set_title("Result")
        .set_text(&format!("{}:\n{:#?}", &name, &value))
        .alert()
        .show()
        .unwrap();
}

fn main() {
    let result = MessageDialogBuilder::new()
        .set_title("Tour")
        .set_text("Do you want to begin the tour?")
        .set_type(MessageType::Warning)
        .confirm()
        .show()
        .unwrap();
    if !result {
        return;
    }
    echo("show_confirm", &result);

    let result = FileDialog::new()
        .set_location("~")
        .open_single_file()
        .show()
        .unwrap();
    echo("show_open_single_file", &result);

    let result = FileDialog::new()
        .add_filter("Rust Source", &["rs"])
        .add_filter("Image", &["png", "jpg", "gif"])
        .open_multiple_file()
        .show()
        .unwrap();
    echo("show_open_multiple_file", &result);

    let result = FileDialog::new().open_single_dir().show().unwrap();
    echo("show_open_single_dir", &result);

    let result = FileDialog::new()
        .add_filter("Rust Source", &["rs"])
        .add_filter("Image", &["png", "jpg", "gif"])
        .save_single_file()
        .show()
        .unwrap();
    echo("show_save_single_file", &result);

    MessageDialogBuilder::new()
        .set_title("End")
        .set_text("That's the end!")
        .alert()
        .show()
        .unwrap();
}
