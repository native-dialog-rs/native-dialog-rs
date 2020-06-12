use crate::Error;

mod file;
mod message;

impl From<osascript::Error> for Error {
    fn from(error: osascript::Error) -> Self {
        match error {
            osascript::Error::Io(e) => Error::IoFailure(e),
            osascript::Error::Json(_) => Error::UnexpectedOutput("osascript"),
            osascript::Error::Script(_) => Error::Unknown,
        }
    }
}
