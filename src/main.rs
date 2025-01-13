use std::process;
use chrono::Local;

use engineering_plotter::{
    cli::CLI,
    error::Result,
    export::JsonExporter,
    plotters::BodePlot,
    types::{FrequencyResponseData, TransferFunction},
    Plot,
    DataExporter,
};

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

fn run() -> Result<()> {
    let matches = CLI::build_cli().get_matches();

    // Parse command line arguments
    let numerator = matches.value_of("numerator").unwrap();
    let denominator = matches.value_of("denominator").unwrap();
    let start_freq = matches.value_of("fstart").unwrap().parse::<f64>()?;
    let end_freq = matches.value_of("fend").unwrap().parse::<f64>()?;
    let points = matches.value_of("points").unwrap().parse::<usize>()?;
    let output = matches.value_of("output").unwrap();
    let export = matches.value_of("export");

    // Parse transfer function
    let transfer_fn = TransferFunction::new(numerator, denominator)?;

    // Generate frequency response data
    let data = FrequencyResponseData::new(
        transfer_fn,
        start_freq,
        end_freq,
        points,
        Local::now().to_rfc3339(),
    );

    // Plot data
    let plotter = BodePlot::new();
    plotter.plot(&data, output)?;

    // Export data if requested
    if let Some(export_path) = export {
        let exporter = JsonExporter::new();
        exporter.export(&data, export_path)?;
    }

    Ok(())
}
