use native_dialog::*;

fn main() {
    let dialog = OpenSingleDir { dir: None };
    let result = dialog.show();
    println!("{:?}", result);
}
