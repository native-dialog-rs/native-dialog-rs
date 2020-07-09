use super::{bytes_to_string, should_use, Error, UseCommand};
use crate::{
    r#impl::OpenDialogTarget, Dialog, OpenMultipleFile, OpenSingleDirectory, OpenSingleFile, Result,
};
use std::process::Command;

impl Dialog for OpenSingleFile<'_> {
    type Output = Option<String>;

    fn show(self) -> Result<Self::Output> {
        match should_use() {
            Some(UseCommand::KDialog(command)) => {
                dialog_implementation_kdialog(ImplementationParams {
                    command,
                    dir: self.dir,
                    filter: self.filter,
                    multiple: false,
                    target: OpenDialogTarget::File,
                })
            }
            Some(UseCommand::Zenity(command)) => {
                dialog_implementation_zenity(ImplementationParams {
                    command,
                    dir: self.dir,
                    filter: self.filter,
                    multiple: false,
                    target: OpenDialogTarget::File,
                })
            }
            None => Err(Error::NoImplementation),
        }
    }
}

impl Dialog for OpenMultipleFile<'_> {
    type Output = Vec<String>;

    fn show(self) -> Result<Self::Output> {
        let lf_separated = match should_use() {
            Some(UseCommand::KDialog(command)) => {
                dialog_implementation_kdialog(ImplementationParams {
                    command,
                    dir: self.dir,
                    filter: self.filter,
                    multiple: true,
                    target: OpenDialogTarget::File,
                })
            }
            Some(UseCommand::Zenity(command)) => {
                dialog_implementation_zenity(ImplementationParams {
                    command,
                    dir: self.dir,
                    filter: self.filter,
                    multiple: true,
                    target: OpenDialogTarget::File,
                })
            }
            None => Err(Error::NoImplementation),
        };

        match lf_separated {
            Ok(Some(s)) => Ok(s.split('\n').map(ToString::to_string).collect()),
            Ok(_) => Ok(vec![]),
            Err(e) => Err(e),
        }
    }
}

impl Dialog for OpenSingleDirectory<'_> {
    type Output = Option<String>;

    fn show(self) -> Result<Self::Output> {
        match should_use() {
            Some(UseCommand::KDialog(command)) => {
                dialog_implementation_kdialog(ImplementationParams {
                    command,
                    dir: self.dir,
                    filter: None,
                    multiple: false,
                    target: OpenDialogTarget::Directory,
                })
            }
            Some(UseCommand::Zenity(command)) => {
                dialog_implementation_zenity(ImplementationParams {
                    command,
                    dir: self.dir,
                    filter: None,
                    multiple: false,
                    target: OpenDialogTarget::Directory,
                })
            }
            None => Err(Error::NoImplementation),
        }
    }
}

struct ImplementationParams<'a> {
    command: Command,
    dir: Option<&'a str>,
    filter: Option<&'a [&'a str]>,
    multiple: bool,
    target: OpenDialogTarget,
}

fn dialog_implementation_kdialog(mut params: ImplementationParams) -> Result<Option<String>> {
    let command = &mut params.command;

    match params.target {
        OpenDialogTarget::File => command.arg("--getopenfilename"),
        OpenDialogTarget::Directory => command.arg("--getexistingdirectory"),
    };

    match params.dir {
        Some(dir) => command.arg(dir),
        None => command.arg(""),
    };

    if params.multiple {
        command.args(&["--multiple", "--separate-output"]);
    }

    if let Some(filter) = params.filter {
        let types: Vec<String> = filter.iter().map(|s| format!("*.{}", s)).collect();
        command.arg(types.join(" "));
    }

    let output = command.output()?;

    match output.status.code() {
        Some(0) => bytes_to_string(output.stdout).map(Some),
        Some(1) => Ok(None),
        _ => Err(Error::UnexpectedOutput("kdialog")),
    }
}

fn dialog_implementation_zenity(mut params: ImplementationParams) -> Result<Option<String>> {
    let command = &mut params.command;

    command.arg("--file-selection");

    if params.target == OpenDialogTarget::Directory {
        command.arg("--directory");
    }

    if params.multiple {
        command.args(&["--multiple", "--separator", "\n"]);
    }

    command.arg("--filename");

    match params.dir {
        Some(dir) => command.arg(dir),
        None => command.arg(""),
    };

    if let Some(filter) = params.filter {
        let types: Vec<String> = filter.iter().map(|s| format!("*.{}", s)).collect();
        command.arg("--file-filter");
        command.arg(types.join(" "));
    }

    let output = command.output()?;

    match output.status.code() {
        Some(0) => bytes_to_string(output.stdout).map(Some),
        Some(1) => Ok(None),
        _ => Err(Error::UnexpectedOutput("zenity")),
    }
}
