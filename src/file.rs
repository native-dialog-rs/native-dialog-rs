pub struct OpenSingleFile<'a> {
    pub dir: Option<&'a str>,
    pub filter: Option<&'a [&'a str]>,
}

pub struct OpenMultipleFile<'a> {
    pub dir: Option<&'a str>,
    pub filter: Option<&'a [&'a str]>,
}

pub struct OpenSingleDir<'a> {
    pub dir: Option<&'a str>,
}

#[doc(hidden)]
#[cfg(any())]
struct SaveFile<'a> {
    pub dir: Option<&'a str>,
    pub name: &'a str,
}
