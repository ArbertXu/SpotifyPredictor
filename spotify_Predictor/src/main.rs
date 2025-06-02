mod data;
use data::{load_data, extract_train_data};
mod preprocess;
use preprocess::{prepare_all};
mod model;
use model::{train_model};
// use dotenv::dotenv;
// use dotenv::from_filename;
// use std::env;
mod predict;
use predict::predict_song_popularity;
use crate::data::Track;
mod to_csv;
use to_csv::export_to_csv;
mod hash_track;
use hash_track::build_track_map;
// use token::{get_access_token, search_for_track};

fn main() {
    // dotenv().ok();
    // from_filename("secret.env").ok();
    // let client_id = env::var("SPOTIFY_CLIENT_ID").expect("Error");
    // let secret_id = env::var("SPOTIFY_CLIENT_SECRET").expect("Error");
    // let access_token = get_access_token(&client_id, &secret_id);
    // test vals
    // let track = "larger than life";
    // let artist = "Denzel Curry";
    // let id = search_for_track(&track, &access_token, &artist);


    let tracks = load_data("SpotifyFeatures.csv"); 
    let track_result = match tracks {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error loading data {}", e);
            return;
        }
    };

    let mut popular: Vec<Track> = Vec::new();
    let mut not_popular: Vec<Track> = Vec::new();
    for track in track_result.iter() {
        if track.popularity > 40 {
            popular.push(track.clone());
        } else {
            not_popular.push(track.clone());
        }
    }
    popular.truncate(not_popular.len());
    let mut balanced = popular;
    balanced.extend(not_popular);


    export_to_csv(&track_result, "features.csv", 30);
    // let (x,y) = extract_train_data(&balanced);
    let (x,y) = extract_train_data(&balanced);
    // let (train, valid) = prepare_data(&x,&y);
    let (actual_model, scaler) = prepare_all(&x,&y);
    // let (train, valid) = actualModel.clone().split_with_ratio(0.8);

    // model_run(train, valid, &track_result);
    let model = train_model(actual_model);
    let track_map = build_track_map(&track_result);
    predict_song_popularity(&track_map, &model, "The one you love", "Randy Newman", &scaler);
    
    
}
