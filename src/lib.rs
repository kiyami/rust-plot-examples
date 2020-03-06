use plotters;
use plotters::prelude::*;

use rand::SeedableRng;
use rand_distr::{Distribution, Normal};
use rand_xorshift::XorShiftRng;

use itertools::Itertools;

use num_traits::sign::Signed;

#[allow(dead_code)]
pub struct Plot<'a, 'b, T, B> where 
    T: plotters::drawing::backend::DrawingBackend,
    B: plotters::coord::CoordTranslate {

    root: DrawingArea<T,B>,
    data: Vec<(f64,f64)>,
    chart: ChartBuilder<'a, 'b, T>,
}

pub fn scatter(data: Vec<(f64,f64)>) -> Result<(), Box<dyn std::error::Error>> {

    let root = BitMapBackend::new("kym.png", (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("my_plot", ("sans-serif", 60))
        .margin(10)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .build_ranged(0f64..10f64, 0f64..10f64)?;

    chart.configure_mesh().draw()?;

    chart.draw_series(LineSeries::new(data, &GREEN.mix(10.0)))?
         .label("Raw Data")
         .legend(|(x,y)| PathElement::new(vec![(x,y), (x+20,y)], &GREEN));

    chart.configure_series_labels()
         .background_style(WHITE.filled())
         .draw()?;

    Ok(())
}