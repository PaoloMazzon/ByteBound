export default function CodeEditor({ code, setCode }) {
  return (
    <>
      <div className="bg-gray-800 px-4 py-2 border-b border-gray-700">
        <span className="text-sm text-gray-400">C</span>
      </div>
      <textarea
        value={code}
        onChange={(e) => setCode(e.target.value)}
        className="flex-1 bg-gray-900 text-gray-100 font-mono text-sm p-4 resize-none focus:outline-none"
        spellCheck="false"
        style={{ fontFamily: 'Monaco, Consolas, "Courier New", monospace' }}
      />
    </>
  );
}
