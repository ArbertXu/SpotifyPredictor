// use linfa_logistic::{LogisticRegression, FittedLogisticRegression};
use ndarray::{Array1, Array2};
use linfa::prelude::*;
// use crate::data::Track;
use linfa_trees::DecisionTree;
// pub fn model_run(train: DatasetBase<Array2<f32>, Array1<u8>>, valid: DatasetBase<Array2<f32>, Array1<u8>>
//                 ,tracks: &[Track]) {

//     let model = LogisticRegression::default().fit(&train).unwrap();
//     let prediction = model.predict(&valid);
//     let true_labels = valid.targets();

//     let correct = prediction
//         .iter()
//         .zip(true_labels.iter())
//         .filter(|(pred, actual)| pred == actual)
//         .count();
//     let accuracy = correct as f32 / true_labels.len() as f32;
//     println!("Validation Accuracy: {:.2}%", accuracy * 100.0);
// }

pub fn train_model(train: DatasetBase<Array2<f32>, Array1<usize>>) -> DecisionTree<f32,usize>{
//LogisticRegression
//     let model = LogisticRegression::default().max_iterations(1000).fit(&train).unwrap();
//     return model;

    let model = DecisionTree::params().max_depth(Some(5)).fit(&train).unwrap();
    let accuracy = model.predict(&train).confusion_matrix(&train).unwrap().accuracy();
    println!("Training Accuracy: {:.2}%", accuracy);

    model
}