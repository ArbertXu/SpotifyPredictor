use crate::data::Track;
use linfa_preprocessing::norm_scaling::NormScaler;
use linfa::dataset::{DatasetBase};
use ndarray:: {Array1, Array2};
use linfa::prelude::*;
pub fn extract_features(track: &Track, scaler: &NormScaler) -> Vec<f32> {
    let features = vec![
        track.danceability,
        track.energy,
        track.instrumentalness,
        // track.loudness,
        track.acousticness,
        track.valence,
        // track.tempo,
        // track.duration_ms,
        // track.liveness,
        track.speechiness,
    ];
    let input = Array2::from_shape_vec((1, features.len()), features)
        .expect("Failed to convert to ndarray");
    
    let input_dataset = DatasetBase::new(input.clone(), Array1::from(vec![1u8]));
    let scaled_input = scaler.transform(input_dataset);
    scaled_input.records.row(0).to_vec()
}

pub fn find_similar_tracks<'a>(
    input: &Track,
    dataset: &'a [Track],
    k: usize,
    scaler: &NormScaler,
) -> Vec<&'a Track> {
    let input_vec = extract_features(input, scaler);
    let mut distances: Vec<(&Track, f32)> = dataset.iter()
        .filter(|track| track.track_id != input.track_id && 
        track.genre == input.genre)
        .map(|track| {
            let scaled_features = extract_features(track, scaler);
            let dist = distance(&input_vec, &scaled_features);
            (track, dist)
        })
        .collect();
    distances.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    distances.iter().take(k).map(|(track, _)| *track).collect()
}

fn  distance(a: &[f32], b: &[f32]) -> f32 {
    a.iter().zip(b.iter()).map(|(x,y)| (x-y).powi(2)).sum::<f32>().sqrt()
}
