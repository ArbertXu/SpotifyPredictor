import { useState } from 'react';
import axios from 'axios';

function PredictForm() {
  const [trackName, setTrackName] = useState('');
  const [artistName, setArtistName] = useState('');
  const [result, setResult] = useState(null);

  const handleSubmit = async (e) => {
    e.preventDefault();
    try {
      const response = await axios.post('http://127.0.0.1:3001/predict', {
        track_name: trackName,
        artist_name: artistName,
      });
      setResult(response.data);
    } catch (err) {
      console.error('Prediction error:', err);
      setResult({ error: 'Could not get prediction' });
    }
  };

  return (
    <div>
      <h2>Predict Song Popularity</h2>
      <form onSubmit={handleSubmit}>
        <input
          type="text"
          placeholder="Track Name"
          value={trackName}
          onChange={(e) => setTrackName(e.target.value)}
        />
        <input
          type="text"
          placeholder="Artist Name"
          value={artistName}
          onChange={(e) => setArtistName(e.target.value)}
        />
        <button type="submit">Predict</button>
      </form>

      {result && (
        <div>
            <h3>Prediction Result</h3>
            <p><strong>Track:</strong> {result.track_name}</p>
            <p><strong>Artist:</strong> {result.artist_name}</p>
            <p><strong>Predicted:</strong> {result.predicted}</p>
            <p><strong>Actual:</strong> {result.actual}</p>

            {result.recommendations && result.recommendations.length > 0 && (
            <div>
                <h4>Similar Songs:</h4>
                <ul>
                {result.recommendations.map((rec, idx) => (
                    <li key={idx}>
                    {rec.track_name} by {rec.artist_name}
                    </li>
                ))}
                </ul>
            </div>
            )}
        </div>
    )}
    </div>
)}


export default PredictForm;