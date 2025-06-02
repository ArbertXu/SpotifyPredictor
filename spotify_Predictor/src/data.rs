use serde::Deserialize;
use ndarray::{Array2, Array1};
use std::{
    error::Error,
    fs::File,
    str,
    path::Path,
};
use csv::ReaderBuilder;
#[derive(Debug, Deserialize)]
 #[derive(Clone)]
pub struct Track {
    pub genre: String,
    pub artist_name: String,
    pub track_name: String,
    pub track_id: String,
    pub popularity: u8,
    pub acousticness: f32,
    pub danceability: f32,
    pub duration_ms: f32,
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
        // println!("{:?}", record);
        tracks.push(record);
    }
    Ok(tracks)
}

pub fn extract_train_data(tracks: &[Track]) -> (Array2<f32>, Array1<usize>) {
    

    let mut features = Vec::with_capacity(tracks.len());
    let mut labels = Vec::with_capacity(tracks.len());
    for track in tracks {
        features.push(vec![
            track.danceability,
             track.energy,
              track.instrumentalness,
            //    track.loudness,
                track.acousticness,
                 track.valence,
            //  track.tempo,
            // track.duration_ms,
            //  track.liveness,

              track.speechiness,
              ]);
        labels.push(if track.popularity > 40 {1} else {0});
    }
    let num_samples = features.len();
    let num_features = features[0].len();
    let features_array = Array2::from_shape_vec(
        (num_samples, num_features),
        features.into_iter().flatten().collect(),
    ).expect("Failed coversion to ndarray");
    let label_array = Array1::from(labels);
    return (features_array, label_array)
}