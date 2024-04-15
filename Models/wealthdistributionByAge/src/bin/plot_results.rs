use std::error::Error;
use std::io;
use gnuplot::{Figure, Caption, Graph, Color, PointSymbol};
use gnuplot::AxesCommon;

fn main() -> Result<(), Box<dyn Error>>{
    let mut x: [Vec<f64>; 3] = [Vec::new(), Vec::new(), Vec::new()];
    let mut y: [Vec<f64>; 3] = [Vec::new(), Vec::new(), Vec::new()];

    let mut reader = csv::Reader::from_reader(io::stdin());
    for result in reader.records() {
        let record = result?;
        let class:usize = record[2].parse().unwrap();
        x[class].push(record[0].parse().unwrap());
        y[class].push(record[1].parse().unwrap());
    }

    let mut fg = Figure::new();
    fg.axes2d()
            .set_title("Cards classification result (based on user's data)", &[])
            .set_legend(Graph(0.9), Graph(0.1), &[], &[])
            .set_x_label("Wealth (KWD)", &[])
            .set_y_label("Expenditure (AVG)", &[])
            .points(
                    &x[0],
                    &y[0],
                    &[Caption("Saver card"), Color("pink"), PointSymbol('+')],
            )
            .points(
                    &x[1],
                    &y[1],
                    &[Caption("Traveller card"), Color("blue"), PointSymbol('x')],
            )
            .points(
                    &x[2],
                    &y[2],
                    &[Caption("Foodie card"), Color("green"), PointSymbol('o')],
            );
    fg.show();
    Ok(())
}
