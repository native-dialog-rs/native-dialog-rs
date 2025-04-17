use std::collections::HashSet;
use std::ffi::OsStr;
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::process::Command;

use super::{execute_command, get_zenity_version, should_use, spawn_command, Error, UseCommand};
use crate::dialog::{
    DialogImpl, Filter, OpenMultipleFile, OpenSingleDir, OpenSingleFile, SaveSingleFile,
};
use crate::utils::resolve_tilde;
use crate::Result;

impl OpenSingleFile {
    fn create(&self) -> Result<Command> {
        let target = get_target(self.location.as_deref(), self.filename.as_deref());

        let params = Params {
            target: target.as_deref(),
            filters: &self.filters,
            multiple: false,
            dir: false,
            save: false,
            title: &self.title,
            attach: unsafe { self.owner.as_x11() },
        };

        let command = match should_use().ok_or(Error::ImplMissing)? {
            UseCommand::KDialog(cmd) => call_kdialog(cmd, params),
            UseCommand::Zenity(cmd) => call_zenity(cmd, params),
        };

        Ok(command)
    }
}

impl DialogImpl for OpenSingleFile {
    fn show(self) -> Result<Self::Output> {
        let command = self.create()?;
        let output = execute_command(command)?;
        Ok(output.map(parse_output))
    }

    #[cfg(feature = "async")]
    async fn spawn(self) -> crate::Result<Self::Output> {
        let command = self.create()?;
        let output = spawn_command(command).await?;
        Ok(output.map(parse_output))
    }
}

impl OpenMultipleFile {
    fn create(&self) -> Result<Command> {
        let target = get_target(self.location.as_deref(), self.filename.as_deref());

        let params = Params {
            target: target.as_deref(),
            filters: &self.filters,
            multiple: true,
            dir: false,
            save: false,
            title: &self.title,
            attach: unsafe { self.owner.as_x11() },
        };

        let command = match should_use().ok_or(Error::ImplMissing)? {
            UseCommand::KDialog(cmd) => call_kdialog(cmd, params),
            UseCommand::Zenity(cmd) => call_zenity(cmd, params),
        };

        Ok(command)
    }
}

impl DialogImpl for OpenMultipleFile {
    fn show(self) -> Result<Self::Output> {
        let command = self.create()?;
        let output = execute_command(command)?;

        let paths = match output {
            Some(output) => output
                .split(|c| *c == b'\n')
                .filter(|c| !c.is_empty())
                .map(parse_output)
                .collect(),
            None => vec![],
        };

        Ok(paths)
    }

    #[cfg(feature = "async")]
    async fn spawn(self) -> crate::Result<Self::Output> {
        let command = self.create()?;
        let output = spawn_command(command).await?;

        let paths = match output {
            Some(output) => output
                .split(|c| *c == b'\n')
                .filter(|c| !c.is_empty())
                .map(parse_output)
                .collect(),
            None => vec![],
        };

        Ok(paths)
    }
}

impl OpenSingleDir {
    fn create(&self) -> Result<Command> {
        let target = get_target(self.location.as_deref(), self.filename.as_deref());

        let params = Params {
            target: target.as_deref(),
            filters: &[],
            multiple: false,
            dir: true,
            save: false,
            title: &self.title,
            attach: unsafe { self.owner.as_x11() },
        };

        let command = match should_use().ok_or(Error::ImplMissing)? {
            UseCommand::KDialog(cmd) => call_kdialog(cmd, params),
            UseCommand::Zenity(cmd) => call_zenity(cmd, params),
        };

        Ok(command)
    }
}

impl DialogImpl for OpenSingleDir {
    fn show(self) -> Result<Self::Output> {
        let command = self.create()?;
        let output = execute_command(command)?;
        Ok(output.map(parse_output))
    }

    #[cfg(feature = "async")]
    async fn spawn(self) -> crate::Result<Self::Output> {
        let command = self.create()?;
        let output = spawn_command(command).await?;
        Ok(output.map(parse_output))
    }
}

impl SaveSingleFile {
    fn create(&self, target: &Option<PathBuf>) -> Result<Command> {
        let params = Params {
            target: target.as_deref(),
            filters: &self.filters,
            multiple: false,
            dir: false,
            save: true,
            title: &self.title,
            attach: unsafe { self.owner.as_x11() },
        };

        let command = match should_use().ok_or(Error::ImplMissing)? {
            UseCommand::KDialog(cmd) => call_kdialog(cmd, params),
            UseCommand::Zenity(cmd) => call_zenity(cmd, params),
        };

        Ok(command)
    }

    fn warn(&self, path: &Option<PathBuf>) -> Result<Command> {
        let message = match path.as_deref().and_then(Path::extension) {
            None => String::from("Unrecognized file type. Please try again."),
            Some(ext) => {
                let ext = ext.display();
                format!("Unrecognized file type: {ext}. Please try again.")
            }
        };

        let command = match should_use().ok_or(Error::ImplMissing)? {
            UseCommand::KDialog(cmd) => call_kdialog_warn(cmd, &message),
            UseCommand::Zenity(cmd) => call_zenity_warn(cmd, &message),
        };

        Ok(command)
    }

