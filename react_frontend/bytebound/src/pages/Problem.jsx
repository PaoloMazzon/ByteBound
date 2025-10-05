import {useState, useEffect} from 'react'
import {useParams} from 'react-router-dom'
import Navbar from '../components/Navbar.jsx'

 const Problem = () => {
    const [text, setText] = useState("");

    return(
        <div>
            <Navbar />
            <h1>Problem Page</h1>
            <p>This is the problem page.</p>

        </div>
    )
}

export default Problem;