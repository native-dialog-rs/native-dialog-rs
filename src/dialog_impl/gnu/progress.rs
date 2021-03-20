use std::cell::RefCell;
use std::io::{ErrorKind, Write};
use std::process::{Child, ChildStdin, Command, Stdio};

use crate::{Error, ProgressDialog, ProgressHandle, Result};
use crate::dialog::DialogImpl;

use super::{escape_pango_entities, should_use, UseCommand};

impl DialogImpl for ProgressDialog<'_> {
    fn show(&mut self) -> Result<Self::Output> {
        let command = should_use().ok_or(Error::NoImplementation)?;

        match command {
            UseCommand::KDialog(c) => call_kdialog(c, self),
            UseCommand::Zenity(c) => call_zenity(c, self)
        }
    }
}

struct KdialogProgressHandle {
    dbus_ref: String,
}

impl ProgressHandle for KdialogProgressHandle {
    fn set_progress(&mut self, percent: f32) -> Result<()> {
        let status = Command::new("qdbus")
            .arg(&self.dbus_ref)
            .arg("Set")
            .arg("\"\"")
            .arg("value")
            .arg(format!("{}", percent as i32))
            .status()?;

        if status.success() {
            Ok(())
        } else {
            Err(Error::ImplementationError("Failed to run qdbus".into()))
        }
    }

    fn set_text(&mut self, text: &str) -> Result<()> {
        let status = Command::new("qdbus")
            .arg(&self.dbus_ref)
            .arg("setLabelText")
            .arg(text)
            .status()?;

        if status.success() {
            Ok(())
        } else {
            Err(Error::ImplementationError("Failed to run qdbus".into()))
        }
    }

    fn check_cancelled(&mut self) -> Result<bool> {
        let output = Command::new("qdbus")
            .arg(&self.dbus_ref)
            .arg("wasCancelled")
            .output()?;

        let text = String::from_utf8(output.stdout)?;
        Ok(text == "true")
    }

    fn close(&mut self) -> Result<()> {
        let status = Command::new("qdbus")
            .arg(&self.dbus_ref)
            .arg("close")
            .status()?;

        if status.success() {
            Ok(())
        } else {
            Err(Error::ImplementationError("Failed to run qdbus".into()))
        }
    }
}

fn call_kdialog(mut command: Command, settings: &ProgressDialog) -> Result<Box<RefCell<dyn ProgressHandle>>> {
    command.arg("--progressbar");
    command.arg(settings.text);
    command.arg("100");

    command.arg("--title");
    command.arg(settings.title);

    let output = command.output()?;
    let dbus_ref = String::from_utf8(output.stdout)?;
    let handle = KdialogProgressHandle { dbus_ref };

    Ok(Box::new(RefCell::new(handle)))
}

struct ZenityProgressHandle {
    child: Child,
    stdin: ChildStdin,
}

impl ZenityProgressHandle {
    fn new(mut child: Child) -> Self {
        ZenityProgressHandle {
            stdin: child.stdin.take().unwrap(),
            child
        }
    }
}

impl ProgressHandle for ZenityProgressHandle {
    fn set_progress(&mut self, percent: f32) -> Result<()> {
        if percent < 0.0 || percent > 100.0 {
            return Err(Error::InvalidPercentage(percent))
        }

        self.stdin.write(format!("{}\n", percent).as_bytes())?;
        Ok(())
    }

    fn set_text(&mut self, text: &str) -> Result<()> {
        self.stdin.write(format!("# {}\n", text).as_bytes())?;
        Ok(())
    }

    fn check_cancelled(&mut self) -> Result<bool> {
        self.child.try_wait().map(|opt| {
            match opt {
                None => false,
                Some(status) => !status.success()
            }
        }).map_err(|e| Error::IoFailure(e))
    }

    fn close(&mut self) -> Result<()> {
        match self.child.kill() {
            Ok(_) => Ok(()),
            Err(err) => {
                if err.kind() == ErrorKind::InvalidInput {
                    // Process is already dead, ignore
                    Ok(())
                } else {
                    Err(Error::IoFailure(err))
                }
            }
        }
    }
}

fn call_zenity(mut command: Command, settings: &ProgressDialog) -> Result<Box<RefCell<dyn ProgressHandle>>> {
    command.arg("--progress");

    command.arg("--title");
    command.arg(settings.title);

    command.arg("--text");
    command.arg(escape_pango_entities(settings.text));

    command.stdin(Stdio::piped());

    let child = command.spawn()?;
    let handle = ZenityProgressHandle::new(child);

    Ok(Box::new(RefCell::new(handle)))
}
