use plotters::prelude::*;
use crate::error::{Error, Result};
use crate::types::FrequencyResponseData;
use super::traitss::Plot;

pub struct BodePlot;

impl BodePlot {
    pub fn new() -> Self {
        BodePlot
    }
}

impl Plot for BodePlot {
    fn plot(&self, data: &FrequencyResponseData, output_path: &str) -> Result<()> {
        let root = BitMapBackend::new(output_path, (800, 600))
            .into_drawing_area();
        
        root.fill(&WHITE)?;

        let (upper, lower) = root.split_vertically(300);

        // Plot magnitude
        {
            let first_freq = data.responses.first()
                .ok_or_else(|| Error::Plot("No frequency response data available".to_string()))?;
            let last_freq = data.responses.last()
                .ok_or_else(|| Error::Plot("No frequency response data available".to_string()))?;

            let mut chart = ChartBuilder::on(&upper)
                .caption("Bode Plot - Magnitude", ("sans-serif", 30))
                .margin(5)
                .x_label_area_size(30)
                .y_label_area_size(30)
                .build_cartesian_2d(
                    first_freq.frequency.log10()..last_freq.frequency.log10(),
                    data.responses
                        .iter()
                        .map(|r| r.magnitude_db)
                        .fold(f64::NAN, f64::min)
                        ..data.responses
                            .iter()
                            .map(|r| r.magnitude_db)
                            .fold(f64::NAN, f64::max),
                )?;

            chart
                .configure_mesh()
                .x_desc("Frequency (Hz)")
                .y_desc("Magnitude (dB)")
                .draw()?;

            chart
                .draw_series(LineSeries::new(
                    data.responses
                        .iter()
                        .map(|r| (r.frequency.log10(), r.magnitude_db)),
                    &BLUE,
                ))?;
        }

        // Plot phase
        {
            let first_freq = data.responses.first()
                .ok_or_else(|| Error::Plot("No frequency response data available".to_string()))?;
            let last_freq = data.responses.last()
                .ok_or_else(|| Error::Plot("No frequency response data available".to_string()))?;

            let mut chart = ChartBuilder::on(&lower)
                .caption("Bode Plot - Phase", ("sans-serif", 30))
                .margin(5)
                .x_label_area_size(30)
                .y_label_area_size(30)
                .build_cartesian_2d(
                    first_freq.frequency.log10()..last_freq.frequency.log10(),
                    data.responses
                        .iter()
                        .map(|r| r.phase_deg)
                        .fold(f64::NAN, f64::min)
                        ..data.responses
                            .iter()
                            .map(|r| r.phase_deg)
                            .fold(f64::NAN, f64::max),
                )?;

            chart
                .configure_mesh()
                .x_desc("Frequency (Hz)")
                .y_desc("Phase (degrees)")
                .draw()?;

            chart
                .draw_series(LineSeries::new(
                    data.responses.iter().map(|r| (r.frequency.log10(), r.phase_deg)),
                    &RED,
                ))?;
        }

        Ok(())
    }

    fn plot_type(&self) -> &'static str {
        "bode"
    }
}
