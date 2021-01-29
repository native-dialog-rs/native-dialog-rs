# native-dialog

[![Crates.io](https://img.shields.io/crates/v/native-dialog.svg)](https://crates.io/crates/native-dialog)
[![Docs.rs](https://docs.rs/native-dialog/badge.svg)](https://docs.rs/native-dialog)
[![License](https://img.shields.io/crates/l/native-dialog.svg)](LICENSE)

A library to display file choosers and message boxes. Supports GNU/Linux, BSD Unix, macOS and Windows.

## Installation

```
cargo add native-dialog
```

## Usage

```rust
use native_dialog::{FileDialog, MessageDialog, MessageType};

fn main() {
    let path = FileDialog::new()
        .set_location("~/Desktop")
        .add_filter("PNG Image", &["png"])
        .add_filter("JPEG Image", &["jpg", "jpeg"])
        .show_open_single_file()
        .unwrap();

    let path = match path {
        Some(path) => path,
        None => return,
    };

    let yes = MessageDialog::new()
        .set_type(MessageType::Info)
        .set_title("Do you want to open the file?")
        .set_text(&format!("{:#?}", path))
        .show_confirm()
        .unwrap();

    if yes {
        do_something(path);
    }
}
```

## Misc

#### Why the dialogs look ugly/blurry on Windows?

Turn on crate features or embed manifests into the `.exe` to enable visual styling and dpi awareness for your program. Check out [examples/windows_manifest](examples/windows_manifest) and [examples/windows_features](examples/windows_features) for example.

#### Why the program crashed when opening a dialog on macOS?

The UI framework of macOS (Cocoa) has a limitation that all UI operations must be performed on the main thread.
