# Control System Plot Generator

A command-line tool for generating various control system plots (Bode, Nyquist, and Polar) from transfer functions.

## Features

- Generate Bode plots (magnitude and phase)
- Nyquist plots (coming soon)
- Polar plots (coming soon)
- Export frequency response data to JSON or CSV
- Configurable frequency range and resolution
- Support for arbitrary-order transfer functions

## Installation

1. Make sure you have Rust installed. If not, install it from [rustup.rs](https://rustup.rs/)
2. Clone the repository:

```bash
git clone https://github.com/SaurabPoudel/bode_plotter.git
cd bode_plotter
```

3. Build and install the project:
```bash
# Build the release version
cargo build --release

# Create a symbolic link to make it globally available as 'cs-plot'
sudo ln -s $(pwd)/target/release/engineering_plotter /usr/local/bin/cs-plot

# Verify installation
cs-plot --help
```

Now you can use the tool from anywhere by typing `cs-plot` instead of `cargo run`.

## Usage

### Basic Usage

Generate a Bode plot for a transfer function:
```bash
cs-plot -n "1000" -d "1,10,1000" -o "bode_plot.png"
```

This example plots H(s) = 1000/(s² + 10s + 1000)

### Command-Line Options

- `-t, --type <TYPE>`: Plot type (bode, nyquist, polar) [default: bode]
- `-n, --numerator <COEFFICIENTS>`: Numerator coefficients (comma-separated, ascending powers)
- `-d, --denominator <COEFFICIENTS>`: Denominator coefficients (comma-separated, ascending powers)
- `-s, --start <FREQ>`: Start frequency in Hz [default: 0.1]
- `-e, --end <FREQ>`: End frequency in Hz [default: 1000]
- `-p, --points <COUNT>`: Number of points per decade [default: 50]
- `-o, --output <FILE>`: Output file path [default: plot.png]
- `-x, --export <FILE>`: Export data to JSON/CSV file (optional)



### Examples

1. Detailed Bode plot with custom frequency range:
```bash
cs-plot -n "1000" -d "1,10,1000" -s 0.01 -e 10000 -p 100 -o "detailed_bode.png"
```

2. Generate Nyquist plot (coming soon):
```bash
cs-plot -t nyquist -n "1000" -d "1,10,1000" -o "nyquist.png"
```

3. Export frequency response data while plotting:
```bash
cs-plot -n "1000" -d "1,10,1000" -o "bode.png" -x "response_data.json"
```

## Input Format

Transfer functions are specified using coefficients in ascending powers:

- For H(s) = (as + b)/(cs² + ds + e), use:
  - Numerator: "b,a" (constant term first)
  - Denominator: "e,d,c" (constant term first)

## Dependencies

- plotters: For generating plots
- clap: Command-line argument parsing
- serde: Data serialization
- chrono: Timestamp handling

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Author

Saurab Poudel (poudelsaurab20@gmail.com)

## Acknowledgments

- The Rust community for excellent documentation and crates
- Control systems theory resources and references