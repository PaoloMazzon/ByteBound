import React, { useState, useRef, useEffect } from 'react';
import { Send, Bot, User, Code, CheckCircle } from 'lucide-react';
import './App.css'
import ProblemPanel from './components/ProblemPanel.jsx'
import preamble from './assets/preamble.js'
import q1 from './assets/q1.json'

import CodeOutputBox from './components/OutpuBox.jsx';

export default function ByteCode() {
  const [code, setCode] = useState(`int main() {
  // Write your solution here
  return 0;
  
}`);
  const [messages, setMessages] = useState([]);
  const [input, setInput] = useState('');
  const [isLoading, setIsLoading] = useState(false);
  const chatEndRef = useRef(null);
  const [ram, setRam] = useState(256000);
  const [cpu, setCpu] = useState(500);


  // Auto-scroll chat to bottom
  useEffect(() => {
    chatEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [messages]);

  const sendMessageToAI = async (message) => {
    if (!message.trim()) return;
    let prompt = preamble + '\nThis is the task information:\n' + JSON.stringify(q1) + '\nThis is the user\'s code:\n' + code + '\nThis is the RAM req in MB: ' + ram + '\nThis is the CPU req in MHz: ' + cpu
    console.log(prompt)
    // Add user message
    const userMessage = { text: message, isUser: true, timestamp: new Date() };
    setMessages(prev => [...prev, userMessage]);
    setInput('');
    setIsLoading(true);

    try {
      // Replace with your AI API endpoint
      const response = await fetch('http://ec2-3-129-9-220.us-east-2.compute.amazonaws.com:3000/ai', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({prompt: message})
      });

      if (response.ok) {
        const data = await response.json();
        const aiMessage = {
          text: data.response || 'No response from AI',
          isUser: false,
          timestamp: new Date()
        };
        setMessages(prev => [...prev, aiMessage]);
      } else {
        throw new Error('AI request failed');
      }
    } catch (error) {
      const errorMessage = {
        text: 'Sorry, I couldn\'t connect to the AI. Please try again.',
        isUser: false,
        timestamp: new Date()
      };
      setMessages(prev => [...prev, errorMessage]);
    } finally {
      setIsLoading(false);
    }
  };

  const handleSubmit = async () => {
    try {

      const requestBody = {
        constraints: { cpu: cpu, ram: ram * 1000 },
        code: code,
        challenge_name: "fib1"
      };

      const response = await fetch('http://ec2-3-129-9-220.us-east-2.compute.amazonaws.com:3000/submit', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(requestBody)
      });
      

      if (response.ok) {
        const data = await response.json();
        const soln = {
          compiled: data.compiled,
          errors: data.errors,
          runtime_us: data.runtime_us,
          success: data.success,
          test_cases: data.test_cases
        };
        console.log(data);
      } else {
        throw new Error('Server Call Failed');
      }
    } catch (error) {
      const soln = {
          compiled: false,
          errors: "",
          runtime_us: -1,
          success: false,
          test_cases: []
        };
        console.log("response not okay")
    }
  };

  const handleKeyPress = (e) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      sendMessageToAI(input);
    }
  };

  return (
    
    <div className="w-full h-screen flex flex-col">
      {/* Header */}
      {/*This is for header styling */}

      <header className="bg-gray-800 border-b border-gray-700 px-6 py-4">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-3">
            <Code className="text-green-400" size={28} />
            <h1 className="text-2xl font-bold text-white">ByteBound</h1>
          </div>
          
          <div className="flex items-center gap-4">
            {/* RAM Input */}
            <div className="flex items-center gap-2">
              <label className="text-gray-400 text-sm">RAM (MB):</label>
              <input
                type="number"
                value={ram}
                onChange={(e) => setRam(e.target.value)}
                placeholder="bytes"
                className="w-24 bg-gray-700 text-white px-3 py-1.5 rounded border border-gray-600 focus:outline-none focus:border-green-400 text-sm"
              />
            </div>
            
            {/* CPU Input */}
            <div className="flex items-center gap-2">
              <label className="text-gray-400 text-sm">CPU (MHz):</label>
              <input
                type="number"
                value={cpu}
                onChange={(e) => setCpu(e.target.value)}
                placeholder="MHz"
                className="w-24 bg-gray-700 text-white px-3 py-1.5 rounded border border-gray-600 focus:outline-none focus:border-green-400 text-sm"
              />
            </div>
            
            {/* Submit Button */}
            <button
              onClick={handleSubmit}
              className="flex items-center gap-2 bg-green-400 hover:bg-green-500 text-black px-6 py-2 rounded-lg font-medium transition-colors"
            >
              <CheckCircle size={20} />
              Submit
            </button>
          </div>
        </div>
      </header>

    {/* Main Content */}
      <div className="flex-1 flex overflow-hidden">
        {ProblemPanel(1)}
        {/* Middle Panel - Code Editor */}
        <div className="flex-1 flex flex-col bg-gray-900">
          <div className="bg-gray-800 px-4 py-2 border-b border-gray-700">
            <span className="text-sm text-gray-400">C</span>
          </div>
          <textarea
            defaultValue={code}
            onChange={(e) => setCode(e.target.value)}
            className="flex-1 bg-gray-900 text-gray-100 font-mono text-sm p-4 resize-none focus:outline-none"
            spellCheck="false"
            style={{ fontFamily: 'Monaco, Consolas, "Courier New", monospace' }}
          />
          <div>
            <CodeOutputBox soln={"Output"} />
          </div>
        </div>

        {/* Right Panel - AI Chat */}
        <div className="w-96 bg-gray-800 flex flex-col border-l border-gray-700">
          {/* Chat Header */}
          <div className="bg-gray-700 px-4 py-3 border-b border-gray-600">
            <div className="flex items-center gap-2">
              <Bot className="text-blue-400" size={20} />
              <h3 className="text-white font-semibold">AI Assistant</h3>
            </div>
          </div>

          {/* Messages */}
          <div className="flex-1 overflow-y-auto p-4 space-y-4">
            {messages.length === 0 ? (
              <div className="text-center text-gray-500 mt-8">
                <Bot size={48} className="mx-auto mb-3 opacity-50" />
                <p>Ask me anything about the problem!</p>
              </div>
            ) : (
              messages.map((msg, idx) => (
                <div
                  key={idx}
                  className={`flex gap-3 ${msg.isUser ? 'flex-row-reverse' : ''}`}
                >
                  <div className={`flex-shrink-0 w-8 h-8 rounded-full flex items-center justify-center ${
                    msg.isUser ? 'bg-green-600' : 'bg-blue-600'
                  }`}>
                    {msg.isUser ? <User size={18} /> : <Bot size={18} />}
                  </div>
                  <div className={`flex-1 ${msg.isUser ? 'text-right' : ''}`}>
                    <div className={`inline-block max-w-full px-4 py-2 rounded-lg ${
                      msg.isUser
                        ? 'bg-green-600 text-white'
                        : 'bg-gray-700 text-gray-100'
                    }`}>
                      <p className="text-sm whitespace-pre-wrap break-words">{msg.text}</p>
                    </div>
                    <p className="text-xs text-gray-500 mt-1">
                      {msg.timestamp.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })}
                    </p>
                  </div>
                </div>
              ))
            )}
            {isLoading && (
              <div className="flex gap-3">
                <div className="flex-shrink-0 w-8 h-8 rounded-full bg-blue-600 flex items-center justify-center">
                  <Bot size={18} />
                </div>
                <div className="bg-gray-700 px-4 py-2 rounded-lg">
                  <div className="flex gap-1">
                    <div className="w-2 h-2 bg-gray-400 rounded-full animate-bounce" style={{ animationDelay: '0ms' }}></div>
                    <div className="w-2 h-2 bg-gray-400 rounded-full animate-bounce" style={{ animationDelay: '150ms' }}></div>
                    <div className="w-2 h-2 bg-gray-400 rounded-full animate-bounce" style={{ animationDelay: '300ms' }}></div>
                  </div>
                </div>
              </div>
            )}
            <div ref={chatEndRef} />
          </div>

          {/* Input */}
          <div className="p-4 bg-gray-700 border-t border-gray-600">
            <div className="flex gap-2">
              <input
                type="text"
                value={input}
                onChange={(e) => setInput(e.target.value)}
                onKeyPress={handleKeyPress}
                placeholder="Ask AI for help..."
                className="flex-1 bg-gray-600 text-white px-4 py-2 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                disabled={isLoading}
              />
              <button
                onClick={() => sendMessageToAI(input)}
                disabled={isLoading || !input.trim()}
                className="bg-green-600 hover:bg-blue-700 disabled:bg-gray-600 disabled:cursor-not-allowed text-black p-2 rounded-lg transition-colors"
              >
                <Send size={20} />
              </button>
            </div>
          </div>
        </div>
      </div>
      </div>
      
  
  );
}
