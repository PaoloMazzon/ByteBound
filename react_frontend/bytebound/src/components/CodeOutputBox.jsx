export default function CodeOutputBox({ solution }) {
  if (!solution) {
    return (
      <div className="flex-1 bg-gray-900 text-gray-400 p-6">
        Submit a solution to see output here.
      </div>
    );
  }

  return (
    <div className="flex-1 bg-gray-900 text-gray-100 p-6 overflow-auto">
      <h2 className="text-xl font-bold mb-3">Compiler Output</h2>
      <pre className="bg-gray-800 p-3 rounded mb-3">{solution.c_stdout}</pre>
      {solution.c_stderr && (
        <pre className="bg-red-800 p-3 rounded mb-3">{solution.c_stderr}</pre>
      )}

      <h2 className="text-xl font-bold mb-3">Program Output</h2>
      <pre className="bg-gray-800 p-3 rounded mb-3">{solution.r_stdout}</pre>
      {solution.r_stderr && (
        <pre className="bg-red-800 p-3 rounded mb-3">{solution.r_stderr}</pre>
      )}

      <h2 className="text-xl font-bold mb-3">Runtime</h2>
      <p>{solution.runtime_us} μs</p>

      {solution.test_cases?.length > 0 && (
        <>
          <h2 className="text-xl font-bold mt-4 mb-2">Test Cases</h2>
          <ul className="list-disc list-inside">
            {solution.test_cases.map((t, i) => (
              <li key={i}>{JSON.stringify(t)}</li>
            ))}
          </ul>
        </>
      )}
    </div>
  );
}
