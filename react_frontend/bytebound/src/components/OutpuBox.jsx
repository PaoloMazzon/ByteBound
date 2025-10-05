import React from 'react';
import { Terminal, CheckCircle, XCircle, Clock, AlertCircle } from 'lucide-react';

const CodeOutputBox = ({ soln }) => {
  if (!soln) {
    return (
      <div className="bg-gray-900 rounded-lg overflow-hidden border border-gray-700 shadow-lg">
        <div className="bg-gray-800 px-4 py-2 border-b border-gray-700 flex items-center gap-2">
          <Terminal className="w-4 h-4 text-gray-400" />
          <span className="text-sm text-gray-300 font-semibold">Output</span>
        </div>
        <div className="bg-gray-900 text-gray-400 p-4 font-mono text-sm">
          No output yet...
        </div>
      </div>
    );
  }

  const { compiled, errors, runtime_us, success, test_cases } = soln;

  return (
    <div className="bg-gray-900 rounded-lg overflow-hidden border border-gray-700 shadow-lg">
      {/* Header */}
      <div className="bg-gray-800 px-4 py-2 border-b border-gray-700 flex items-center justify-between">
        <div className="flex items-center gap-2">
          <Terminal className="w-4 h-4 text-green-400" />
          <span className="text-sm text-gray-300 font-semibold">Execution Results</span>
        </div>
        <div className="flex items-center gap-2">
          {success ? (
            <CheckCircle className="w-4 h-4 text-green-400" />
          ) : (
            <XCircle className="w-4 h-4 text-red-400" />
          )}
          <span className={`text-xs font-semibold ${success ? 'text-green-400' : 'text-red-400'}`}>
            {success ? 'SUCCESS' : 'FAILED'}
          </span>
        </div>
      </div>
      
      {/* Output Content */}
      <div className="bg-gray-900 p-4 font-mono text-sm overflow-auto max-h-96">
        {/* Compilation Status */}
        <div className="mb-4">
          <div className="flex items-center gap-2 mb-2">
            {compiled ? (
              <CheckCircle className="w-4 h-4 text-green-400" />
            ) : (
              <XCircle className="w-4 h-4 text-red-400" />
            )}
            <span className={compiled ? 'text-green-400' : 'text-red-400'}>
              Compilation: {compiled ? 'Success' : 'Failed'}
            </span>
          </div>
        </div>

        {/* Errors */}
        {errors && errors.length > 0 && (
          <div className="mb-4 bg-red-950 border border-red-800 rounded p-3">
            <div className="flex items-center gap-2 mb-2">
              <AlertCircle className="w-4 h-4 text-red-400" />
              <span className="text-red-400 font-semibold">Errors:</span>
            </div>
            <pre className="text-red-300 whitespace-pre-wrap text-xs">
              {errors}
            </pre>
          </div>
        )}

        {/* Runtime */}
        <div className="mb-4 flex items-center gap-2">
          <Clock className="w-4 h-4 text-blue-400" />
          <span className="text-gray-300">Runtime:</span>
          <span className="text-blue-400">{runtime_us ? `${runtime_us} μs` : 'N/A'}</span>
        </div>

        {/* Test Cases */}
        {test_cases && test_cases.length > 0 && (
          <div>
            <div className="text-gray-300 font-semibold mb-3">Test Cases:</div>
            <div className="space-y-2">
              {test_cases.map((testCase, index) => (
                <div
                  key={index}
                  className={`p-3 rounded border ${
                    testCase.passed
                      ? 'bg-green-950 border-green-800'
                      : 'bg-red-950 border-red-800'
                  }`}
                >
                  <div className="flex items-center justify-between mb-2">
                    <span className="text-gray-300 font-semibold">Test {index + 1}</span>
                    {testCase.passed ? (
                      <span className="text-green-400 text-xs font-semibold">PASSED</span>
                    ) : (
                      <span className="text-red-400 text-xs font-semibold">FAILED</span>
                    )}
                  </div>
                  {testCase.input && (
                    <div className="text-xs mb-1">
                      <span className="text-gray-400">Input:</span>
                      <span className="text-gray-300 ml-2">{testCase.input}</span>
                    </div>
                  )}
                  {testCase.expected && (
                    <div className="text-xs mb-1">
                      <span className="text-gray-400">Expected:</span>
                      <span className="text-gray-300 ml-2">{testCase.expected}</span>
                    </div>
                  )}
                  {testCase.actual && (
                    <div className="text-xs">
                      <span className="text-gray-400">Actual:</span>
                      <span className={testCase.passed ? 'text-green-400 ml-2' : 'text-red-400 ml-2'}>
                        {testCase.actual}
                      </span>
                    </div>
                  )}
                </div>
              ))}
            </div>
          </div>
        )}
      </div>
    </div>
  );
};

export default CodeOutputBox;