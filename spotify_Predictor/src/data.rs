use serde::Deserialize;
use ndarray::{Array2, Array1};
use std::{
    env,
    error::Error,
    ffi::OsString,
    fs::File,
    process,
    str,
    path::Path,
};
use csv::ReaderBuilder;
#[derive(Debug, Deserialize)]
pub struct Track {
    pub genre: String,
    pub artist_name: String,
    pub track_name: String,
    pub track_id: String,
    pub popularity: u8,
    pub acousticness: f32,
    pub danceability: f32,
    pub duration_ms: u32,
    pub energy: f32,
    pub instrumentalness: f32,
    pub key: String,
    pub liveness: f32,
    pub loudness: f32,
    pub mode: String,
    pub speechiness: f32,
    pub tempo: f32,
    pub time_signature: String,
    pub valence: f32,
}

pub fn load_data<P: AsRef<Path>>(path: P) -> Result<Vec<Track>, Box<dyn Error>> {
    let file = File::open(path)?;
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);
    let mut tracks = Vec::new();
    for result in reader.deserialize()
    {
        let record: Track = result?;
        println!("{:?}", record);
        tracks.push(record);
    }
    Ok(tracks)
}