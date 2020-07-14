pub struct OpenSingleFile<'a> {
    pub dir: Option<&'a str>,
    pub filter: Option<&'a [&'a str]>,
}

pub struct OpenMultipleFile<'a> {
    pub dir: Option<&'a str>,
    pub filter: Option<&'a [&'a str]>,
}

pub struct OpenSingleDirectory<'a> {
    pub dir: Option<&'a str>,
}

pub struct SaveFile<'a> {
    pub dir: Option<&'a str>,
    pub name: &'a str,
}
