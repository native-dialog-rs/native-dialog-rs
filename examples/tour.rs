use native_dialog::{
    MessageAlert, MessageConfirm, OpenMultipleFile, OpenSingleDir, OpenSingleFile,
};

fn echo<T: std::fmt::Debug>(name: &str, value: &T) {
    MessageAlert::new()
        .title("Result")
        .text(&format!("{}: {:?}", &name, &value))
        .show()
        .unwrap();
}

fn main() {
    let result = MessageConfirm::new()
        .title("Tour")
        .text("Do you want to begin the tour?")
        .show()
        .unwrap();
    if !result {
        return;
    }
    echo("MessageConfirm", &result);

    let result = OpenSingleFile::new().location("~").show().unwrap();
    echo("OpenSingleFile", &result);

    let result = OpenMultipleFile::new()
        .filter("Rust Source", &["rs"])
        .filter("Image", &["png", "jpg", "gif"])
        .show()
        .unwrap();
    echo("OpenMultipleFile", &result);

    let result = OpenSingleDir::new().show().unwrap();
    echo("OpenSingleDir", &result);

    MessageAlert::new()
        .title("End")
        .text("That's the end!")
        .show()
        .unwrap();
}
