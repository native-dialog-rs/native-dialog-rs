use super::Dialog;
use crate::Filter;
use raw_window_handle::RawWindowHandle;
use std::path::{Path, PathBuf};

pub struct OpenSingleFile<'a> {
    pub(crate) filename: Option<&'a str>,
    pub(crate) location: Option<&'a Path>,
    pub(crate) title: &'a str,
    pub(crate) filters: Vec<Filter<'a>>,
    #[cfg_attr(not(any(target_os = "macos", target_os = "windows")), allow(dead_code))]
    pub(crate) owner: Option<RawWindowHandle>,
}

impl Dialog for OpenSingleFile<'_> {
    type Output = Option<PathBuf>;
}

impl<'a> OpenSingleFile<'a> {
    dialog_delegate!();
}

pub struct OpenMultipleFile<'a> {
    pub(crate) filename: Option<&'a str>,
    pub(crate) location: Option<&'a Path>,
    pub(crate) title: &'a str,
    pub(crate) filters: Vec<Filter<'a>>,
    #[cfg_attr(not(any(target_os = "macos", target_os = "windows")), allow(dead_code))]
    pub(crate) owner: Option<RawWindowHandle>,
}

impl Dialog for OpenMultipleFile<'_> {
    type Output = Vec<PathBuf>;
}

impl<'a> OpenMultipleFile<'a> {
    dialog_delegate!();
}

pub struct OpenSingleDir<'a> {
    pub(crate) filename: Option<&'a str>,
    pub(crate) location: Option<&'a Path>,
    pub(crate) title: &'a str,
    #[cfg_attr(not(any(target_os = "macos", target_os = "windows")), allow(dead_code))]
    pub(crate) owner: Option<RawWindowHandle>,
}

impl Dialog for OpenSingleDir<'_> {
    type Output = Option<PathBuf>;
}

impl<'a> OpenSingleDir<'a> {
    dialog_delegate!();
}

pub struct SaveSingleFile<'a> {
    pub(crate) filename: Option<&'a str>,
    pub(crate) location: Option<&'a Path>,
    pub(crate) title: &'a str,
    pub(crate) filters: Vec<Filter<'a>>,
    #[cfg_attr(not(any(target_os = "macos", target_os = "windows")), allow(dead_code))]
    pub(crate) owner: Option<RawWindowHandle>,
}

impl Dialog for SaveSingleFile<'_> {
    type Output = Option<PathBuf>;
}

impl<'a> SaveSingleFile<'a> {
    dialog_delegate!();
}
