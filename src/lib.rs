use thiserror::Error;

pub(crate) mod builder;
pub(crate) mod dialog;
pub(crate) mod util;

pub use builder::*;

pub type Result<T = ()> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("system error or I/O failure")]
    Io(#[from] std::io::Error),

    #[error("the implementation returns an invalid utf-8 string")]
    Utf8(#[from] std::string::FromUtf8Error),

    #[error("cannot find any dialog implementation (kdialog/zenity)")]
    NoImplementation,

    #[error("the implementation reports error")]
    ImplementationError(String),
}
