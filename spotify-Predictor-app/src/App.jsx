import { useState } from 'react'
import './App.css'
import PredictForm from './predict'

function App() {
  const [count, setCount] = useState(0)

  return (
    <>
      <div>
        <PredictForm/>
      </div>
    </>
  )
}

export default App
