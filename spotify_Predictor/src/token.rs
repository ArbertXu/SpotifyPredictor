use reqwest::blocking::Client;
use base64::Engine;
use urlencoding::encode;
use serde_json::Value;
use crate::data::Track;

pub fn get_access_token(client_id: &str, client_secret: &str) -> String {
    let credentials = base64::engine::general_purpose::STANDARD.encode(format!("{}:{}", client_id, client_secret));
    let client = Client::new();

    let res: Value = client
        .post("https://accounts.spotify.com/api/token")
        .header("Authorization", format!("Basic {}", credentials))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&[("grant_type", "client_credentials")])
        .send()
        .unwrap()
        .json()
        .unwrap();

    res["access_token"].as_str().unwrap().to_string()
}

pub fn search_for_track(track_name: &str, access_token: &str, artist_name: &str) -> Option<Track> {
    let query = format!("track:{} artist:{}", track_name, artist_name);
    let encoded_query = encode(&query);
    let track_url = format!(
        "https://api.spotify.com/v1/search?q={}&type=track&limit=1",
        encoded_query
    );
    let client = Client::new();
    let response_track = client
        .get(&track_url)
        .bearer_auth(access_token)
        .send()
        .unwrap()
        .json::<Value>()
        .unwrap();
     let track_id = response_track["tracks"]["items"][0]["id"]
        .as_str()?
        .to_string();
    let duration_ms = response_track["tracks"]["items"][0]["duration_ms"].as_f64()? as f32;
    let audio_url = format!("https://api.spotify.com/v1/audio-features/{}", track_id);
    let clientTwo = Client::new();
    let response = clientTwo
        .get(&audio_url)
        .header("Authorization", format!("Bearer {}", access_token))
        .bearer_auth(access_token)
        .send()
        .ok()?
        .json::<Value>()
        .unwrap();
    println!("Audio Features URL: {}", audio_url);
    println!("Track ID: {} (len: {})", track_id, track_id.len());
    println!("{}", response);
    Some(Track {
        genre: "NA".to_string(),
        artist_name: artist_name.to_string(),
        track_name: track_name.to_string(),
        track_id,
        popularity: 0,
        acousticness: response["acousticness"].as_f64()? as f32,
        danceability: response["danceability"].as_f64()? as f32,
        duration_ms,
        energy: response["energy"].as_f64()? as f32,
        instrumentalness: response["instrumentalness"].as_f64()? as f32,
        key: response["key"].to_string(),
        liveness: response["liveness"].as_f64()? as f32,
        loudness: response["loudness"].as_f64()? as f32,
        mode: response["mode"].to_string(),
        speechiness: response["speechiness"].as_f64()? as f32,
        tempo: response["tempo"].as_f64()? as f32,
        time_signature: response["time_signature"].to_string(),
        valence: response["valence"].as_f64()? as f32,
    })
}

// pub fn get_audio_features(track_id: &str, access_token: &str, track_name: &str, artist_name: &str) -> Option<Track> {
    
// }
