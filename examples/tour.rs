use native_dialog::{FileDialog, MessageDialog};

fn echo<T: std::fmt::Debug>(name: &str, value: &T) {
    MessageDialog::new()
        .set_title("Result")
        .set_text(&format!("{}: {:?}", &name, &value))
        .alert()
        .unwrap();
}

fn main() {
    let result = MessageDialog::new()
        .set_title("Tour")
        .set_text("Do you want to begin the tour?")
        .confirm()
        .unwrap();
    if !result {
        return;
    }
    echo("MessageConfirm", &result);

    let result = FileDialog::new()
        .set_location("~")
        .open_single_file()
        .unwrap();
    echo("OpenSingleFile", &result);

    let result = FileDialog::new()
        .add_filter("Rust Source", &["rs"])
        .add_filter("Image", &["png", "jpg", "gif"])
        .open_single_file()
        .unwrap();
    echo("OpenMultipleFile", &result);

    let result = FileDialog::new().open_single_file().unwrap();
    echo("OpenSingleDir", &result);

    MessageDialog::new()
        .set_title("End")
        .set_text("That's the end!")
        .alert()
        .unwrap();
}
