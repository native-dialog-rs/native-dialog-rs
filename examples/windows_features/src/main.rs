use native_dialog::{DialogBuilder, MessageLevel};

fn main() {
    let path = DialogBuilder::file().open_single_file().show();

    let result = DialogBuilder::message()
        .set_title("What is happening?")
        .set_text(format!("Shit is on fire!\n\n{:?}", path))
        .set_level(MessageLevel::Warning)
        .confirm()
        .show();

    println!("{:?}", result);
}
