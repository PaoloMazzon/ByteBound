    // CodeOutputBox.jsx
    import React from 'react';

    const CodeOutputBox = ({ code }) => {
      return (
        <div className="bg-gray-800 text-white p-4 rounded-md font-mono overflow-auto h-50">
          <pre>
            <code>{code}</code>
          </pre>
        </div>
      );
    };

    export default CodeOutputBox;