use std::ffi::OsStr;
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};

use super::backend::{Backend, BackendKind};
use crate::dialog::{
    DialogImpl, Filter, OpenMultipleFile, OpenSingleDir, OpenSingleFile, SaveSingleFile,
};
use crate::utils::resolve_tilde;
use crate::Result;

impl OpenSingleFile {
    fn create(&self) -> Result<Backend> {
        let target = get_target(self.location.as_deref(), self.filename.as_deref());

        let params = Params {
            target: target.as_deref(),
            filters: &self.filters,
            multiple: false,
            dir: false,
            save: false,
            title: &self.title,
            owner: unsafe { self.owner.as_x11() },
        };

        create_backend(params)
    }
}

impl DialogImpl for OpenSingleFile {
    fn show(self) -> Result<Self::Output> {
        let backend = self.create()?;
        let output = backend.exec()?;
        Ok(output.map(parse_output))
    }

    #[cfg(feature = "async")]
    async fn spawn(self) -> Result<Self::Output> {
        let backend = self.create()?;
        let output = backend.spawn().await?;
        Ok(output.map(parse_output))
    }
}

impl OpenMultipleFile {
    fn create(&self) -> Result<Backend> {
        let target = get_target(self.location.as_deref(), self.filename.as_deref());

        let params = Params {
            target: target.as_deref(),
            filters: &self.filters,
            multiple: true,
            dir: false,
            save: false,
            title: &self.title,
            owner: unsafe { self.owner.as_x11() },
        };

        create_backend(params)
    }
}

