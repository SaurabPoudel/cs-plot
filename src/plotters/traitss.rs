use crate::error::Result;
use crate::types::FrequencyResponseData;

pub trait Plot {
    fn plot(&self, data: &FrequencyResponseData, output_path: &str) -> Result<()>;
    fn plot_type(&self) -> &'static str;
}

pub struct PlotFormat;

impl PlotFormat {
    pub fn new(plot_type: &str) -> Result<Box<dyn Plot>> {
        match plot_type {
            "bode" => Ok(Box::new(crate::plotters::bode::BodePlot::new())),
            "nyquist" => Ok(Box::new(crate::plotters::nyquist::NyquistPlot::new())),
            _ => Err(crate::error::Error::Plot(
                "Unsupported plot type. Use 'bode' or 'nyquist'".into()
            ))
        }
    }
}