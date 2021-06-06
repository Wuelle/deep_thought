use anyhow::Result;
use serde::Deserialize;
use ndarray::prelude::*;
use rust_nn::{
    activation::Activation,
    neural_network::{Layer, NeuralNetworkBuilder},
    loss::Loss,
    dataset::{Dataset, BatchSize},
};

#[derive(Debug, Deserialize)]
struct HeartFailureRecord {
    age: f64,
    anaemia: f64,
    creatinine_phosphokinase: f64,
    diabetes: f64,
    ejection_fraction: f64,
    high_blood_pressure: f64,
    platelets: f64,
    serum_creatinine: f64,
    serum_sodium: f64,
    sex: f64,
    smoking: f64,
    time: f64,
    death_event: f64,
}

fn main() -> Result<()>{
    // Dataset from https://www.kaggle.com/andrewmvd/heart-failure-clinical-data
    let mut rdr = csv::Reader::from_path("datasets/heart_failure_clinical_records_dataset.csv")?;

    let mut records = Array::zeros((0, 12));
    let mut labels = Array::zeros((0, 1));

    for result in rdr.deserialize() {
        let r: HeartFailureRecord = result?;
        
        records.push_row(ArrayView::from(&vec![r.age, r.anaemia, r.creatinine_phosphokinase, r.diabetes, r.ejection_fraction,
            r.high_blood_pressure, r.platelets, r.serum_creatinine, r.serum_sodium, r.sex, r.smoking, r.time]))?;
        labels.push_row(ArrayView::from(&vec![r.death_event]))?;
    }

    let dataset = Dataset::new(records, labels, 0.8, BatchSize::Number(2))?;

    // Build the neural net
    let mut net = NeuralNetworkBuilder::new()
        .learning_rate(0.01)
        .add_layer(Layer::new(12, 20))
        .add_layer(Layer::new(20, 10))
        .add_layer(Layer::new(10, 5))
        .add_layer(Layer::new(5, 1));//.activation(Activation::Sigmoid));

    // train the network
    for epoch in 0..1 {
        println!("training epoch {}", epoch);
        for (samples, labels) in dataset.iter_train().into_iter().take(5) {
            let _out = net.forward(&samples);
            println!("{}", _out);
            net.backprop(samples, labels, Loss::MSE);
        }
    }

    // evaluate the net 
    // let mut total_loss: f64 = 0.;
    // let loss = Loss::MSE;
    // for (sample, label) in dataset.iter_test() {
    //     let out = net.forward(&sample);
    //     println!("{} should be {}", &out, &label);
    //     total_loss += loss.compute(&out, &label);
    // }

    // println!("Mean loss over 100 test samples: {}", total_loss / 100.);
    Ok(())
}