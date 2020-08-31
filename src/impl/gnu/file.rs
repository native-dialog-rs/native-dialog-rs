use super::{should_use, Error, UseCommand};
use crate::r#impl::DialogImpl;
use crate::{Filter, OpenMultipleFile, OpenSingleDir, OpenSingleFile, Result};
use std::ffi::OsStr;
use std::os::unix::ffi::OsStrExt;
use std::path::PathBuf;
use std::process::Command;

impl DialogImpl for OpenSingleFile<'_> {
    type Output = Option<PathBuf>;

    fn show(&mut self) -> Result<Self::Output> {
        let output = match should_use() {
            Some(UseCommand::KDialog(command)) => {
                dialog_implementation_kdialog(ImplementationParams {
                    command,
                    location: self.location,
                    filters: &self.filters,
                    multiple: false,
                    open_dir: false,
                })
            }
            Some(UseCommand::Zenity(command)) => {
                dialog_implementation_zenity(ImplementationParams {
                    command,
                    location: self.location,
                    filters: &self.filters,
                    multiple: false,
                    open_dir: false,
                })
            }
            None => Err(Error::NoImplementation),
        }?;

        Ok(output.as_deref().map(trim_newlines).map(to_path_buf))
    }
}

impl DialogImpl for OpenMultipleFile<'_> {
    type Output = Vec<PathBuf>;

    fn show(&mut self) -> Result<Self::Output> {
        let output = match should_use() {
            Some(UseCommand::KDialog(command)) => {
                dialog_implementation_kdialog(ImplementationParams {
                    command,
                    location: self.location,
                    filters: &self.filters,
                    multiple: true,
                    open_dir: false,
                })
            }
            Some(UseCommand::Zenity(command)) => {
                dialog_implementation_zenity(ImplementationParams {
                    command,
                    location: self.location,
                    filters: &self.filters,
                    multiple: true,
                    open_dir: false,
                })
            }
            None => Err(Error::NoImplementation),
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
    type Output = Option<PathBuf>;

    fn show(&mut self) -> Result<Self::Output> {
        let output = match should_use() {
            Some(UseCommand::KDialog(command)) => {
                dialog_implementation_kdialog(ImplementationParams {
                    command,
                    location: self.location,
                    filters: &[],
                    multiple: false,
                    open_dir: true,
                })
            }
            Some(UseCommand::Zenity(command)) => {
                dialog_implementation_zenity(ImplementationParams {
                    command,
                    location: self.location,
                    filters: &[],
                    multiple: false,
                    open_dir: true,
                })
            }
            None => Err(Error::NoImplementation),
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

struct ImplementationParams<'a> {
    command: Command,
    location: Option<&'a str>,
    filters: &'a [Filter<'a>],
    multiple: bool,
    open_dir: bool,
}

fn dialog_implementation_kdialog(mut params: ImplementationParams) -> Result<Option<Vec<u8>>> {
    let command = &mut params.command;

    match params.open_dir {
        true => command.arg("--getopenfilename"),
        false => command.arg("--getexistingdirectory"),
    };

    match params.location {
        Some(dir) => command.arg(dir),
        None => command.arg(""),
    };

    if params.multiple {
        command.args(&["--multiple", "--separate-output"]);
    }

    if !params.filters.is_empty() {
        let types: Vec<String> = params
            .filters
            .iter()
            .map(|filter| format!("{} ({})", filter.description, filter.extensions.join(" ")))
            .collect();
        command.arg(types.join("\n"));
    }

    let output = command.output()?;

    match output.status.code() {
        Some(0) => Ok(Some(output.stdout)),
        Some(1) => Ok(None),
        _ => Err(Error::UnexpectedOutput("kdialog")),
    }
}

fn dialog_implementation_zenity(mut params: ImplementationParams) -> Result<Option<Vec<u8>>> {
    let command = &mut params.command;

    command.arg("--file-selection");

    if params.open_dir {
        command.arg("--directory");
    }

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
