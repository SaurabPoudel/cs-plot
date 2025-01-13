use std::fs::File;
use std::io::Write;

use crate::error::Result;
use crate::types::FrequencyResponseData;
use super::DataExporter;

pub struct CsvExporter;

impl CsvExporter {
    pub fn new() -> Self {
        CsvExporter
    }
}

impl DataExporter for CsvExporter {
    fn export(&self, data: &FrequencyResponseData, path: &str) -> Result<()> {
        let mut file = File::create(path)?;

        // Write header
        writeln!(file, "Frequency (Hz),Magnitude (dB),Phase (degrees)")?;

        // Write data rows
        for response in &data.responses {
            writeln!(
                file,
                "{},{},{}",
                response.frequency,
                response.magnitude_db,
                response.phase_deg
            )?;
        }

        Ok(())
    }

    fn format(&self) -> &'static str {
        "csv"
    }
}