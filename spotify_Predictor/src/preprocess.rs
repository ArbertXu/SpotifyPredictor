use linfa::dataset::{Dataset, DatasetBase};
use linfa::traits::Transformer;
// use linfa_preprocessing::traits::Fit;
use linfa_preprocessing::norm_scaling::NormScaler;
use ndarray::{Array1, Array2};
// use rand_chacha::ChaCha8Rng;
// use rand::SeedableRng;



// pub fn prepare_data(x: &Array2<f32>, y: &Array1<u8>) -> (DatasetBase<Array2<f32>, Array1<u8>>, DatasetBase<Array2<f32>, Array1<u8>>) {

//     let scaler = NormScaler::l2();
//     let dataset = Dataset::new(x.clone(), y.clone());
//     let dataset = scaler.transform(dataset);
//     let mut rng = ChaCha8Rng::seed_from_u64(42);
//     let shuffled = dataset.shuffle(&mut rng);
//     let (train, valid) = shuffled.split_with_ratio(0.8);
//     (train, valid)
// }

pub fn prepare_all(x: &Array2<f32>, y: &Array1<usize>) -> (DatasetBase<Array2<f32>, Array1<usize>>, NormScaler)
{
    let scaler = NormScaler::l2();
    let dataset = Dataset::new(x.clone(), y.clone());
    let dataset = scaler.transform(dataset);
    (dataset, scaler)
}
