# native-dialog

[![Crates.io](https://img.shields.io/crates/v/native-dialog.svg)](https://crates.io/crates/native-dialog)
[![License](https://img.shields.io/crates/l/native-dialog.svg)](LICENSE)

A library to display file choosers and message boxes. Supports GNU/Linux, macOS and Windows.

## Installation

```
cargo add native-dialog
```

## Usage

```rust
use native_dialog::{OpenMultipleFile, MessageConfirm, MessageType};

fn main() {
    let result = OpenMultipleFile::new()
        .location("~/Desktop")
        .filter("PNG Image", &["png"])
        .filter("JPEG Image", &["jpg", "jpeg"])
        .show()
        .unwrap();

    let message = format!("{:#?}", result);

    let result = MessageConfirm::new()
        .typ(MessageType::Info)
        .title("Do you want to open these files?")
        .text(&message)
        .show()
        .unwrap();

    assert_eq!(result, true);
}
```

## Misc

#### Why the dialogs look ugly/blurry on Windows?

Turn on crate features or embed manifests into the `.exe` to enable visual styling and dpi awareness for your program. Check out [examples/windows_manifest](examples/windows_manifest) and [examples/windows_features](examples/windows_features) for example.

#### Why the program crashed when opening a dialog on macOS?

The API of macOS has a limitation that all UI operations must be performed on the main thread.
