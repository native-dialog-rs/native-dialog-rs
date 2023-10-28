#[cfg(target_os = "linux")]
#[test]
fn test_kdialog_version() {
    use crate::dialog_impl::gnu::message;
    let version = message::get_kdialog_version().unwrap();
    println!("{}, {}, {}", version.0, version.1, version.2);
}
