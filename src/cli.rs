use clap::{App, Arg};

pub struct CLI;

impl CLI {
    pub fn new() -> Self {
        CLI
    }

    pub fn build_cli() -> App<'static, 'static> {
        App::new("Control Engineering Plot Generator")
            .version("1.0")
            .author("Saurab Poudel poudelsaurab20@gmail.com")
            .about("Generates various plots for transfer functions")
            .arg(
                Arg::with_name("plot-type")
                    .short("t")
                    .long("type")
                    .value_name("TYPE")
                    .help("Type of plot to generate (bode, nyquist, polar)")
                    .possible_values(&["bode", "nyquist", "polar"])
                    .default_value("bode")
                    .required(false),
            )
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
                    .value_name("COUNT")
                    .help("Number of points per decade")
                    .default_value("50"),
            )
            .arg(
                Arg::with_name("output")
                    .short("o")
                    .long("output")
                    .value_name("FILE")
                    .help("Output file path")
                    .default_value("plot.png"),
            )
            .arg(
                Arg::with_name("export")
                    .short("x")
                    .long("export")
                    .value_name("FILE")
                    .help("Export data to JSON/CSV file (determined by extension)")
                    .required(false),
            )
    }
}