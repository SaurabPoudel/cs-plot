use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum PlotterError {
    InvalidInput(String),
    ExportError(String),
    PlottingError(String),
}

impl fmt::Display for PlotterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PlotterError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            PlotterError::ExportError(msg) => write!(f, "Export error: {}", msg),
            PlotterError::PlottingError(msg) => write!(f, "Plotting error: {}", msg),
        }
    }
}

impl Error for PlotterError {}

