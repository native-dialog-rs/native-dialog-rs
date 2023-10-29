#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;
#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc_foundation;

use thiserror::Error;

pub use file::*;
pub use message::*;
pub use progress::*;

#[derive(Error, Debug)]
pub enum Error {
    #[error("system error or I/O failure")]
    IoFailure(#[from] std::io::Error),

    #[error("the implementation returns malformed strings")]
    InvalidString(#[from] std::string::FromUtf8Error),

    #[error("failed to parse the string returned from implementation")]
    UnexpectedOutput(&'static str),

    #[error("cannot find any dialog implementation (kdialog/zenity)")]
    NoImplementation,

    #[error("the implementation reports error")]
    ImplementationError(String),

    #[error("Percentage must be between 0 and 100 inclusive, you passed {0}")]
    InvalidPercentage(f32),
}

pub type Result<T> = std::result::Result<T, Error>;

pub(crate) mod dialog;
pub(crate) mod dialog_impl;
pub(crate) mod util;

mod file;
mod message;
mod progress;

mod tests;
