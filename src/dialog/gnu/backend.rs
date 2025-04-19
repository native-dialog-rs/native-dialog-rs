use std::process::Command;

use ascii::AsAsciiStr;

use super::version::Version;
use crate::{Error, Result};

pub enum BackendKind {
    KDialog,
    Zenity,
}

pub struct Backend {
    pub command: Command,
    pub kind: BackendKind,
}

impl Backend {
    pub fn new() -> Result<Backend> {
        let has_display = match std::env::var("DISPLAY") {
            Ok(display) => !display.is_empty(),
            _ => false,
        };

        let candidates = match std::env::var("XDG_CURRENT_DESKTOP").as_deref() {
            Ok("KDE") if has_display => [Self::new_kdialog, Self::new_zenity],
            _ => [Self::new_zenity, Self::new_kdialog],
        };

        for candidate in candidates {
            if let Some(command) = candidate() {
                return Ok(command);
            }
        }

        Err(Error::MissingDep)
    }

    fn new_kdialog() -> Option<Backend> {
        let path = which::which("kdialog").ok()?;
        let command = Command::new(path);

        Some(Self {
            command,
            kind: BackendKind::KDialog,
        })
    }

    fn new_zenity() -> Option<Backend> {
        let path = which::which("zenity").ok()?;
        let command = Command::new(path);

        Some(Self {
            command,
            kind: BackendKind::Zenity,
        })
    }

    pub fn version(&self) -> Option<Version> {
        let program = self.command.get_program();
        let output = Command::new(program).arg("--version").output().ok()?;
        let stdout = output.stdout.as_ascii_str().ok()?.to_string();
        stdout.split_whitespace().last().and_then(Version::parse)
    }

    pub fn exec(mut self) -> Result<Option<Vec<u8>>> {
        let program = self.command.get_program().to_os_string();

        let output = self.command.output()?;
        match output.status.code() {
            Some(0) => Ok(Some(output.stdout)),
            Some(_) => Ok(None),
            None => Err(Error::Killed(program)),
        }
    }

    #[cfg(feature = "async")]
    pub async fn spawn(self) -> Result<Option<Vec<u8>>> {
        let (send, recv) = futures_channel::oneshot::channel();

        std::thread::spawn(move || {
            let _ = send.send(self.exec());
        });

        recv.await.unwrap_or(Ok(None))
    }
}
