use std::fs::File;
use std::io::Write;
use crate::Track;
pub fn export_to_csv(tracks: &[Track], filename: &str, popular_threshold: u8)
{
    let mut file = File::create(filename).expect("can not make file");
    writeln!(file, "danceability,energy,instrumentalness,loudness,acousticness,valence,tempo,duration_ms,liveness,speechiness,popular")
        .expect("Failed to write header");
    for track in tracks {
        let label = if track.popularity > popular_threshold {1} else {0};
        writeln!(file, "{}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}",
            track.danceability,
            track.energy,
            track.instrumentalness,
            track.loudness,
            track.acousticness,
            track.valence,
            track.tempo,
            track.duration_ms,
            track.liveness,
            track.speechiness,
            label ).expect("Failed to write");
    }
}