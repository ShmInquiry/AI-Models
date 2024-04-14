/*
use std::error::Error;
use std::io;
use gnuplot::{Figure, Caption, Graph};
use gnuplot::AxesCommon;

fn main() -> Result<(), Box<dyn Error>>{
    let mut x: Vec<f64> = Vec::new();
    let mut y: Vec<f64> = Vec::new();

    let mut reader = csv::Reader::from_reader(io::stdin());
    for result in reader.records() {
        let record = result?;
        x.push(record[0].parse().unwrap());
        y.push(record[1].parse().unwrap());
    }

    let mut fg = Figure::new();
    fg.axes2d()
        .set_title("Wealth distribution", &[])
        .set_legend(Graph(0.9), Graph(0.1), &[], &[])
        .set_x_label("Avg Expenditure (KWD)", &[])
        .set_y_label("Wealth (KWD)", &[])
        .points(x, y, &[Caption("Target Distribution by income rate")]);
    fg.show();
    Ok(())
}
*/

use gnuplot::Graph;
use std::error::Error;
use std::io;
use gnuplot::{Figure, Caption, Color, PointSymbol};
use gnuplot::AxesCommon;

fn main() -> Result<(), Box<dyn Error>> {
    let mut x: [Vec<f64>; 3] = [Vec::new(), Vec::new(), Vec::new()];
    let mut y: [Vec<f64>; 3] = [Vec::new(), Vec::new(), Vec::new()];

    let mut reader = csv::Reader::from_reader(io::stdin());
    for result in reader.records() {
        let record = result?;
        let class: usize = record[2].parse().unwrap();
        x[class].push(record[0].parse().unwrap());
        y[class].push(record[1].parse().unwrap());
    }

    let mut fg = Figure::new();
    fg.axes2d()
        .set_title("Customer classification result", &[])
        .set_legend(Graph(0.9), Graph(0.1), &[], &[])
        .set_x_label("wealth (KWD)", &[])
        .set_y_label("expenditure (AVG)", &[])
        .points(&x[0], &y[0], &[Caption("Blue"), Color("blue"), PointSymbol('+')])
        .points(&x[1], &y[1], &[Caption("Silver"), Color("grey"), PointSymbol('x')])
        .points(&x[2], &y[2], &[Caption("Gold"), Color("yellow"), PointSymbol('o')]);

    fg.show();
    Ok(())
}
