use linfa::prelude::*;
// use linfa_logistic::{LogisticRegression, FittedLogisticRegression};
use ndarray::{ Array2, Array1};
use crate::data::Track;
use linfa_preprocessing::norm_scaling::NormScaler;
use linfa::dataset::{DatasetBase};
use linfa_trees::DecisionTree;
use crate::hash_track::find_track;
use std::collections::HashMap;
use serde::Serialize;
use axum::Json;
use crate::PredictResponse;


pub fn predict_song_popularity(tracks: &HashMap<(String, String), Track>, model: & DecisionTree<f32,usize>, track_name: &str, artist_name: &str, scaler: &NormScaler) -> Option<PredictResponse>{

    // let track = match maybe_track {
    //     Some(t) => t,
    //     None => {
    //         println!("Track not found in dataset.");
    //         return;
    //     }
    // };
    let track = find_track(tracks, track_name, artist_name)?;

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

    // Predict
    let prediction = model.predict(&scaled_input);

    let predicted_label = prediction[0];
    let actual_label = if track.popularity > 60 { 1 } else { 0 };

    // println!(
    //     "Track: '{}' by {}\nPredicted: {}\nActual: {}",
    //     track.track_name,
    //     track.artist_name,
    //     if predicted_label == 1 { "Popular" } else { "Not Popular" },
    //     if actual_label == 1 { "Popular" } else { "Not Popular" },
    // );
    Some(PredictResponse {
        track_name: track.track_name.clone(),
        artist_name: track.artist_name.clone(),
        predicted: if predicted_label == 1 {
            "Popular".into()
        } else {
            "Not Popular".into()
        },
        actual: if actual_label == 1 {
            "Actually Popular".into()
        } else {
            "Not Popular".into()
        },
    })
}