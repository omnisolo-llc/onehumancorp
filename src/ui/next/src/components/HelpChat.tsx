"use client";

import React, { useState, useRef, useEffect } from 'react';

type Message = {
  id: string;
  sender: 'user' | 'agent';
  text: string;
  link?: { url: string, title: string };
};

export function HelpChat() {
  const [isOpen, setIsOpen] = useState(false);
  const [messages, setMessages] = useState<Message[]>([
    { id: '1', sender: 'agent', text: "Hi! I'm your AI Help Agent. Need help setting up your store or understanding payments?" }
  ]);
  const [inputValue, setInputValue] = useState("");
  const messagesEndRef = useRef<HTMLDivElement>(null);

  const scrollToBottom = () => {
    messagesEndRef.current?.scrollIntoView({ behavior: "smooth" });
  };

  useEffect(() => {
    scrollToBottom();
  }, [messages, isOpen]);

  const handleSend = async (e?: React.FormEvent) => {
    e?.preventDefault();
    if (!inputValue.trim()) return;

    const userMessage: Message = { id: Date.now().toString(), sender: 'user', text: inputValue };
    setMessages(prev => [...prev, userMessage]);
    setInputValue("");
    try {
      const response = await fetch('/api/chat', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ message: inputValue })
      });

      if (!response.ok) throw new Error("Failed to fetch");

      const data = await response.json();

      setMessages(prev => [...prev, {
        id: (Date.now() + 1).toString(),
        sender: 'agent',
        text: data.reply,
        link: data.link
      }]);
    } catch (err) {
      setMessages(prev => [...prev, {
        id: (Date.now() + 1).toString(),
        sender: 'agent',
        text: "Sorry, I'm having trouble connecting right now."
      }]);
    };
  };

  if (process.env.NEXT_PUBLIC_E2E === 'true') {
    return null; // Disable in E2E
  }

  return (
    <div className="help-chat-wrapper">
      {/* Floating Button */}
      <div className="fixed bottom-6 right-[5.5rem] z-50">
        {!isOpen && (
          <button
            onClick={() => setIsOpen(true)}
            className="bg-[#1D1D1F] text-white p-4 rounded-full shadow-2xl hover:shadow-xl hover:scale-105 transition-all flex items-center justify-center gap-2 group border border-white/10"
          >
            <span className="text-xl">✨</span>
            <span className="font-outfit font-bold max-w-0 overflow-hidden group-hover:max-w-xs transition-all duration-300 whitespace-nowrap px-0 group-hover:px-2">Ask anything</span>
          </button>
        )}
      </div>

      {/* Chat Interface */}
      {isOpen && (
        <div className="fixed bottom-24 right-6 z-[60] w-[380px] max-w-[calc(100vw-48px)] bg-white/70 backdrop-blur-[30px] saturate-[180%] rounded-3xl shadow-2xl flex flex-col overflow-hidden border border-white/50 animate-slide-up-chat">
          {/* Header */}
          <div className="bg-[#1D1D1F]/90 text-white p-6 flex justify-between items-center backdrop-blur-md">
            <div className="flex items-center gap-3">
              <div className="w-10 h-10 bg-white/10 rounded-xl flex items-center justify-center text-xl backdrop-blur-sm border border-white/10">✨</div>
              <div>
                <h3 className="font-bold font-outfit text-lg leading-tight">Help Agent</h3>
                <p className="text-xs text-gray-400 font-inter font-medium">Always here to help</p>
              </div>
            </div>
            <button onClick={() => setIsOpen(false)} className="text-gray-400 hover:text-white transition-colors bg-white/5 p-2 rounded-full">
              <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" /></svg>
            </button>
          </div>

          {/* Messages */}
          <div className="flex-1 p-6 overflow-y-auto h-[400px] bg-transparent flex flex-col gap-6 font-inter text-sm">
            {messages.map(msg => (
              <div key={msg.id} className={`flex flex-col ${msg.sender === 'user' ? 'items-end' : 'items-start'}`}>
                <div className={`px-5 py-3.5 rounded-2xl max-w-[85%] leading-relaxed font-medium shadow-sm ${
                  msg.sender === 'user'
                    ? 'bg-blue-600 text-white rounded-tr-none'
                    : 'bg-white/90 border border-gray-100 text-[#1D1D1F] rounded-tl-none'
                }`}>
                  {msg.text}
                </div>
                {msg.link && (
                  <a href={msg.link.url} className="mt-3 text-blue-600 hover:text-blue-800 text-xs font-bold hover:underline bg-blue-50/80 backdrop-blur-md px-4 py-2 rounded-full border border-blue-100 flex items-center shadow-sm transition-all hover:scale-105 active:scale-95">
                    {msg.link.title}
                  </a>
                )}
              </div>
            ))}
            <div ref={messagesEndRef} />
          </div>

          {/* Input */}
          <form onSubmit={handleSend} className="p-4 bg-white/40 backdrop-blur-md border-t border-white/30 flex gap-3">
            <input
              type="text"
              value={inputValue}
              onChange={(e) => setInputValue(e.target.value)}
              placeholder="Ask me anything..."
              className="flex-1 bg-white/60 backdrop-blur-md border border-gray-200 rounded-2xl px-5 py-3.5 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500/50 font-inter font-medium shadow-inner"
            />
            <button
              type="submit"
              disabled={!inputValue.trim()}
              className="bg-blue-600 text-white p-3.5 rounded-2xl disabled:opacity-50 disabled:cursor-not-allowed hover:bg-blue-700 transition-all shadow-md active:scale-95"
            >
              <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 19l9 2-9-18-9 18 9-2zm0 0v-8" /></svg>
            </button>
          </form>
        </div>
      )}

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
        @keyframes slide-up-chat {
          0% { opacity: 0; transform: translateY(20px) scale(0.95); }
          100% { opacity: 1; transform: translateY(0) scale(1); }
        }
        .animate-slide-up-chat { animation: slide-up-chat 0.3s cubic-bezier(0.16, 1, 0.3, 1) forwards; transform-origin: bottom right; }
      `}} />
    </div>
  );
}
