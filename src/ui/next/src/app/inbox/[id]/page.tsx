'use client';

import React, { useState, useEffect } from 'react';
import Link from 'next/link';
import { useParams } from 'next/navigation';

interface Message {
  id: string;
  content: string;
  senderType: 'customer' | 'agent';
  timestamp: string;
}

export default function ConversationThread() {
  const params = useParams();
  const id = params.id as string;
  const [messages, setMessages] = useState<Message[]>([]);
  const [input, setInput] = useState('');

  useEffect(() => {
    // In a real app, this would fetch messages from the Rust backend
    setMessages([
      {
        id: 'm1',
        content: 'Hi, do you do custom vegan cakes?',
        senderType: 'customer',
        timestamp: '10:00 AM',
      },
    ]);
  }, [id]);

  const handleSend = () => {
    if (!input.trim()) return;
    setMessages([
      ...messages,
      {
        id: Date.now().toString(),
        content: input,
        senderType: 'agent',
        timestamp: new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' }),
      },
    ]);
    setInput('');
  };

  const handleAiSuggest = () => {
    setInput('Hi there! Yes, we do offer custom vegan cakes. What kind of flavors were you thinking of?');
  };

  return (
    <div className="flex flex-col h-screen w-full max-w-[375px] mx-auto bg-gray-50 text-black">
      <header className="px-4 py-3 bg-white/80 backdrop-blur-md border-b sticky top-0 z-10 flex items-center">
        <Link href="/inbox" className="mr-3 text-blue-500 hover:text-blue-700">
          &larr; Back
        </Link>
        <div>
          <h1 className="text-lg font-bold">Maya Baker</h1>
          <p className="text-xs text-gray-500">Requested vegan cake quote</p>
        </div>
      </header>

      <main className="flex-1 overflow-y-auto p-4 space-y-3">
        {messages.map((msg) => (
          <div
            key={msg.id}
            className={`flex ${msg.senderType === 'agent' ? 'justify-end' : 'justify-start'}`}
          >
            <div
              className={`max-w-[80%] p-3 rounded-2xl ${
                msg.senderType === 'agent'
                  ? 'bg-blue-500 text-white rounded-br-none'
                  : 'bg-white border rounded-bl-none'
              }`}
            >
              <p className="text-sm">{msg.content}</p>
              <span className={`text-[10px] block mt-1 ${msg.senderType === 'agent' ? 'text-blue-100' : 'text-gray-400'}`}>
                {msg.timestamp}
              </span>
            </div>
          </div>
        ))}
      </main>

      <footer className="p-3 bg-white/90 backdrop-blur-md border-t flex flex-col space-y-2 sticky bottom-0">
        <button
          onClick={handleAiSuggest}
          className="self-start text-xs font-medium text-purple-600 bg-purple-50 px-3 py-1 rounded-full border border-purple-100 flex items-center hover:bg-purple-100 transition"
        >
          ✨ AI Suggest
        </button>
        <div className="flex space-x-2 items-center">
          <input
            type="text"
            className="flex-1 border rounded-full px-4 py-2 text-sm focus:outline-none focus:border-blue-500"
            placeholder="Type a message..."
            value={input}
            onChange={(e) => setInput(e.target.value)}
            onKeyDown={(e) => e.key === 'Enter' && handleSend()}
          />
          <button
            onClick={handleSend}
            className="bg-blue-500 text-white rounded-full w-9 h-9 flex items-center justify-center hover:bg-blue-600 transition"
          >
            &uarr;
          </button>
        </div>
      </footer>
    </div>
  );
}
