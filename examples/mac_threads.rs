use dispatch2::run_on_main;
use native_dialog::DialogBuilder;
use objc2::MainThreadMarker;
use objc2_app_kit::{NSApp, NSApplication};

fn main() {
    std::thread::spawn(|| {
        println!("Hello from another thread!");
        let path = DialogBuilder::file().open_single_file().show().unwrap();
        dbg!(path);

        run_on_main(|mtm| {
            println!("Stopping the application...");
            NSApp(mtm).stop(None);
        });
    });

    println!("Running the application...");
    NSApplication::main(MainThreadMarker::new().unwrap())
}
