use native_dialog::*;

fn echo<T: std::fmt::Debug>(name: &str, value: &T) {
    let dialog = MessageAlert {
        title: "Result",
        text: &format!("{}: {:?}", &name, &value),
        typ: MessageType::Info,
    };
    dialog.show().unwrap();
}

fn main() {
    let dialog = MessageConfirm {
        title: "Tour",
        text: "Let's begin the tour!",
        typ: MessageType::Info,
    };
    let result = dialog.show().unwrap();
    if !result {
        return;
    }
    echo("MessageConfirm", &result);

    let dialog = OpenSingleFile {
        dir: None,
        filter: None,
    };
    let result = dialog.show().unwrap();
    echo("OpenSingleFile", &result);

    let dialog = OpenMultipleFile {
        dir: None,
        filter: None,
    };
    let result = dialog.show().unwrap();
    echo("OpenMultipleFile", &result);

    let dialog = OpenSingleDir { dir: None };
    let result = dialog.show().unwrap();
    echo("OpenSingleDir", &result);

    let dialog = MessageAlert {
        title: "End",
        text: "That's the end!",
        typ: MessageType::Info,
    };
    dialog.show().unwrap();
}
