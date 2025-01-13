use serde::{Deserialize, Serialize};
use super::transfer_fn::TransferFunction;
use std::f64::consts::PI;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrequencyResponse {
    pub frequency: f64,
    pub magnitude_db: f64,
    pub phase_deg: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrequencyResponseData {
    pub transfer_function: TransferFunction,
    pub responses: Vec<FrequencyResponse>,
    pub metadata: ExportMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportMetadata {
    pub timestamp: String,
    pub start_frequency: f64,
    pub end_frequency: f64,
    pub num_points: usize,
}

impl FrequencyResponseData {
    pub fn new(
        transfer_function: TransferFunction,
        start_freq: f64,
        end_freq: f64,
        num_points: usize,
        timestamp: String,
    ) -> Self {
        let frequencies = generate_frequency_points(start_freq, end_freq, num_points);
        let responses = calculate_response(&transfer_function, &frequencies);

        FrequencyResponseData {
            transfer_function,
            responses,
            metadata: ExportMetadata {
                timestamp,
                start_frequency: start_freq,
                end_frequency: end_freq,
                num_points,
            },
        }
    }
}

fn generate_frequency_points(start: f64, end: f64, points: usize) -> Vec<f64> {
    let log_start = start.log10();
    let log_end = end.log10();
    let step = (log_end - log_start) / (points as f64 - 1.0);

    (0..points)
        .map(|i| 10.0_f64.powf(log_start + step * i as f64))
        .collect()
}

fn calculate_response(tf: &TransferFunction, frequencies: &[f64]) -> Vec<FrequencyResponse> {
    frequencies
        .iter()
        .map(|&f| {
            let resp = tf.evaluate(f);
            FrequencyResponse {
                frequency: f,
                magnitude_db: 20.0 * resp.norm().log10(),
                phase_deg: resp.arg() * 180.0 / PI,
            }
        })
        .collect()
}