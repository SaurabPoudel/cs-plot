use plotters::prelude::*;
use crate::error::{Error, Result};
use crate::types::FrequencyResponseData;
use super::traitss::Plot;

pub struct NyquistPlot;

impl NyquistPlot {
    pub fn new() -> Self {
        NyquistPlot
    }
}

impl Plot for NyquistPlot {
    fn plot(&self, data: &FrequencyResponseData, output_path: &str) -> Result<()> {
        let root = BitMapBackend::new(output_path, (800, 600))
            .into_drawing_area();
        
        root.fill(&WHITE)?;

        let complex_points: Vec<(f64, f64)> = data.responses
            .iter()
            .map(|response| {
                let complex_val = data.transfer_function.evaluate(response.frequency);
                (complex_val.re, complex_val.im)
            })
            .collect();

        if complex_points.is_empty() {
            return Err(Error::Plot("No frequency response data available".to_string()));
        }

        let real_min = complex_points.iter().map(|(re, _)| *re).fold(f64::INFINITY, f64::min);
        let real_max = complex_points.iter().map(|(re, _)| *re).fold(f64::NEG_INFINITY, f64::max);
        let imag_min = complex_points.iter().map(|(_, im)| *im).fold(f64::INFINITY, f64::min);
        let imag_max = complex_points.iter().map(|(_, im)| *im).fold(f64::NEG_INFINITY, f64::max);

        let real_range = real_max - real_min;
        let imag_range = imag_max - imag_min;
        let padding = 0.1;
        
        let real_start = real_min - real_range * padding;
        let real_end = real_max + real_range * padding;
        let imag_start = imag_min - imag_range * padding;
        let imag_end = imag_max + imag_range * padding;

        let mut chart = ChartBuilder::on(&root)
            .caption("Nyquist Plot", ("sans-serif", 40))
            .margin(20)
            .x_label_area_size(50)
            .y_label_area_size(50)
            .build_ranged(real_start..real_end, imag_start..imag_end)?;

        chart
            .configure_mesh()
            .x_desc("Real Part")
            .y_desc("Imaginary Part")
            .draw()?;

        chart
            .draw_series(LineSeries::new(
                complex_points.iter().cloned(),
                &BLUE,
            ))?
            .label("Nyquist Plot")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], &BLUE));

        if let Some(&first_point) = complex_points.first() {
            chart
                .draw_series(PointSeries::of_element(
                    vec![first_point],
                    5,
                    &GREEN,
                    &|c, s, st| {
                        return EmptyElement::at(c)
                            + Circle::new((0, 0), s, st.filled())
                    },
                ))?
                .label("Start (Low Freq)")
                .legend(|(x, y)| Circle::new((x + 10, y), 5, &GREEN));
        }

        if let Some(&last_point) = complex_points.last() {
            chart
                .draw_series(PointSeries::of_element(
                    vec![last_point],
                    5,
                    &RED,
                    &|c, s, st| {
                        return EmptyElement::at(c)  
                            + Circle::new((0, 0), s, st.filled()) 
                    },
                ))?
                .label("End (High Freq)")
                .legend(|(x, y)| Circle::new((x + 10, y), 5, &RED));
        }

        if real_start <= 0.0 && real_end >= 0.0 {
            chart
                .draw_series(LineSeries::new(
                    vec![(0.0, imag_start), (0.0, imag_end)],
                    &BLACK.mix(0.3),
                ))?;
        }

        if imag_start <= 0.0 && imag_end >= 0.0 {
            chart
                .draw_series(LineSeries::new(
                    vec![(real_start, 0.0), (real_end, 0.0)],
                    &BLACK.mix(0.3),
                ))?;
        }

        chart
            .configure_series_labels()
            .background_style(&WHITE.mix(0.8))
            .border_style(&BLACK)
            .draw()?;

        root.present()?;
        
        Ok(())
    }

    fn plot_type(&self) -> &'static str {
        "nyquist"
    }
}