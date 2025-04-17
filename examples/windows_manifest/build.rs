use winresource::WindowsResource;

fn main() {
    let mut res = WindowsResource::new();
    res.set_manifest_file("manifest.xml");
    res.compile().unwrap();
}
