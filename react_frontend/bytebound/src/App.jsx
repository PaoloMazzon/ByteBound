import React, { useState } from 'react';
import './App.css';
import Header from './components/Header.jsx';
import ProblemPanel from './components/ProblemPanel.jsx';
import CodeEditor from './components/CodeEditor.jsx';
import CodeOutputBox from './components/CodeOutputBox.jsx';

export default function ByteCode() {
  const [code, setCode] = useState(`int fib(int n) {
    return 0;
}`);
  const [ram, setRam] = useState(256000);
  const [cpu, setCpu] = useState(500);
  const [solution, setSolution] = useState(null);

  return (
    <div className="w-full h-screen flex flex-col">
      <Header ram={ram} setRam={setRam} cpu={cpu} setCpu={setCpu} onSubmitSolution={(soln) => setSolution(soln)}/>
      
      <div className="flex-1 flex overflow-hidden">
        <ProblemPanel problemNumber={1} />

        <div className="flex-1 flex flex-col bg-gray-900">
          <CodeEditor code={code} setCode={setCode} />
          <CodeOutputBox soln={"Output"} />
        </div>
      </div>
    </div>
  );
}
