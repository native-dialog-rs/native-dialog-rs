#[cfg(test)]

use crate::dialog_impl::gnu::message;

#[test]
fn test_kdialog_version() {
    let version = message::get_kdialog_version().unwrap();
    println!("{}, {}, {}", version.0, version.1, version.2);
}