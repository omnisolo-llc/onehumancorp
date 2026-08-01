"use client";
import React, { useState, useEffect } from 'react';

export default function UnifiedChatWidget() {
  const [isOpen, setIsOpen] = useState(false);
  const [messages, setMessages] = useState<{sender: string, text: string}[]>([]);
  const [input, setInput] = useState('');

  // Fallback / mock real-time logic for web widget
  const handleSend = () => {
    if (!input.trim()) return;
    setMessages(prev => [...prev, { sender: 'You', text: input }]);
    setInput('');
    setTimeout(() => {
      setMessages(prev => [...prev, { sender: 'AI Agent', text: 'Thank you for reaching out! How can I help you today?' }]);
    }, 1000);
  };

  return (
    <div className="fixed bottom-4 right-4 z-50">
      {isOpen ? (
        <div
          className="w-[375px] h-[500px] flex flex-col rounded-[16px] overflow-hidden shadow-2xl"
          style={{
            background: 'rgba(255, 255, 255, 0.65)',
            backdropFilter: 'blur(30px) saturate(210%)',
            border: '1px solid rgba(255, 255, 255, 0.4)',
          }}
          data-testid="chat-widget-window"
        >
          <div className="bg-[#0066FF] text-white p-4 font-bold flex justify-between items-center rounded-t-[16px]">
            <span>OHC Support</span>
            <button onClick={() => setIsOpen(false)} className="text-white hover:opacity-80" data-testid="chat-widget-close">
              ✕
            </button>
          </div>
          <div className="flex-1 p-4 overflow-y-auto flex flex-col gap-2">
            {messages.map((msg, i) => (
              <div key={i} className={`p-2 rounded-[8px] max-w-[80%] ${msg.sender === 'You' ? 'bg-[#0066FF] text-white self-end' : 'bg-white/80 text-black self-start border border-black/10'}`}>
                {msg.text}
              </div>
            ))}
          </div>
          <div className="p-3 border-t border-black/10 bg-white/50 flex gap-2">
            <input
              type="text"
              value={input}
              onChange={(e) => setInput(e.target.value)}
              onKeyDown={(e) => e.key === 'Enter' && handleSend()}
              className="flex-1 rounded-[8px] border border-black/10 px-3 py-2 bg-white/80 outline-none focus:border-[#0066FF]"
              placeholder="Type a message..."
              data-testid="chat-widget-input"
            />
            <button
              onClick={handleSend}
              className="bg-[#0066FF] text-white px-4 py-2 rounded-[8px] font-semibold hover:bg-blue-600 transition-colors min-w-[44px] min-h-[44px]"
              data-testid="chat-widget-send"
            >
              Send
            </button>
          </div>
        </div>
      ) : (
        <button
          onClick={() => setIsOpen(true)}
          className="bg-[#0066FF] text-white w-14 h-14 rounded-full shadow-lg flex items-center justify-center hover:scale-105 transition-transform"
          data-testid="chat-widget-toggle"
          aria-label="Open chat"
        >
          <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8 10h.01M12 10h.01M16 10h.01M9 16H5a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v8a2 2 0 01-2 2h-5l-5 5v-5z" />
          </svg>
        </button>
      )}
    </div>
  );
}
