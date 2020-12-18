use std::path::Path;

pub struct Filter<'a> {
    #[cfg_attr(target_os = "macos", allow(dead_code))]
    pub(crate) description: &'a str,
    pub(crate) extensions: &'a [&'a str],
}

pub struct OpenSingleFile<'a> {
    pub(crate) location: Option<&'a Path>,
    pub(crate) filters: Vec<Filter<'a>>,
}

impl<'a> OpenSingleFile<'a> {
    pub fn new() -> Self {
        OpenSingleFile {
            location: None,
            filters: vec![],
        }
    }

    pub fn location<P: AsRef<Path> + ?Sized>(&mut self, path: &'a P) -> &mut Self {
        self.location = Some(path.as_ref());
        self
    }

    pub fn filter(&mut self, description: &'a str, extensions: &'a [&'a str]) -> &mut Self {
        if extensions.is_empty() {
            panic!("The file extensions of a filter must be specified.")
        }

        self.filters.push(Filter {
            description,
            extensions,
        });
        self
    }

    show_impl!();
}

pub struct OpenMultipleFile<'a> {
    pub(crate) location: Option<&'a Path>,
    pub(crate) filters: Vec<Filter<'a>>,
}

impl<'a> OpenMultipleFile<'a> {
    pub fn new() -> Self {
        OpenMultipleFile {
            location: None,
            filters: vec![],
        }
    }

    pub fn location<P: AsRef<Path> + ?Sized>(&mut self, path: &'a P) -> &mut Self {
        self.location = Some(path.as_ref());
        self
    }

    pub fn filter(&mut self, description: &'a str, extensions: &'a [&'a str]) -> &mut Self {
        if extensions.is_empty() {
            panic!("The file extensions of a filter must be specified.")
        }

        self.filters.push(Filter {
            description,
            extensions,
        });
        self
    }

    show_impl!();
}

pub struct OpenSingleDir<'a> {
    pub(crate) location: Option<&'a Path>,
}

impl<'a> OpenSingleDir<'a> {
    pub fn new() -> Self {
        OpenSingleDir { location: None }
    }

    pub fn location<P: AsRef<Path> + ?Sized>(&mut self, path: &'a P) -> &mut Self {
        self.location = Some(path.as_ref());
        self
    }

    show_impl!();
}
