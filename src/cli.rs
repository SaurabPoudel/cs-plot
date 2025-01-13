use clap::{App, Arg};


pub struct CLI;

impl CLI {
    pub fn new() -> Self {
        CLI
    }

    pub fn build_cli() -> App<'static, 'static> {
        App::new("Bode Plot Generator")
            .version("1.0")
            .author("Your Name")
            .about("Generates Bode plots for transfer functions")
            .arg(
                Arg::with_name("numerator")
                    .short("n")
                    .long("numerator")
                    .value_name("COEFFICIENTS")
                    .help("Numerator coefficients (comma-separated, ascending powers)")
                    .required(true),
            )
            .arg(
                Arg::with_name("denominator")
                    .short("d")
                    .long("denominator")
                    .value_name("COEFFICIENTS")
                    .help("Denominator coefficients (comma-separated, ascending powers)")
                    .required(true),
            )
            .arg(
                Arg::with_name("fstart")
                    .short("s")
                    .long("start")
                    .value_name("FREQ")
                    .help("Start frequency in Hz")
                    .default_value("0.1"),
            )
            .arg(
                Arg::with_name("fend")
                    .short("e")
                    .long("end")
                    .value_name("FREQ")
                    .help("End frequency in Hz")
                    .default_value("1000"),
            )
            .arg(
                Arg::with_name("points")
                    .short("p")
                    .long("points")
                    .value_name("NUM")
                    .help("Number of frequency points")
                    .default_value("1000"),
            )
            .arg(
                Arg::with_name("output")
                    .short("o")
                    .long("output")
                    .value_name("FILE")
                    .help("Output plot file path")
                    .default_value("bode_plot.png"),
            )
            .arg(
                Arg::with_name("export")
                    .short("x")
                    .long("export")
                    .value_name("FILE")
                    .help("Export data (supported formats: .csv, .json)")
                    .takes_value(true),
            )
    }
}