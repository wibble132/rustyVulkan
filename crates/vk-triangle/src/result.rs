use std::fmt::{Debug, Display, Formatter};

pub(crate) type Result<T> = std::result::Result<T, Box<dyn core::error::Error>>;

pub(crate) struct Error {
    msg: String,
}

pub fn err(msg: &str) -> Error {
    Error {
        msg: msg.to_string(),
    }
}

pub fn error<T>(msg: &str) -> Result<T> {
    Err(Box::new(Error {
        msg: msg.to_string(),
    }))
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl std::error::Error for Error {}
