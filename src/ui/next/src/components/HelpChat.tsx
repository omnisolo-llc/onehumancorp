"use client";

import React, { useState, useRef, useEffect } from 'react';

type Message = {
  id: string;
  sender: 'user' | 'agent';
  text: string;
  link?: { url: string, title: string };
};

function isSafeLink(url: unknown): url is string {
  return typeof url === 'string' && (url.startsWith('/') || url.startsWith('https://') || url.startsWith('http://'));
}

function normalizeAgentReply(data: unknown): Pick<Message, 'text' | 'link'> {
  if (!data || typeof data !== 'object') {
    throw new Error('Invalid chat response');
  }

  const reply = 'reply' in data ? (data as { reply?: unknown }).reply : undefined;
  if (typeof reply !== 'string' || !reply.trim()) {
    throw new Error('Invalid chat reply');
  }

  const link = 'link' in data ? (data as { link?: unknown }).link : undefined;
  if (link && typeof link === 'object') {
    const candidate = link as { url?: unknown; title?: unknown };
    if (isSafeLink(candidate.url) && typeof candidate.title === 'string' && candidate.title.trim()) {
      return { text: reply, link: { url: candidate.url, title: candidate.title } };
    }
  }

  return { text: reply };
}

export function HelpChat() {
  const [isOpen, setIsOpen] = useState(false);
  const [messages, setMessages] = useState<Message[]>([
    { id: '1', sender: 'agent', text: "Hi! I'm your AI Help Agent. Need help setting up your store or understanding payments?" }
  ]);
  const [inputValue, setInputValue] = useState("");
  const messagesEndRef = useRef<HTMLDivElement>(null);
  const nextIdRef = useRef(2);

  const nextMessageId = (suffix: string) => `${Date.now()}-${nextIdRef.current++}-${suffix}`;

  const scrollToBottom = () => {
    if (messagesEndRef.current && typeof messagesEndRef.current.scrollIntoView === 'function') {
      messagesEndRef.current.scrollIntoView({ behavior: "smooth" });
    }
  };

  useEffect(() => {
    scrollToBottom();
  }, [messages, isOpen]);

  const handleSend = async (e?: React.FormEvent) => {
    e?.preventDefault();
    const messageText = inputValue.trim();
    if (!messageText) return;

    const userMessage: Message = { id: nextMessageId('user'), sender: 'user', text: messageText };
    setMessages(prev => [...prev, userMessage]);
    setInputValue("");
    try {
      const response = await fetch('/api/chat', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ message: messageText })
      });

      if (!response.ok) throw new Error("Failed to fetch");

      const data = await response.json();
      const reply = normalizeAgentReply(data);

      setMessages(prev => [...prev, {
        id: nextMessageId('agent'),
        sender: 'agent',
        ...reply
      }]);
    } catch (err) {
      setMessages(prev => [...prev, {
        id: nextMessageId('agent'),
        sender: 'agent',
        text: "Sorry, I'm having trouble connecting right now."
      }]);
    };
  };

  if (process.env.OHC_E2E === 'true') {
    return null; // Disable in E2E
  }

  return (
    <div className="help-chat-wrapper">
      {/* Floating Button */}
      <div className="fixed bottom-6 right-[5.5rem] z-50">
        {!isOpen && (
          <button
            onClick={() => setIsOpen(true)}
            className="bg-gray-900 text-white p-4 rounded-full shadow-2xl hover:shadow-xl hover:scale-105 transition-all flex items-center justify-center gap-2 group"
            aria-label="Open help chat"
          >
            <span className="text-xl">✨</span>
            <span className="font-outfit font-bold max-w-0 overflow-hidden group-hover:max-w-xs transition-all duration-300 whitespace-nowrap px-0 group-hover:px-2">Ask anything</span>
          </button>
        )}
      </div>

      {/* Chat Interface */}
      {isOpen && (
        <div className="fixed bottom-24 right-6 z-[60] w-[350px] max-w-[calc(100vw-48px)] bg-white/70 backdrop-blur-[20px] saturate-200 rounded-2xl shadow-2xl flex flex-col overflow-hidden border border-white/50 animate-slide-up-chat">
          {/* Header */}
          <div className="bg-gray-900/90 text-white p-4 flex justify-between items-center backdrop-blur-md">
            <div className="flex items-center gap-2">
              <span className="text-xl">✨</span>
              <div>
                <h3 className="font-bold font-outfit text-sm">Help Agent</h3>
                <p className="text-xs text-gray-300 font-inter">Always here to help</p>
              </div>
            </div>
            <button onClick={() => setIsOpen(false)} className="text-gray-400 hover:text-white transition-colors" aria-label="Close help chat">
              <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" /></svg>
            </button>
          </div>

          {/* Messages */}
          <div className="flex-1 p-4 overflow-y-auto h-[350px] bg-transparent flex flex-col gap-4 font-inter text-sm">
            {messages.map(msg => (
              <div key={msg.id} className={`flex flex-col ${msg.sender === 'user' ? 'items-end' : 'items-start'}`}>
                <div className={`px-4 py-2.5 rounded-2xl max-w-[85%] leading-relaxed ${
                  msg.sender === 'user'
                    ? 'bg-blue-600/90 backdrop-blur-md text-white rounded-br-sm shadow-sm'
                    : 'bg-white/80 backdrop-blur-md border border-white/50 text-gray-800 rounded-bl-sm shadow-sm'
                }`}>
                  {msg.text}
                </div>
                {msg.link && (
                  <a href={msg.link.url} className="mt-2 ml-1 text-blue-600 hover:text-blue-800 text-xs font-semibold hover:underline bg-blue-50/80 backdrop-blur-md px-3 py-1.5 rounded-full border border-blue-100 flex items-center shadow-sm">
                    {msg.link.title}
                  </a>
                )}
              </div>
            ))}
            <div ref={messagesEndRef} />
          </div>

          {/* Input */}
          <form onSubmit={handleSend} className="p-3 bg-white/50 backdrop-blur-md border-t border-white/30 flex gap-2">
            <input
              type="text"
              value={inputValue}
              onChange={(e) => setInputValue(e.target.value)}
              placeholder="Ask me anything..."
              className="flex-1 bg-white/60 backdrop-blur-md border border-white/50 rounded-xl px-4 py-2.5 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500/50 font-inter"
            />
            <button
              type="submit"
              disabled={!inputValue.trim()}
              className="bg-blue-600/90 backdrop-blur-md text-white p-2.5 rounded-xl disabled:opacity-50 disabled:cursor-not-allowed hover:bg-blue-700/90 transition-colors shadow-sm"
              aria-label="Send message"
            >
              <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 19l9 2-9-18-9 18 9-2zm0 0v-8" /></svg>
            </button>
          </form>
        </div>
      )}

      <style dangerouslySetInnerHTML={{__html: `
        @keyframes slide-up-chat {
          0% { opacity: 0; transform: translateY(20px) scale(0.95); }
          100% { opacity: 1; transform: translateY(0) scale(1); }
        }
        .animate-slide-up-chat { animation: slide-up-chat 0.2s cubic-bezier(0.16, 1, 0.3, 1) forwards; transform-origin: bottom right; }
      `}} />
    </div>
  );
}
