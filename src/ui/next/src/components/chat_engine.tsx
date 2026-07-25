import React, { useEffect, useState } from 'react';

export const ChatEngine = () => {
  const [messages, setMessages] = useState<string[]>([]);
  const [input, setInput] = useState('');

  useEffect(() => {
    const ws = new WebSocket(`ws://${window.location.host}/api/v1/chat_engine/ws`);
    ws.onmessage = (event) => {
      setMessages((prev) => [...prev, event.data]);
    };
    return () => ws.close();
  }, []);

  return (
    <div className="flex flex-col h-full bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800 rounded-lg shadow-sm">
      <div className="p-4 border-b border-gray-200 dark:border-gray-800 backdrop-blur-md bg-white/70 dark:bg-gray-900/70">
        <h2 className="text-lg font-semibold text-gray-900 dark:text-white">Unified Inbox</h2>
      </div>
      <div className="flex-1 p-4 overflow-y-auto space-y-4">
        {messages.map((m, i) => (
          <div key={i} className="bg-gray-100 dark:bg-gray-800 p-3 rounded-lg text-sm text-gray-900 dark:text-white">
            {m}
          </div>
        ))}
      </div>
      <div className="p-4 border-t border-gray-200 dark:border-gray-800 backdrop-blur-md bg-white/70 dark:bg-gray-900/70">
        <div className="flex space-x-2">
          <input
            type="text"
            className="flex-1 px-4 py-2 border border-gray-300 dark:border-gray-700 rounded-full focus:outline-none focus:ring-2 focus:ring-blue-500 bg-transparent text-gray-900 dark:text-white"
            placeholder="Type a message..."
            value={input}
            onChange={(e) => setInput(e.target.value)}
          />
          <button
            className="px-6 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-full font-medium transition-colors"
            onClick={() => {
              // implementation needed to send msg
              setInput('');
            }}
          >
            Send
          </button>
        </div>
      </div>
    </div>
  );
};
