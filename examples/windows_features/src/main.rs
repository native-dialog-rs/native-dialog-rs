use native_dialog::*;

fn main() {
    let path = OpenSingleFile::new().show();

    let result = MessageConfirm::new()
        .title("What is happening?")
        .text(&format!("Shit is on fire!\n\n{:?}", path))
        .typ(MessageType::Warning)
        .show();

    println!("{:?}", result);
}