    fn accepts(&self, path: &Option<PathBuf>, extensions: &HashSet<&OsStr>) -> bool {
        if extensions.is_empty() {
            return true;
        }

        let Some(path) = path else {
            return true;
        };

        if let Some(ext) = path.extension() {
            if extensions.contains(&ext) {
                return true;
            }
        }

        false
    }
}

impl DialogImpl for SaveSingleFile {
    fn show(self) -> Result<Self::Output> {
        let extensions = get_extensions(&self.filters);

        let mut target = get_target(self.location.as_deref(), self.filename.as_deref());
        loop {
            let command = self.create(&target)?;
            let output = execute_command(command)?;

            let path = output.map(parse_output);
            if self.accepts(&path, &extensions) {
                break Ok(path);
            }

            let warn = self.warn(&path)?;
            execute_command(warn)?;

            target = path;
        }
    }

    #[cfg(feature = "async")]
    async fn spawn(self) -> crate::Result<Self::Output> {
        let extensions = get_extensions(&self.filters);

        let mut target = get_target(self.location.as_deref(), self.filename.as_deref());
        loop {
            let command = self.create(&target)?;
            let output = spawn_command(command).await?;

            let path = output.map(parse_output);
            if self.accepts(&path, &extensions) {
                break Ok(path);
            }

            let warn = self.warn(&path)?;
            spawn_command(warn).await?;

            target = path;
        }
    }
}

fn parse_output(buf: impl AsRef<[u8]>) -> PathBuf {
    let bytes = buf.as_ref().trim_ascii();
    PathBuf::from(OsStr::from_bytes(bytes))
}

fn get_target(location: Option<&Path>, filename: Option<&str>) -> Option<PathBuf> {
    match (location.and_then(resolve_tilde), filename) {
        (Some(location), Some(filename)) => Some(location.join(filename)),
        (Some(location), None) => Some(location.join("Untitled")),
        (None, Some(filename)) => Some(PathBuf::from(filename)),
        (None, None) => None,
    }
}

fn get_extensions(filters: &[Filter]) -> HashSet<&OsStr> {
    filters
        .iter()
        .flat_map(|filter| &filter.extensions)
        .map(OsStr::new)
        .collect()
}

struct Params<'a> {
    target: Option<&'a Path>,
    filters: &'a [Filter],
    multiple: bool,
    dir: bool,
    save: bool,
    title: &'a str,
    attach: Option<u64>,
}

fn call_kdialog(mut command: Command, params: Params) -> Command {
    if let Some(attach) = params.attach {
        command.arg("--attach");
        command.arg(attach.to_string());
    }

    match (params.dir, params.save) {
        (false, false) => command.arg("--getopenfilename"),
        (false, true) => command.arg("--getsavefilename"),
        (true, false) => command.arg("--getexistingdirectory"),
        (true, true) => unreachable!("???"),
    };

    command.arg("--title");
    command.arg(params.title);

    match params.target {
        Some(path) => command.arg(path),
        None => command.arg(""),
    };

    if params.multiple {
        command.args(["--multiple", "--separate-output"]);
    }

    if !params.filters.is_empty() {
        let filters: Vec<String> = params
            .filters
            .iter()
            .map(|filter| {
                // let extensions: Vec<String> = filter
                //     .extensions
                //     .iter()
                //     .map(|s| format!("*.{}", s))
                //     .collect();
                // format!("{} ({})", filter.description, extensions.join(" "))

                // TODO: test this
                filter.format("{desc} ({types})", "*.{ext}", " ")
            })
            .collect();

        command.arg(filters.join("\n"));
    }

    command
}

fn call_zenity(mut command: Command, params: Params) -> Command {
    if let Some(attach) = params.attach {
        command.arg("--attach");
        command.arg(attach.to_string());
    }

    command.arg("--file-selection");

    command.arg("--title");
    command.arg(params.title);

    if params.dir {
        command.arg("--directory");
    }

    if params.save {
        command.arg("--save");

        // `--confirm-overwrite` was removed at zenity 3.91.0
        // https://gitlab.gnome.org/GNOME/zenity/-/issues/55
        if matches!(get_zenity_version(), Some(v) if v < (3, 91, 0)) {
            command.arg("--confirm-overwrite");
        }
    };

    if params.multiple {
        command.args(["--multiple", "--separator", "\n"]);
    }

    if let Some(path) = params.target {
        command.arg("--filename");
        command.arg(path);
    }

    if !params.filters.is_empty() {
        for filter in params.filters {
            // let extensions: Vec<String> = filter
            //     .extensions
            //     .iter()
            //     .map(|s| format!("*.{}", s))
            //     .collect();
            // let extensions = extensions.join(" ");

            command.arg("--file-filter");
            // command.arg(format!(
            //     "{} ({}) | {}",
            //     filter.description, extensions, extensions
            // ));

            // TODO: test this
            command.arg(filter.format("{desc} ({types}) | {types}", "*.{ext}", " "));
        }
    }

    command
}

fn call_kdialog_warn(mut command: Command, message: &str) -> Command {
    command.args([
        "--msgbox",
        message,
        "--title",
        "Warning",
        "--icon=dialog-warning",
    ]);

    command
}

fn call_zenity_warn(mut command: Command, message: &str) -> Command {
    command.args([
        "--width=400",
        "--warning",
        "--title",
        "Warning",
        "--text",
        message,
    ]);

    command
}
