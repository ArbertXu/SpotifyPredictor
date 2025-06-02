use std::collections::HashMap;
use crate::data::Track;

type SongKey = (String, String);
pub type TrackMap = HashMap<SongKey, Track>;

pub fn build_track_map(tracks: &[Track]) -> TrackMap {
    tracks.iter().map(|t|  {
        ( ( t.track_name.to_lowercase(), t.artist_name.to_lowercase()), t.clone(),)
    }).collect()
}

pub fn find_track<'a> (map: &'a TrackMap, track_name: &str, artist_name: &str) -> Option<&'a Track> {
    let key = (track_name.to_lowercase(), artist_name.to_lowercase());
    map.get(&key)
}