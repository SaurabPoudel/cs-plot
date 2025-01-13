use num_complex::Complex64;
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;
use crate::error::{Error, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferFunction {
    pub num_coeffs: Vec<f64>,
    pub den_coeffs: Vec<f64>,
}

impl TransferFunction {
    pub fn new(num_str: &str, den_str: &str) -> Result<Self> {
        let num_coeffs = parse_coefficients(num_str)?;
        let den_coeffs = parse_coefficients(den_str)?;

        if den_coeffs.iter().all(|&x| x == 0.0) {
            return Err(Error::Parse("Denominator cannot be all zeros".into()));
        }

        Ok(TransferFunction {
            num_coeffs,
            den_coeffs,
        })
    }

    pub fn evaluate(&self, freq: f64) -> Complex64 {
        let s = Complex64::new(0.0, 2.0 * PI * freq);

        let num: Complex64 = self
            .num_coeffs
            .iter()
            .enumerate()
            .map(|(i, &c)| c * s.powi(i as i32))
            .sum();

        let den: Complex64 = self
            .den_coeffs
            .iter()
            .enumerate()
            .map(|(i, &c)| c * s.powi(i as i32))
            .sum();

        num / den
    }
}

fn parse_coefficients(coeff_str: &str) -> Result<Vec<f64>> {
    coeff_str
        .split(',')
        .map(|s| s.trim().parse::<f64>())
        .collect::<std::result::Result<Vec<f64>, _>>()
        .map_err(|e| Error::Parse(format!("Failed to parse coefficients: {}", e)))
}