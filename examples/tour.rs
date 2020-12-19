use native_dialog::{FileDialog, MessageDialog, MessageType};

fn echo<T: std::fmt::Debug>(name: &str, value: &T) {
    MessageDialog::new()
        .set_title("Result")
        .set_text(&format!("{}:\n{:#?}", &name, &value))
        .show_alert()
        .unwrap();
}

fn main() {
    let result = MessageDialog::new()
        .set_title("Tour")
        .set_text("Do you want to begin the tour?")
        .set_type(MessageType::Warning)
        .show_confirm()
        .unwrap();
    if !result {
        return;
    }
    echo("show_confirm", &result);

    let result = FileDialog::new()
        .set_location("~")
        .show_open_single_file()
        .unwrap();
    echo("show_open_single_file", &result);

    let result = FileDialog::new()
        .add_filter("Rust Source", &["rs"])
        .add_filter("Image", &["png", "jpg", "gif"])
        .show_open_multiple_file()
        .unwrap();
    echo("show_open_multiple_file", &result);

    let result = FileDialog::new().show_open_single_dir().unwrap();
    echo("show_open_single_dir", &result);

    let result = FileDialog::new().show_save_single_file().unwrap();
    echo("show_save_single_file", &result);

    MessageDialog::new()
        .set_title("End")
        .set_text("That's the end!")
        .show_alert()
        .unwrap();
}
