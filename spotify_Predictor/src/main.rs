mod data;
use data::{load_data, extract_train_data, extract_data_own};
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
use axum::{extract::State, routing::post, Json, Router};
use axum::response::Response;
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, sync::Arc};
use tower_http::cors::{Any, CorsLayer};
use linfa_trees::DecisionTree;
use linfa_preprocessing::norm_scaling::NormScaler;
mod similar;
use similar::find_similar_tracks;
mod decision_tree;
use decision_tree::{build_tree, accuracy};
#[derive(Deserialize)]
struct PredictRequest {
    track_name: String,
    artist_name: String,
}
#[derive(Serialize)]
pub struct SimilarTrack {
    track_name: String,
    artist_name: String,
}

#[derive(Serialize)]
pub struct PredictResponse {
    track_name: String,
    artist_name: String,
    predicted: String,
    actual: String,
    recommendations: Vec<SimilarTrack>,
}

struct AppState {
    model: DecisionTree<f32,usize>,
    scaler: NormScaler,
    track_map: hash_track::TrackMap,
    pub all_tracks: Vec<Track>,

}
// use token::{get_access_token, search_for_track};
// async fn predict_handler(
//     State(state): State<Arc<AppState>>,
//     Json(payload): Json<PredictRequest>,
// ) -> Json<PredictResponse> {
//     match predict_song_popularity(
//         &state.track_map,
//         &state.model,
//         &payload.track_name,
//         &payload.artist_name,
//         &state.scaler,
//     ) {
//         Some((track, predicted_label, actual_label)) => {
//             let recommendations = find_similar_tracks(
//                 &track,
//                 &state.all_tracks,
//                 5,
//                 &state.scaler
//             ).iter().map(|t| SimilarTrack {
//                 track_name: t.track_name.clone(),
//                 artist_name: t.artist_name.clone(),
//             })
//             .collect::<Vec<SimilarTrack>>();

//             Json(PredictResponse {
//                 track_name: track.track_name.clone(),
//                 artist_name: track.artist_name.clone(),
//                 predicted: if predicted_label == 1 { "Popular".into() } else { "Not Popular".into() },
//                 actual: if actual_label == 1 { "Popular".into() } else { "Not Popular".into() },
//                 recommendations,
//             })
//         },
//         None => Json(PredictResponse {
//             track_name: payload.track_name.clone(),
//             artist_name: payload.artist_name.clone(),
//             predicted: "Not Found".into(),
//             actual: "Unknown".into(),
//             recommendations: vec![],
//         }),
//     }
// }

// #[tokio::main]
// async fn main() {
//     let tracks = load_data("SpotifyFeatures.csv").expect("Failed to load data");

//     let mut popular = Vec::new();
//     let mut not_popular = Vec::new();

//     for track in &tracks {
//         if track.popularity > 40 {
//             popular.push(track.clone());
//         } else {
//             not_popular.push(track.clone());
//         }
//     }

//     popular.truncate(not_popular.len());
//     let mut balanced = popular;
//     balanced.extend(not_popular);

//     let (x, y) = extract_train_data(&balanced);
//     let (dataset, scaler) = prepare_all(&x, &y);
//     let model = train_model(dataset);
//     let track_map = build_track_map(&tracks);

//     let state = Arc::new(AppState {
//         model,
//         scaler,
//         track_map,
//         all_tracks: tracks.clone(),
//     });

//     let app = Router::new()
//         .route("/predict", post(predict_handler))
//         .with_state(state)
//         .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any));

//         let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
//     println!("Server running at http://{}", addr);
//     axum_server::bind(addr)
//         .serve(app.into_make_service())
//         .await
//         .unwrap();

// }








// Terminal Code
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
    // for track in track_result.iter() {
    //     if track.popularity > 40 {
    //         popular.push(track.clone());
    //     } else {
    //         not_popular.push(track.clone());
    //     }
    // }
    // popular.truncate(not_popular.len());
    for track in &track_result {
        if track.popularity > 40 {
            if popular.len() < 20000 {
                popular.push(track.clone());
            }
        } else {
            if not_popular.len() < 20000 {
                not_popular.push(track.clone());
            }
        }
        if popular.len() == 20000 && not_popular.len() == 20000 {
            break;
        }
    }
    let mut balanced = popular;
    balanced.extend(not_popular);


    // export_to_csv(&track_result, "features.csv", 30);
    // let (x,y) = extract_train_data(&balanced);
    let (x,y) = extract_data_own(&balanced);
    let split = x.len() * 80 / 100;
    let (train_x, test_x) = x.split_at(split);
    let (train_y, test_y) = y.split_at(split);
    let all_rows: Vec<usize> = (0..train_x.len()).collect();
    let tree = build_tree(train_x, train_y, &all_rows, 0, 5);
    let acc = accuracy(&tree, test_x, test_y);
    println!("Accuracy: {}", acc);
    // let (train, valid) = prepare_data(&x,&y);
    // let (actual_model, scaler) = prepare_all(&x,&y);
    // let (train, valid) = actualModel.clone().split_with_ratio(0.8);
    // model_run(train, valid, &track_result);
    // let model = train_model(actual_model);
    // let track_map = build_track_map(&track_result);
    // predict_song_popularity(&track_map, &model, "The one you love", "Randy Newman", &scaler);


    
    
}
