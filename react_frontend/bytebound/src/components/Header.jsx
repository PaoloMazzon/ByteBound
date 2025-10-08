import React from 'react'
import {Code, CheckCircle} from 'lucide-react'
import useSubmitSolution from './hooks/useSubmitSolution'

export default function Header({ ram, setRam, cpu, setCpu, onSubmitSolution }) {
  const { handleSubmit } = useSubmitSolution();

  async function handleClick() {
    try {
      // Assuming you’ll later include `code` as well (for now it's not in Header)
      const solution = await handleSubmit({ ram, cpu });
      onSubmitSolution(solution);
    } catch (err) {
      console.error("Submission failed:", err);
      onSubmitSolution({ error: "Failed to fetch response" });
    }
  }

  return (
    <header className="bg-gray-800 border-b border-gray-700 px-6 py-4">
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-3">
          <Code className="text-green-400" size={28} />
          <h1 className="text-2xl font-bold text-white">ByteBound</h1>
        </div>

        <div className="flex items-center gap-4">
          <Input label="RAM (MB)" value={ram} onChange={setRam} />
          <Input label="CPU (MHz)" value={cpu} onChange={setCpu} />

          <button
            onClick={handleClick}
            className="flex items-center gap-2 bg-green-400 hover:bg-green-500 text-black px-6 py-2 rounded-lg font-medium transition-colors"
          >
            <CheckCircle size={20} />
            Submit
          </button>
        </div>
      </div>
    </header>
  );
}

function Input({label, value, onChange})
{
  return (
    <div className="flex items-center gap-2">
      <label className="text-gray-400 text-sm">{label}:</label>
      <input
        type="number"
        value={value}
        onChange={(e) => onChange(e.target.value)}
        className="w-24 bg-gray-700 text-white px-3 py-1.5 rounded border border-gray-600 focus:outline-none focus:border-green-400 text-sm"
      />
    </div>
  )  
}