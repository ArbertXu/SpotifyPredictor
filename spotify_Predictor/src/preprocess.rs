use linfa::dataset::{Dataset, DatasetBase};
// use linfa::prelude::*;
use linfa::traits::Transformer;
use linfa_preprocessing::norm_scaling::NormScaler;
use ndarray::{Array1, Array2, Axis};
// use rand::thread_rng;
use rand_chacha::ChaCha8Rng;
use rand::SeedableRng;

fn check_l2_normalization(x: &Array2<f32>) {
    for (i, row) in x.axis_iter(Axis(0)).enumerate() {
        let norm = row.dot(&row).sqrt();
        if norm < 1.0
        {
            println!("Row {} norm: {:.4}", i, norm);
        }
    }
}


pub fn prepare_data(x: Array2<f32>, y: Array1<u8>) -> (DatasetBase<Array2<f32>, Array1<u8>>, DatasetBase<Array2<f32>, Array1<u8>>) {
    // let y = y.insert_axis(Axis(1));

    let scaler = NormScaler::l2();
    // let x_scaled: Array2<f32> = scaler.transform(x);
    let dataset = Dataset::new(x.clone(), y.clone());
    let dataset = scaler.transform(dataset);
    // check_l2_normalization(&dataset.records);
    // println!("y shape = {:?}", y.dim()); 
    // let mut rng = ChaCha8Rng::seed_from_u64(42);
    // let shuffled = dataset.shuffle(&mut rng);
    println!("x shape: {:?}", x.dim()); // should be (num_samples, num_features)
    println!("y shape: {:?}", y.dim()); // should be (num_samples,)
    let (train, valid) = dataset.split_with_ratio(0.8);
    // println!("Train set:");
    // for (i, (x, y)) in train.records.outer_iter().zip(train.targets.iter()).enumerate().take(100) {
    //     println!("Sample {}: features = {:?}, label = {}", i, x, y);
    // }
    (train, valid)
}
