use std::fs::File;
use crate::error::Result;
use crate::types::FrequencyResponseData;
use super::DataExporter;

pub struct JsonExporter;

impl JsonExporter {
    pub fn new() -> Self {
        JsonExporter
    }
}

impl DataExporter for JsonExporter {
    fn export(&self, data: &FrequencyResponseData, path: &str) -> Result<()> {
        let file = File::create(path)?;
        serde_json::to_writer_pretty(file, data)?;
        Ok(())
    }

    fn format(&self) -> &'static str {
        "json"
    }
}