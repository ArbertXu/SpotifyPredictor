use linfa_logistic::LogisticRegression;
use ndarray::{Array1, Array2, Axis};
use linfa::prelude::*;
use crate::data::Track;
pub fn modelRun(train: DatasetBase<Array2<f32>, Array1<u8>>, valid: DatasetBase<Array2<f32>, Array1<u8>>
                ,tracks: &[Track]) {

    let model = LogisticRegression::default().fit(&train).unwrap();
    let prediction = model.predict(&valid);
    let true_labels = valid.targets();

    let correct = prediction
        .iter()
        .zip(true_labels.iter())
        .filter(|(pred, actual)| pred == actual)
        .count();

    let accuracy = correct as f32 / true_labels.len() as f32;
    println!("Validation Accuracy: {:.2}%", accuracy * 100.0);
}