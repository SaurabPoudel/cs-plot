use std::error::Error as StdError;
use std::fmt;
use std::io;
use plotters::drawing::DrawingAreaErrorKind;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Plot(String),
    Export(String),
    Parse(String),
    Drawing(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(err) => write!(f, "IO error: {}", err),
            Error::Plot(msg) => write!(f, "Plot error: {}", msg),
            Error::Export(msg) => write!(f, "Export error: {}", msg),
            Error::Parse(msg) => write!(f, "Parse error: {}", msg),
            Error::Drawing(msg) => write!(f, "Drawing error: {}", msg),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::Io(err) => Some(err),
            _ => None,
        }
    }
}

// Conversion from IO errors
impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}

// Conversion from serde_json errors
impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Export(err.to_string())
    }
}

// Conversion from float parse errors
impl From<std::num::ParseFloatError> for Error {
    fn from(err: std::num::ParseFloatError) -> Self {
        Error::Parse(err.to_string())
    }
}

// Conversion from integer parse errors
impl From<std::num::ParseIntError> for Error {
    fn from(err: std::num::ParseIntError) -> Self {
        Error::Parse(err.to_string())
    }
}

// Conversion from plotters drawing errors
impl<T: StdError + Send + Sync + 'static> From<DrawingAreaErrorKind<T>> for Error {
    fn from(err: DrawingAreaErrorKind<T>) -> Self {
        Error::Drawing(err.to_string())
    }
}

pub type Result<T> = std::result::Result<T, Error>;
