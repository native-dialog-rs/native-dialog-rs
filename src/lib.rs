use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("system error or I/O failure")]
    Io(#[from] std::io::Error),

    #[error("the implementation returns an invalid utf-8 string")]
    Utf8(#[from] std::string::FromUtf8Error),

    #[error("the implementation is terminated by signal")]
    ProcessTerminated(&'static str),

    #[error("cannot find any dialog implementation (kdialog/zenity)")]
    NoImplementation,

    #[error("the implementation reports error")]
    ImplementationError(String),
}

pub type Result<T> = std::result::Result<T, Error>;

pub(crate) mod dialog;
pub(crate) mod dialog_impl;
pub(crate) mod util;

mod message;
pub use message::*;

mod file;
pub use file::*;
