use super::{should_use, Error, UseCommand};
use crate::dialog::{DialogImpl, OpenMultipleFile, OpenSingleDir, OpenSingleFile, SaveSingleFile};
use crate::{Filter, Result};
use std::ffi::OsStr;
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::process::Command;

impl DialogImpl for OpenSingleFile<'_> {
    fn show(&mut self) -> Result<Self::Output> {
        let command = should_use().ok_or(Error::NoImplementation)?;

        let params = Params {
            location: self.location,
            filters: &self.filters,
            multiple: false,
            dir: false,
            save: false,
        };

        let output = match command {
            UseCommand::KDialog(cmd) => call_kdialog(cmd, params),
            UseCommand::Zenity(cmd) => call_zenity(cmd, params),
        }?;

        Ok(output.as_deref().map(trim_newlines).map(to_path_buf))
    }
}

impl DialogImpl for OpenMultipleFile<'_> {
    fn show(&mut self) -> Result<Self::Output> {
        let command = should_use().ok_or(Error::NoImplementation)?;

        let params = Params {
            location: self.location,
            filters: &self.filters,
            multiple: true,
            dir: false,
            save: false,
        };

        let output = match command {
            UseCommand::KDialog(cmd) => call_kdialog(cmd, params),
            UseCommand::Zenity(cmd) => call_zenity(cmd, params),
        }?;

        match output {
            Some(lf_separated) => {
                let paths = lf_separated
                    .split(|c| *c == b'\n')
                    .filter(|c| !c.is_empty())
                    .map(to_path_buf)
                    .collect();
                Ok(paths)
            }
            None => Ok(vec![]),
        }
    }
}

impl DialogImpl for OpenSingleDir<'_> {
    fn show(&mut self) -> Result<Self::Output> {
        let command = should_use().ok_or(Error::NoImplementation)?;

        let params = Params {
            location: self.location,
            filters: &[],
            multiple: false,
            dir: true,
            save: false,
        };

        let output = match command {
            UseCommand::KDialog(cmd) => call_kdialog(cmd, params),
            UseCommand::Zenity(cmd) => call_zenity(cmd, params),
        }?;

        Ok(output.as_deref().map(trim_newlines).map(to_path_buf))
    }
}

impl DialogImpl for SaveSingleFile<'_> {
    fn show(&mut self) -> Result<Self::Output> {
        let command = should_use().ok_or(Error::NoImplementation)?;

        let params = Params {
            location: self.location,
            filters: &self.filters,
            multiple: false,
            dir: false,
            save: true,
        };

        let output = match command {
            UseCommand::KDialog(cmd) => call_kdialog(cmd, params),
            UseCommand::Zenity(cmd) => call_zenity(cmd, params),
        }?;

        Ok(output.as_deref().map(trim_newlines).map(to_path_buf))
    }
}

fn trim_newlines(s: &[u8]) -> &[u8] {
    fn is_not_newline(c: &u8) -> bool {
        *c != b'\n'
    }

    if let Some(first) = s.iter().position(is_not_newline) {
        let last = s.iter().rposition(is_not_newline).unwrap();
        &s[first..last + 1]
    } else {
        &[]
    }
}

fn to_path_buf(buf: impl AsRef<[u8]>) -> PathBuf {
    PathBuf::from(OsStr::from_bytes(buf.as_ref()))
}

struct Params<'a> {
    location: Option<&'a Path>,
    filters: &'a [Filter<'a>],
    multiple: bool,
    dir: bool,
    save: bool,
}

fn call_kdialog(mut command: Command, params: Params) -> Result<Option<Vec<u8>>> {
    match (params.dir, params.save) {
        (false, false) => command.arg("--getopenfilename"),
        (false, true) => command.arg("--getsavefilename"),
        (true, false) => command.arg("--getexistingdirectory"),
        (true, true) => unreachable!("???"),
    };

    match params.location {
        Some(dir) => command.arg(dir),
        None => command.arg(""),
    };

    if params.multiple {
        command.args(&["--multiple", "--separate-output"]);
    }

    if !params.filters.is_empty() {
        let filters: Vec<String> = params
            .filters
            .iter()
            .map(|filter| {
                let extensions: Vec<String> = filter
                    .extensions
                    .iter()
                    .map(|s| format!("*.{}", s))
                    .collect();
                format!("{} ({})", filter.description, extensions.join(" "))
            })
            .collect();
        command.arg(filters.join("\n"));
    }

    let output = command.output()?;

    match output.status.code() {
        Some(0) => Ok(Some(output.stdout)),
        Some(1) => Ok(None),
        _ => Err(Error::UnexpectedOutput("kdialog")),
    }
}

fn call_zenity(mut command: Command, params: Params) -> Result<Option<Vec<u8>>> {
    command.arg("--file-selection");

    if params.dir {
        command.arg("--directory");
    }

    if params.save {
        command.args(&["--save", "--confirm-overwrite"]);
    };

    if params.multiple {
        command.args(&["--multiple", "--separator", "\n"]);
    }

    command.arg("--filename");

    match params.location {
        Some(dir) => command.arg(dir),
        None => command.arg(""),
    };

    if !params.filters.is_empty() {
        for filter in params.filters {
            let extensions: Vec<String> = filter
                .extensions
                .iter()
                .map(|s| format!("*.{}", s))
                .collect();

            command.arg("--file-filter");
            command.arg(format!("{} | {}", filter.description, extensions.join(" ")));
        }
    }

    let output = command.output()?;

    match output.status.code() {
        Some(0) => Ok(Some(output.stdout)),
        Some(1) => Ok(None),
        _ => Err(Error::UnexpectedOutput("zenity")),
    }
}
