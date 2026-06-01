"use client";

import React, { useState, useEffect, useRef } from 'react';

interface ChatMessage {
  role: 'user' | 'agent';
  content: string;
}

export default function Booking() {
  const [messages, setMessages] = useState<ChatMessage[]>([
    { role: 'agent', content: 'Hello! I am your scheduling assistant. What time would you like to book an appointment?' }
  ]);
  const [input, setInput] = useState('');
  const [isLoading, setIsLoading] = useState(false);
  const messagesEndRef = useRef<HTMLDivElement>(null);

  const scrollToBottom = () => {
    messagesEndRef.current?.scrollIntoView({ behavior: "smooth" });
  };

  useEffect(() => {
    scrollToBottom();
  }, [messages]);

  const handleSend = async () => {
    if (!input.trim() || isLoading) return;

    const userMessage: ChatMessage = { role: 'user', content: input.trim() };
    setMessages(prev => [...prev, userMessage]);
    setInput('');
    setIsLoading(true);

    try {
      const res = await fetch('/api/booking/chat', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ message: userMessage.content })
      });

      if (!res.ok) throw new Error('Network response was not ok');

      const data = await res.json();

      setMessages(prev => [...prev, { role: 'agent', content: data.reply }]);
    } catch (error) {
      console.error('Failed to send message:', error);
      setMessages(prev => [...prev, { role: 'agent', content: 'Sorry, I encountered an error connecting to the scheduling system.' }]);
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <div className="min-h-screen bg-[#F5F5F7] dark:bg-[#16161a] font-inter flex flex-col justify-center px-4 py-8 sm:px-6 lg:px-8">
      <div className="w-full max-w-[375px] mx-auto mac-glass-container rounded-[16px] shadow-lg overflow-hidden flex flex-col h-[650px] relative border border-white/50 dark:border-white/10">

        {/* Header */}
        <div className="p-4 border-b border-white/50 dark:border-white/10 bg-white/60 dark:bg-black/30 backdrop-blur-md flex items-center justify-between z-10">
          <div>
             <h2 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7]">Booking Agent</h2>
             <p className="text-xs text-[#34C759] font-medium flex items-center gap-1">
               <span className="w-2 h-2 rounded-full bg-[#34C759] animate-pulse"></span> Online
             </p>
          </div>
        </div>

        {/* Chat Area */}
        <div className="flex-1 p-4 overflow-y-auto space-y-4 flex flex-col bg-transparent">
          {messages.map((msg, index) => (
            <div key={index} className={`flex ${msg.role === 'user' ? 'justify-end' : 'justify-start'}`}>
              <div className={`max-w-[80%] rounded-2xl p-3 text-sm shadow-sm transition-all ${
                msg.role === 'user'
                  ? 'bg-[#0066FF] text-white rounded-br-sm'
                  : 'bg-white dark:bg-gray-800 text-[#1D1D1F] dark:text-[#F5F5F7] rounded-bl-sm border border-gray-100 dark:border-gray-700'
              }`}>
                {msg.content}
              </div>
            </div>
          ))}
          {isLoading && (
            <div className="flex justify-start">
               <div className="bg-white dark:bg-gray-800 border border-gray-100 dark:border-gray-700 rounded-2xl rounded-bl-sm p-4 shadow-sm flex items-center gap-1">
                 <div className="w-2 h-2 bg-gray-400 rounded-full animate-bounce"></div>
                 <div className="w-2 h-2 bg-gray-400 rounded-full animate-bounce" style={{ animationDelay: '0.2s' }}></div>
                 <div className="w-2 h-2 bg-gray-400 rounded-full animate-bounce" style={{ animationDelay: '0.4s' }}></div>
               </div>
            </div>
          )}
          <div ref={messagesEndRef} />
        </div>

        {/* Input Area */}
        <div className="p-4 bg-white/60 dark:bg-black/30 backdrop-blur-md border-t border-white/50 dark:border-white/10 z-10">
          <div className="flex items-center gap-2 relative">
            <input
              type="text"
              value={input}
              onChange={(e) => setInput(e.target.value)}
              onKeyDown={(e) => e.key === 'Enter' && handleSend()}
              placeholder="Message..."
              className="flex-1 bg-white dark:bg-[#2C2C2E] text-[#1D1D1F] dark:text-[#F5F5F7] rounded-full px-4 py-3 outline-none border border-gray-200 dark:border-gray-600 focus:border-[#0066FF] transition-all text-sm pr-12 shadow-inner"
            />
            <button
              onClick={handleSend}
              disabled={!input.trim() || isLoading}
              className="absolute right-1 top-1/2 -translate-y-1/2 w-8 h-8 rounded-full bg-[#0066FF] text-white flex items-center justify-center disabled:opacity-50 disabled:cursor-not-allowed hover:bg-[#0052cc] transition-colors"
            >
              <svg className="w-4 h-4 ml-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 12h14M12 5l7 7-7 7" /></svg>
            </button>
          </div>
        </div>

      </div>
    </div>
  );
}
