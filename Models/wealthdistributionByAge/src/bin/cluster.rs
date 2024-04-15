extern crate rusty_machine;
extern crate rand;

use std::io;
use std::error::Error;

use rusty_machine::linalg::{Matrix, BaseMatrix};
use rusty_machine::learning::k_means::KMeansClassifier;
use rusty_machine::learning::UnSupModel;

const CLUSTER_COUNT: usize = 3;

fn read_data_from_stdin() -> Result<Matrix<f64>, Box<dyn Error>> {
    let mut reader = csv::Reader::from_reader(io::stdin());
    let mut data: Vec<f64>= vec!();
    for result in reader.records() {
        let record = result?;
        data.push(record[0].parse().unwrap());
        data.push(record[1].parse().unwrap());
    }

    Ok(Matrix::new(&data.len() / 2, 2, data))
}

fn export_result_to_stdout(samples: Matrix<f64>, classes: Vec<&str>) -> Result<(), Box<dyn Error>> {
    let mut writer = csv::Writer::from_writer(io::stdout());
    writer.write_record(&["expenditure", "wealth", "class"])?;
    
    for (sample, user_category) in samples.iter_rows().zip(classes.iter()) {
        let mut record: Vec<String> = sample.iter().map(|&x| x.to_string()).collect();
        record.push(user_category.to_string());
        writer.write_record(&record)?;
    }
    
    Ok(())
}
// fn main() {

//     let samples = read_data_from_stdin().unwrap();

//     let mut model = KMeansClassifier::new(CLUSTER_COUNT);
//     model.train(&samples).unwrap();

//     let classes = model.predict(&samples).unwrap();

//     export_result_to_stdout(samples, classes.into_vec()).unwrap();
// }
fn main() {
    let samples = read_data_from_stdin().unwrap();

    let mut model = KMeansClassifier::new(CLUSTER_COUNT);
    model.train(&samples).unwrap();

    let classes = model.predict(&samples).unwrap();

    let mut user_categories = vec!["saver", "traveler", "foodie"]; // Define the new user categories

    // Map the cluster indices to user categories
    let classified_users: Vec<&str> = classes.iter().map(|&class_index| user_categories[class_index]).collect();

    export_result_to_stdout(samples, classified_users).unwrap();
}
