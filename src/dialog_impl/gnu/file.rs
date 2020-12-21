use super::{should_use, Error, UseCommand};
use crate::dialog::{DialogImpl, OpenMultipleFile, OpenSingleDir, OpenSingleFile, SaveSingleFile};
use crate::util::resolve_tilde;
use crate::{Filter, Result};
use std::ffi::OsStr;
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::process::Command;

impl DialogImpl for OpenSingleFile<'_> {
    fn show(&mut self) -> Result<Self::Output> {
        let target_path = get_target_path(self.location, self.filename);
        let params = Params {
            path: target_path.as_deref(),
            filters: &self.filters,
            multiple: false,
            dir: false,
            save: false,
        };

        let command = should_use().ok_or(Error::NoImplementation)?;
        let output = match command {
            UseCommand::KDialog(cmd) => call_kdialog(cmd, params),
            UseCommand::Zenity(cmd) => call_zenity(cmd, params),
        }?;

        Ok(output.as_deref().map(trim_newlines).map(to_path_buf))
    }
}

impl DialogImpl for OpenMultipleFile<'_> {
    fn show(&mut self) -> Result<Self::Output> {
        let target_path = get_target_path(self.location, self.filename);
        let params = Params {
            path: target_path.as_deref(),
            filters: &self.filters,
            multiple: true,
            dir: false,
            save: false,
        };

        let command = should_use().ok_or(Error::NoImplementation)?;
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
        let target_path = get_target_path(self.location, self.filename);
        let params = Params {
            path: target_path.as_deref(),
            filters: &[],
            multiple: false,
            dir: true,
            save: false,
        };

        let command = should_use().ok_or(Error::NoImplementation)?;
        let output = match command {
            UseCommand::KDialog(cmd) => call_kdialog(cmd, params),
            UseCommand::Zenity(cmd) => call_zenity(cmd, params),
        }?;

        Ok(output.as_deref().map(trim_newlines).map(to_path_buf))
    }
}

impl DialogImpl for SaveSingleFile<'_> {
    fn show(&mut self) -> Result<Self::Output> {
        let allowed_extensions = get_all_allowed_extension(&self.filters);

        let mut target_path = get_target_path(self.location, self.filename);
        loop {
            let params = Params {
                path: target_path.as_deref(),
                filters: &self.filters,
                multiple: false,
                dir: false,
                save: true,
            };

            let command = should_use().ok_or(Error::NoImplementation)?;
            let output = match command {
                UseCommand::KDialog(cmd) => call_kdialog(cmd, params),
                UseCommand::Zenity(cmd) => call_zenity(cmd, params),
            }?;

            let path = output.as_deref().map(trim_newlines).map(to_path_buf);
            if allowed_extensions.is_empty() {
                break Ok(path);
            }

            if let Some(path) = path {
                if let Some(ext) = path.extension() {
                    if allowed_extensions.contains(&ext) {
                        // Filename has an extension and it is valid
                        break Ok(Some(path));
                    }
                }

                let message = get_warn_extension_message(&path);

                let command = should_use().ok_or(Error::NoImplementation)?;
                match command {
                    UseCommand::KDialog(cmd) => call_kdialog_warn_extension(cmd, &message),
                    UseCommand::Zenity(cmd) => call_zenity_warn_extension(cmd, &message),
                }?;

                target_path = Some(path);

                continue;
            } else {
                break Ok(None);
            }
        }
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

fn get_target_path(location: Option<&Path>, filename: Option<&str>) -> Option<PathBuf> {
    match (location.and_then(resolve_tilde), filename) {
        (Some(location), Some(filename)) => Some(location.join(filename)),
        (Some(location), None) => Some(location.join("Untitled")),
        (None, Some(filename)) => Some(PathBuf::from(filename)),
        (None, None) => None,
    }
}

fn get_all_allowed_extension<'a>(filters: &'a [Filter<'a>]) -> Vec<&'a OsStr> {
    filters
        .iter()
        .flat_map(|filter| filter.extensions)
        .map(OsStr::new)
        .collect()
}

struct Params<'a> {
    path: Option<&'a Path>,
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

    match params.path {
        Some(path) => command.arg(path),
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

    match params.path {
        Some(path) => command.arg(path),
        None => command.arg(""),
    };

    if !params.filters.is_empty() {
        for filter in params.filters {
            let extensions: Vec<String> = filter
                .extensions
                .iter()
                .map(|s| format!("*.{}", s))
                .collect();
            let extensions = extensions.join(" ");

            command.arg("--file-filter");
            command.arg(format!(
                "{} ({}) | {}",
                filter.description, extensions, extensions
            ));
        }
    }

    let output = command.output()?;

    match output.status.code() {
        Some(0) => Ok(Some(output.stdout)),
        Some(1) => Ok(None),
        _ => Err(Error::UnexpectedOutput("zenity")),
    }
}

fn call_kdialog_warn_extension(mut command: Command, message: &str) -> Result<()> {
    command.args(&[
        "--msgbox",
        message,
        "--title",
        "Incorrect file extension",
        "--icon=dialog-warning",
    ]);

    command.output()?;
    Ok(())
}

fn call_zenity_warn_extension(mut command: Command, message: &str) -> Result<()> {
    command.args(&[
        "--width=400",
        "--warning",
        "--title",
        "Incorrect file extension",
        "--text",
        message,
    ]);

    command.output()?;
    Ok(())
}

fn get_warn_extension_message(path: &Path) -> String {
    match path.extension() {
        None => String::from(
            "You haven't specify a file extension in the filename. Please check it again.",
        ),
        Some(ext) => {
            format!(
                "We could not recognize the file extension \".{}\" you've input. Please check it again.",
                ext.to_string_lossy()
            )
        }
    }
}
