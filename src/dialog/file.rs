use super::Dialog;
use crate::Filter;
use std::path::{Path, PathBuf};

pub struct OpenSingleFile<'a> {
    pub(crate) filename: Option<&'a str>,
    pub(crate) location: Option<&'a Path>,
    pub(crate) filters: Vec<Filter<'a>>,
}

impl Dialog for OpenSingleFile<'_> {
    type Output = Option<PathBuf>;
}

pub struct OpenMultipleFile<'a> {
    pub(crate) filename: Option<&'a str>,
    pub(crate) location: Option<&'a Path>,
    pub(crate) filters: Vec<Filter<'a>>,
}

impl Dialog for OpenMultipleFile<'_> {
    type Output = Vec<PathBuf>;
}

pub struct OpenSingleDir<'a> {
    pub(crate) filename: Option<&'a str>,
    pub(crate) location: Option<&'a Path>,
}

impl Dialog for OpenSingleDir<'_> {
    type Output = Option<PathBuf>;
}

pub struct SaveSingleFile<'a> {
    pub(crate) filename: Option<&'a str>,
    pub(crate) location: Option<&'a Path>,
    pub(crate) filters: Vec<Filter<'a>>,
}

impl Dialog for SaveSingleFile<'_> {
    type Output = Option<PathBuf>;
}
