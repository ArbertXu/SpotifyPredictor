mod data;
use data::{load_data, extractTrainData};
mod preprocess;
use preprocess::{prepare_data};
mod model;
use model::modelRun;
fn main() {
    let tracks = load_data("SpotifyFeatures.csv"); 
    let track_result = match tracks {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error loading data {}", e);
            return;
        }
    };
    let (x,y) = extractTrainData(&track_result);
    println!("Loaded {} songs with {} features each.", x.nrows(), x.ncols());
    println!("First label {:?}", y[0]);
    println!("First row values {:?}", x.row(0));
    
    let (train, valid) = prepare_data(x,y);
    modelRun(train, valid, &track_result);
    
    
}
