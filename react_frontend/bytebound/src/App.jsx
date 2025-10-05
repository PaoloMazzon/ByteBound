import { useState,useEffect } from 'react'
import {Routes, Route, BrowserRouter} from 'react-router-dom'
import Problem from "./pages/Problem.jsx";
import './App.css'

function App() {
  const [count, setCount] = useState(0)

  return (
    <>
      
      <BrowserRouter>
        <Routes>
          <Route path = "/" element = {<Problem />} />
        </Routes>
      </BrowserRouter>
      
    
    </>
  )
}

export default App
