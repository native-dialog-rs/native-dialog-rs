use native_dialog::*;

fn main() {
    let dialog = OpenSingleFile {
        dir: None,
        filter: None,
    };
    let result = dialog.show();

    let message = format!("Shit is on fire!\n\n{:?}", result);

    let dialog = MessageConfirm {
        title: "What is happening?",
        text: &message,
        typ: MessageType::Info,
    };
    let result = dialog.show();

    println!("{:?}", result);
}
