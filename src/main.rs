use std::process;
use chrono::Local;

use engineering_plotter::{
    cli::CLI,
    error::{Error, Result},
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
    let plot_type = matches.value_of("plot-type").unwrap_or("bode");
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

    // Create and use plotter based on type
    match plot_type {
        "bode" => {
            let plotter = BodePlot::new();
            plotter.plot(&data, output)?;
        }
        "nyquist" => {
            return Err(Error::Plot("Nyquist plot not yet implemented".into()));
        }
        "polar" => {
            return Err(Error::Plot("Polar plot not yet implemented".into()));
        }
        _ => unreachable!(), // clap ensures this
    }

    // Export data if requested
    if let Some(export_path) = export {
        let exporter = JsonExporter::new();
        exporter.export(&data, export_path)?;
    }

    Ok(())
}
