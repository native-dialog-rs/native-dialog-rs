use std::thread::sleep;
use std::time::Duration;

use native_dialog::{Error, ProgressDialog};

fn main() -> Result<(), Error> {
    let progress = ProgressDialog::new()
        .set_title("Progress Example")
        .set_text("Doing complicated things...")
        .show()?;

    let mut handle = progress.borrow_mut();
    handle.set_progress(20.0)?;

    sleep(Duration::from_secs(3));
    if handle.check_cancelled()? {
        eprintln!("Cancelled!");
        return Ok(());
    }

    handle.set_progress(80.0)?;
    handle.set_text("Almost there...")?;

    sleep(Duration::from_secs(2));
    handle.set_progress(100.0)?;

    sleep(Duration::from_secs(1));
    handle.close()?;

    Ok(())
}
