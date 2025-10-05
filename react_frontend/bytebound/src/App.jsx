import React, { useState, useRef, useEffect } from 'react';
import { Send, Bot, User, Code, CheckCircle } from 'lucide-react';
import './App.css'

export default function LeetCodeClone() {
  const [code, setCode] = useState(`function twoSum(nums, target) {
  // Write your solution here
  
}`);
  const [messages, setMessages] = useState([]);
  const [input, setInput] = useState('');
  const [isLoading, setIsLoading] = useState(false);
  const chatEndRef = useRef(null);

  // Auto-scroll chat to bottom
  useEffect(() => {
    chatEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [messages]);

  const sendMessageToAI = async (message) => {
    if (!message.trim()) return;

    // Add user message
    const userMessage = { text: message, isUser: true, timestamp: new Date() };
    setMessages(prev => [...prev, userMessage]);
    setInput('');
    setIsLoading(true);

    try {
      // Replace with your AI API endpoint
      const response = await fetch('http://localhost:3001/chat', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ message, code })
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

  const handleSubmit = () => {
    alert('Code submitted!\n\n' + code);
  };

  const handleKeyPress = (e) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      sendMessageToAI(input);
    }
  };

  return (
    <div className="h-screen flex flex-col bg-gray-900">
      {/* Header */}
      <header className="bg-gray-800 border-b border-gray-700 px-6 py-4">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-3">
            <Code className="text-green-400" size={28} />
            <h1 className="text-2xl font-bold text-white">CodeChallenge</h1>
          </div>
          <button
            onClick={handleSubmit}
            className="flex items-center gap-2 bg-green-600 hover:bg-green-700 text-white px-6 py-2 rounded-lg font-medium transition-colors"
          >
            <CheckCircle size={20} />
            Submit
          </button>
        </div>
      </header>

      {/* Main Content */}
      <div className="flex-1 flex overflow-hidden">
        {/* Left Panel - Problem Description */}
        <div className="w-1/3 bg-gray-800 overflow-y-auto border-r border-gray-700">
          <div className="p-6">
            <h2 className="text-2xl font-bold text-white mb-4">1. Two Sum</h2>
            
            <div className="mb-6">
              <span className="inline-block px-3 py-1 bg-green-900 text-green-300 rounded-full text-sm font-medium">
                Easy
              </span>
            </div>

            <div className="text-gray-300 space-y-4">
              <p>
                Given an array of integers <code className="bg-gray-700 px-2 py-1 rounded">nums</code> and 
                an integer <code className="bg-gray-700 px-2 py-1 rounded">target</code>, return indices 
                of the two numbers such that they add up to target.
              </p>

              <p>
                You may assume that each input would have exactly one solution, and you may not use 
                the same element twice.
              </p>

              <p>You can return the answer in any order.</p>

              <div className="bg-gray-900 p-4 rounded-lg mt-6">
                <p className="text-white font-semibold mb-2">Example 1:</p>
                <pre className="text-sm">
                  <div className="text-gray-400">Input:</div>
                  <div>nums = [2,7,11,15], target = 9</div>
                  <div className="text-gray-400 mt-2">Output:</div>
                  <div>[0,1]</div>
                  <div className="text-gray-400 mt-2">Explanation:</div>
                  <div>Because nums[0] + nums[1] == 9, we return [0, 1].</div>
                </pre>
              </div>

              <div className="bg-gray-900 p-4 rounded-lg mt-4">
                <p className="text-white font-semibold mb-2">Example 2:</p>
                <pre className="text-sm">
                  <div className="text-gray-400">Input:</div>
                  <div>nums = [3,2,4], target = 6</div>
                  <div className="text-gray-400 mt-2">Output:</div>
                  <div>[1,2]</div>
                </pre>
              </div>

              <div className="mt-6">
                <p className="text-white font-semibold mb-2">Constraints:</p>
                <ul className="list-disc list-inside space-y-1 text-sm">
                  <li>2 ≤ nums.length ≤ 10⁴</li>
                  <li>-10⁹ ≤ nums[i] ≤ 10⁹</li>
                  <li>-10⁹ ≤ target ≤ 10⁹</li>
                  <li>Only one valid answer exists.</li>
                </ul>
              </div>
            </div>
          </div>
        </div>

        {/* Middle Panel - Code Editor */}
        <div className="flex-1 flex flex-col bg-gray-900">
          <div className="bg-gray-800 px-4 py-2 border-b border-gray-700">
            <span className="text-sm text-gray-400">JavaScript</span>
          </div>
          <textarea
            value={code}
            onChange={(e) => setCode(e.target.value)}
            className="flex-1 bg-gray-900 text-gray-100 font-mono text-sm p-4 resize-none focus:outline-none"
            spellCheck="false"
            style={{ fontFamily: 'Monaco, Consolas, "Courier New", monospace' }}
          />
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
                className="bg-blue-600 hover:bg-blue-700 disabled:bg-gray-600 disabled:cursor-not-allowed text-white p-2 rounded-lg transition-colors"
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