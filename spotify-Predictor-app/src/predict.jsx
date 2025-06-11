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
    <div className="flex flex-col items-center">
      <h2 className='text-white justify-center text-4xl font-bold p-5'>Predict Song Popularity </h2>
      <form className='flex flex-col space-y-5 bg-[#0A1128] w-full max-w-sm  border-5 border-[#0A1128] rounded-lg'  onSubmit={handleSubmit}>
        <input className='bg-[#0A1128] min-w-3xs text-center'
          type="text"
          placeholder="Track Name"
          value={trackName}
          onChange={(e) => setTrackName(e.target.value)}
        />
        <input className='bg-[#0A1128] min-w-xs text-center'
          type="text"
          placeholder="Artist Name"
          value={artistName}
          onChange={(e) => setArtistName(e.target.value)}
        />
        <button className='bg-[#0A1128] min-w-xs hover:bg-cyan-400' type="submit">Predict</button>
      </form>
      

      {result && (
        <div className='p-5'>
            <h3 className='text-xl font-bold'>Prediction Result</h3>
            <p><strong>Track:</strong> {result.track_name}</p>
            <p><strong>Artist:</strong> {result.artist_name}</p>
            <p><strong>Predicted:</strong> {result.predicted}</p>
            <p><strong>Actual:</strong> {result.actual}</p>

            {result.recommendations && result.recommendations.length > 0 && (
            <div className='p-5'>
                <h4 className='text-white'>Similar Songs:</h4>
                <ul>
                {result.recommendations.map((rec, idx) => (
                    <li className='text-white p-2' key={idx}>
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