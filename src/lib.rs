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
    chart: Option<ChartBuilder<'a, 'b, T>>,
}

pub fn plot(data: &Vec<(f64,f64)>) -> Result<(), Box<dyn std::error::Error>> {

    let mut data_copy: Vec<(f64,f64)> = vec![];
    for &x in data.iter() {
        data_copy.push(x);
    } 

    let root = BitMapBackend::new("kym-plot.png", (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Yanar dönerli Büşraa..", ("sans-serif", 60))
        .margin(10)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .build_ranged(0f64..10f64, 0f64..10f64)?;

    chart.configure_mesh().draw()?;

    chart.draw_series(LineSeries::new(data_copy, &GREEN.mix(20.0)))?
         .label("Raw Data")
         .legend(|(x,y)| PathElement::new(vec![(x,y), (x+20,y)], &GREEN));

    chart.configure_series_labels()
         .background_style(WHITE.filled())
         .draw()?;

    Ok(())
}

pub fn scatter(data: &Vec<(f64,f64)>) -> Result<(), Box<dyn std::error::Error>> {

    let mut data_copy: Vec<(f64,f64)> = vec![];
    for &x in data.iter() {
        data_copy.push(x);
    } 

    let root = BitMapBackend::new("kym-scatter.png", (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("my_plot", ("sans-serif", 60))
        .margin(10)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .build_ranged(0f64..10f64, 0f64..10f64)?;

    chart.configure_mesh().draw()?;

    chart.draw_series(data_copy.iter().map(|(x, y)| {
                ErrorBar::new_vertical(*x, *y, *y, *y, BLUE.filled(), 10)
            }),
        )?;

    chart.configure_series_labels()
         .background_style(WHITE.filled())
         .draw()?;

    Ok(())
}

pub fn scatter_error(data: &Vec<(f64,f64,f64,f64)>) -> Result<(), Box<dyn std::error::Error>> {

    let mut data_copy: Vec<(f64,f64,f64,f64)> = vec![];
    for &x in data.iter() {
        data_copy.push(x);
    } 

    let root = BitMapBackend::new("kym-scatter-error.png", (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("my_plot", ("sans-serif", 60))
        .margin(10)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .build_ranged(0f64..10f64, 0f64..10f64)?;

    chart.configure_mesh().draw()?;

    chart.draw_series(data_copy.iter().map(|(x, yl, ym, yh)| {
                ErrorBar::new_vertical(*x, *yl, *ym, *yh, BLUE.filled(), 10)
            }),
        )?;

    chart.configure_series_labels()
         .background_style(WHITE.filled())
         .draw()?;

    Ok(())
}

pub fn scatter_error2(data: &Vec<(f64,f64,f64)>) -> Result<(), Box<dyn std::error::Error>> {

    let mut data_copy: Vec<(f64,f64,f64)> = vec![];
    for &x in data.iter() {
        data_copy.push(x);
    } 

    let root = BitMapBackend::new("kym-scatter-error2.png", (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("my_plot", ("sans-serif", 60))
        .margin(10)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .build_ranged(0f64..10f64, 0f64..10f64)?;

    chart.configure_mesh().draw()?;

    chart.draw_series(data_copy.iter().map(|(x, y, y_err)| {
                ErrorBar::new_vertical(*x, (*y-*y_err), *y, (*y+*y_err), BLUE.filled(), 10)
            }),
        )?;

    chart.configure_series_labels()
         .background_style(WHITE.filled())
         .draw()?;

    Ok(())
}

pub fn scatter_error3(data: &Vec<(f64,f64,f64,f64)>) -> Result<(), Box<dyn std::error::Error>> {

    let mut data_copy: Vec<(f64,f64,f64,f64)> = vec![];
    for &x in data.iter() {
        data_copy.push(x);
    } 

    let root = BitMapBackend::new("kym-scatter-error3.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("my_plot", ("sans-serif", 60))
        .margin(10)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .build_ranged(0f64..4f64, 0f64..4f64)?;

    chart.configure_mesh().draw()?;

    for &i in data_copy.iter() {
        let temp_val_x = vec![(i.0-i.1, i.2), (i.0+i.1, i.2)];
        chart.draw_series(LineSeries::new(temp_val_x, &BLUE.mix(100.0)))?;

        let temp_val_y = vec![(i.0, i.2-i.3), (i.0, i.2+i.3)];
        chart.draw_series(LineSeries::new(temp_val_y, &BLUE.mix(100.0)))?;
    }

    chart.draw_series(LineSeries::new(vec![(0.0,0.0), (0.0,0.0)], &BLUE.mix(100.0)))?
        .label("Data")
        .legend(|(x,y)| PathElement::new(vec![(x,y), (x+20,y)], &BLUE));

    chart.configure_series_labels()
         .background_style(WHITE.filled())
         .draw()?;

    Ok(())
}

pub fn range(min_val: f64, max_val: f64, n: usize) -> Vec<f64> {
    let diff = max_val - min_val;
    let step_size = diff / (n as f64);
    let mut range_vec: Vec<f64> = vec![];
    let x_range = (1..=n).map(|i| i as f64 * step_size + min_val);
    for i in x_range {
        range_vec.push(i);
    }
    range_vec
}

pub fn draw_function(f: &dyn Fn(f64) -> f64, range_x: (f64,f64), range_y: (f64,f64)) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("draw_function.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("y=x^2", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_ranged(range_x.0..range_x.1, range_y.0..range_y.1)?;

    chart.configure_mesh().draw()?;

    let n: usize = 100;
    let diff = range_x.1 - range_x.0;
    let step = diff / (n as f64);
    let x_range = (1..=n).map(|i| i as f64 * step + range_x.0);
    //let x_range = range(range_x.0, range_x.1, 100);

    chart
        .draw_series(LineSeries::new(
            x_range.map(|x| (x, f(x))),
            &RED,
        ))?
        .label("y = x^2")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    Ok(())
}