use std::path::Path;
use crate::error::Result;
use crate::types::FrequencyResponseData;

mod csv;
mod json;

pub use csv::CsvExporter;
pub use json::JsonExporter;

pub trait DataExporter {
    fn export(&self, data: &FrequencyResponseData, path: &str) -> Result<()>;
    fn format(&self) -> &'static str;
}

#[derive(Debug)]
pub struct ExportFormat;

impl ExportFormat {
    pub fn from_path(path: &str) -> Result<Box<dyn DataExporter>> {
        match Path::new(path).extension().and_then(|s| s.to_str()) {
            Some("csv") => Ok(Box::new(CsvExporter::new())),
            Some("json") => Ok(Box::new(JsonExporter::new())),
            _ => Err(crate::error::Error::Export(
                "Unsupported file format. Use .csv or .json".into()
            ))
        }
    }
}
