import { useState, useEffect } from 'react';
import q1 from '../assets/q1.json';
import q2 from '../assets/q2.json';
import q3 from '../assets/q3.json';

export default function ProblemPanel(problemNumber) {
  const [problemData, setProblemData] = useState(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);

  useEffect(() => {
    const loadProblem = async () => {
      try {
        let file_map = {
            1: q1,
            2: q2,
            3: q3
        }
        let data = file_map[problemNumber]
        setProblemData(data);
        setLoading(false);
      } catch (err) {
        setError(err.message);
        setLoading(false);
      }
    };

    loadProblem();
  }, []);

  if (loading) {
    return (
      <div className="flex-1 flex items-center justify-center bg-gray-800">
        <div className="text-white text-xl">Loading problem...</div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="flex-1 flex items-center justify-center bg-gray-800">
        <div className="text-red-400 text-xl">Error loading problem: {error}</div>
      </div>
    );
  }

  const { problem, examples, test_cases, difficulty, topics } = problemData;

  const difficultyColors = {
    Easy: 'bg-green-900 text-green-300',
    Medium: 'bg-yellow-900 text-yellow-300',
    Hard: 'bg-red-900 text-red-300'
  };

  return (
    <div className="flex-col flex-1 overflow-hidden">
      {/* Left Panel - Problem Description */}
      <div className="bg-gray-800 overflow-y-auto border-r border-gray-700">
        <div className="p-5">
          <h2 className="text-2xl font-bold text-white mb-4">{problem.title}</h2>
         
          <div className="mb-6">
            <span className={`inline-block px-3 py-1 rounded-full text-sm font-medium ${difficultyColors[difficulty]}`}>
              {difficulty}
            </span>
          </div>
          
          {topics && topics.length > 0 && (
            <div className="mb-6 flex flex-wrap gap-2">
              {topics.map((topic) => (
                <span key={topic} className="inline-block px-3 py-1 bg-gray-700 text-gray-300 rounded-full text-xs">
                  {topic}
                </span>
              ))}
            </div>
          )}
          
          <div className="text-gray-300 space-y-4">
            <p>{problem.description}</p>
            <p className="font-semibold text-white">{problem.question}</p>
            
            {examples.map((example, idx) => (
              <div key={idx} className="bg-gray-900 p-4 rounded-lg mt-4">
                <p className="text-white font-semibold mb-2">Example {idx + 1}:</p>
                <pre className="text-sm">
                  <div className="text-gray-400">Input:</div>
                  <div>{example.input}</div>
                  <div className="text-gray-400 mt-2">Output:</div>
                  <div>{example.output}</div>
                  {example.explanation && (
                    <>
                      <div className="text-gray-400 mt-2">Explanation:</div>
                      <div>{example.explanation}</div>
                    </>
                  )}
                </pre>
              </div>
            ))}
            
            {test_cases && test_cases.length > 0 && (
              <div className="mt-6">
                <p className="text-white font-semibold mb-2">Test Cases:</p>
                <div className="space-y-2 text-sm">
                  {test_cases.map((testCase, idx) => (
                    <div key={idx} className="bg-gray-900 p-3 rounded">
                      <span className="text-gray-400">Input: </span>
                      <span>n = {testCase.input}</span>
                      <span className="text-gray-400 ml-4">Output: </span>
                      <span>{testCase.output}</span>
                    </div>
                  ))}
                </div>
              </div>
            )}
          </div>
        </div>
      </div>
    </div>
  );
}