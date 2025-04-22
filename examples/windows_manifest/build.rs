#[cfg(windows)]
fn main() {
    use winresource::WindowsResource;

    let mut res = WindowsResource::new();
    res.set_manifest_file("manifest.xml");
    res.compile().unwrap();
}

#[cfg(not(windows))]
fn main() {}