impl DialogImpl for OpenMultipleFile {
    fn show(self) -> Result<Self::Output> {
        let backend = self.create()?;
        let output = backend.exec()?;

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
    async fn spawn(self) -> Result<Self::Output> {
        let backend = self.create()?;
        let output = backend.spawn().await?;

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
    fn create(&self) -> Result<Backend> {
        let target = get_target(self.location.as_deref(), self.filename.as_deref());

        let params = Params {
            target: target.as_deref(),
            filters: &[],
            multiple: false,
            dir: true,
            save: false,
            title: &self.title,
            owner: unsafe { self.owner.as_x11() },
        };

        create_backend(params)
    }
}

impl DialogImpl for OpenSingleDir {
    fn show(self) -> Result<Self::Output> {
        let backend = self.create()?;
        let output = backend.exec()?;
        Ok(output.map(parse_output))
    }

    #[cfg(feature = "async")]
    async fn spawn(self) -> Result<Self::Output> {
        let backend = self.create()?;
        let output = backend.spawn().await?;
        Ok(output.map(parse_output))
    }
}

impl SaveSingleFile {
    fn create(&self, target: &Option<PathBuf>) -> Result<Backend> {
        let params = Params {
            target: target.as_deref(),
            filters: &self.filters,
            multiple: false,
            dir: false,
            save: true,
            title: &self.title,
            owner: unsafe { self.owner.as_x11() },
        };

        create_backend(params)
    }

    fn warn(&self, path: &Option<PathBuf>) -> Result<Backend> {
        let message = match path.as_deref().and_then(Path::extension) {
            None => String::from("Unrecognized file type. Please try again."),
            Some(ext) => {
                let ext = ext.display();
                format!("Unrecognized file type: {ext}. Please try again.")
            }
        };

        let mut backend = Backend::new()?;
        match backend.kind {
            BackendKind::KDialog => {
                backend.command.args([
                    "--msgbox",
                    &message,
                    "--title",
                    "Warning",
                    "--icon=dialog-warning",
                ]);
            }
            BackendKind::Zenity => {
                backend.command.args([
                    "--width=400",
                    "--warning",
                    "--title",
                    "Warning",
                    "--text",
                    &message,
                ]);
            }
        };

        Ok(backend)
    }

    fn accepts(&self, path: &Option<PathBuf>) -> bool {
        if self.filters.is_empty() {
            return true;
        }

        let Some(path) = path else {
            return true;
        };

        if let Some(ext) = path.extension() {
            for filter in &self.filters {
                for accepting in &filter.extensions {
                    if OsStr::new(accepting) == ext {
                        return true;
                    }
                }
            }
        }

        false
    }
}

impl DialogImpl for SaveSingleFile {
    fn show(self) -> Result<Self::Output> {
        let mut target = get_target(self.location.as_deref(), self.filename.as_deref());

        loop {
            let backend = self.create(&target)?;
            let output = backend.exec()?;

            let path = output.map(parse_output);
            if self.accepts(&path) {
                break Ok(path);
            }

            self.warn(&path)?.exec()?;

            target = path;
        }
    }

    #[cfg(feature = "async")]
    async fn spawn(self) -> Result<Self::Output> {
        let mut target = get_target(self.location.as_deref(), self.filename.as_deref());

        loop {
            let backend = self.create(&target)?;
            let output = backend.spawn().await?;

            let path = output.map(parse_output);
            if self.accepts(&path) {
                break Ok(path);
            }

            self.warn(&path)?.spawn().await?;

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

struct Params<'a> {
    target: Option<&'a Path>,
    filters: &'a [Filter],
    multiple: bool,
    dir: bool,
    save: bool,
    title: &'a str,
    owner: Option<u64>,
}

fn create_backend(params: Params) -> Result<Backend> {
    let mut backend = Backend::new()?;
    match backend.kind {
        BackendKind::KDialog => init_kdialog(&mut backend, params),
        BackendKind::Zenity => init_zenity(&mut backend, params),
    };

    Ok(backend)
}

fn init_kdialog(backend: &mut Backend, params: Params) {
    if let Some(owner) = params.owner {
        backend.command.arg(format!("--attach=0x{:x}", owner));
    }

    match (params.dir, params.save) {
        (false, false) => backend.command.arg("--getopenfilename"),
        (false, true) => backend.command.arg("--getsavefilename"),
        (true, false) => backend.command.arg("--getexistingdirectory"),
        (true, true) => unreachable!(),
    };

    backend.command.arg("--title");
    backend.command.arg(params.title);

    match params.target {
        Some(path) => backend.command.arg(path),
        None => backend.command.arg(""),
    };

    if params.multiple {
        backend.command.args(["--multiple", "--separate-output"]);
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

        backend.command.arg(filters.join("\n"));
    }
}

fn init_zenity(backend: &mut Backend, params: Params) {
    backend.command.arg("--file-selection");

    backend.command.arg("--title");
    backend.command.arg(params.title);

    if params.dir {
        backend.command.arg("--directory");
    }

    if params.save {
        backend.command.arg("--save");

        // `--confirm-overwrite` was removed at zenity 3.91.0
        // https://gitlab.gnome.org/GNOME/zenity/-/issues/55
        if matches!(backend.version(), Some(v) if v < (3, 91, 0)) {
            backend.command.arg("--confirm-overwrite");
        }
    };

    if params.multiple {
        backend.command.args(["--multiple", "--separator", "\n"]);
    }

    if let Some(path) = params.target {
        backend.command.arg("--filename");
        backend.command.arg(path);
    }

    if !params.filters.is_empty() {
        for filter in params.filters {
            // let extensions: Vec<String> = filter
            //     .extensions
            //     .iter()
            //     .map(|s| format!("*.{}", s))
            //     .collect();
            // let extensions = extensions.join(" ");

            // backend.command.arg("--file-filter");
            // backend.command.arg(format!(
            //     "{} ({}) | {}",
            //     filter.description, extensions, extensions
            // ));

            // TODO: test this
            let formatted = filter.format("{desc} ({types}) | {types}", "*.{ext}", " ");
            backend.command.arg("--file-filter");
            backend.command.arg(formatted);
        }
    }
}
