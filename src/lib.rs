#![doc = include_str!("../README.md")]

use std::ffi::OsString;

use thiserror::Error;

pub(crate) mod builder;
pub(crate) mod dialog;
pub(crate) mod ffi;
pub(crate) mod utils;

pub use builder::*;

pub type Result<T = ()> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("system error or I/O failure")]
    Io(#[from] std::io::Error),

    #[error("invalid utf-8 string")]
    Utf8(#[from] std::string::FromUtf8Error),

    #[error("cannot find implementation (kdialog/zenity)")]
    MissingDep,

    #[error("subprocess killed by signal")]
    Killed(OsString),

    #[error("other errors reported by implementation")]
    Other(String),
}
