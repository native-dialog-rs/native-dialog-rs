# native-dialog

[![Crates.io](https://img.shields.io/crates/v/native-dialog.svg)](https://crates.io/crates/iced)
[![License](https://img.shields.io/crates/l/native-dialog.svg)](https://github.com/hecrj/iced/blob/master/LICENSE)

A library to display file choosers and message boxes. Supports GNU/Linux, macOS and Windows.

## Installation

```
cargo add native-dialog
```

## Usage

```rust
use native_dialog::*;

let dialog = OpenMultipleFile {
    dir: None,
    filter: None,
};
let result = dialog.show().unwrap();

let message = format!("{:?}", result);

let dialog = MessageConfirm {
    title: "Do you want to open these files?",
    text: &message,
    typ: MessageType::Info,
};
let result = dialog.show().unwrap();

assert_eq!(result, true);
```

## Misc

#### Why the dialogs look ugly/blurry on Windows?

Embed manifests into the `.exe` to enable visual styling and dpi awareness for your program. Check out [examples/windows_manifest](examples/windows_manifest) for example.
