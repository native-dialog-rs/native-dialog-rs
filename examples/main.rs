use native_dialog::*;

fn main() {
    let r = MessageConfirm {
        title: "What is happening?",
        text: "Shit is on fire!",
        typ: MessageType::Info,
    }
    .show();

    println!("{:?}", r);
}
