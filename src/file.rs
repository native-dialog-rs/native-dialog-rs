pub struct Filter<'a> {
    #[cfg_attr(target_os = "macos", allow(dead_code))]
    pub(crate) description: &'a str,
    pub(crate) extensions: &'a [&'a str],
}

pub struct OpenSingleFile<'a> {
    pub(crate) location: Option<&'a str>,
    pub(crate) filters: Vec<Filter<'a>>,
}

impl<'a> OpenSingleFile<'a> {
    pub fn new() -> Self {
        OpenSingleFile {
            location: None,
            filters: vec![],
        }
    }

    pub fn location(&mut self, path: &'a str) -> &mut Self {
        self.location = Some(path);
        self
    }

    pub fn filter(&mut self, description: &'a str, extensions: &'a [&'a str]) -> &mut Self {
        self.filters.push(Filter {
            description,
            extensions,
        });
        self
    }

    show_impl!();
}

pub struct OpenMultipleFile<'a> {
    pub(crate) location: Option<&'a str>,
    pub(crate) filters: Vec<Filter<'a>>,
}

impl<'a> OpenMultipleFile<'a> {
    pub fn new() -> Self {
        OpenMultipleFile {
            location: None,
            filters: vec![],
        }
    }

    pub fn location(&mut self, path: &'a str) -> &mut Self {
        self.location = Some(path);
        self
    }

    pub fn filter(&mut self, description: &'a str, extensions: &'a [&'a str]) -> &mut Self {
        self.filters.push(Filter {
            description,
            extensions,
        });
        self
    }

    show_impl!();
}

pub struct OpenSingleDir<'a> {
    pub(crate) location: Option<&'a str>,
}

impl<'a> OpenSingleDir<'a> {
    pub fn new() -> Self {
        OpenSingleDir { location: None }
    }

    pub fn location(&mut self, dir: &'a str) -> &mut Self {
        self.location = Some(dir);
        self
    }

    show_impl!();
}
