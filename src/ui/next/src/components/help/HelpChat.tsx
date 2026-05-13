'use client';

import React, { useState, useRef, useEffect } from 'react';

interface Message {
  id: string;
  role: 'user' | 'assistant';
  content: string;
  link?: string;
}

export function HelpChat({ inModal = false }: { inModal?: boolean }) {
  const [messages, setMessages] = useState<Message[]>([
    { id: '1', role: 'assistant', content: 'Hi there! I am your AI Help Agent. I can answer any questions about using OneHumanCorp. What do you need help with today?' }
  ]);
  const [input, setInput] = useState('');
  const [isTyping, setIsTyping] = useState(false);
  const endRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    endRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [messages, isTyping]);

  const handleSend = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!input.trim()) return;

    const userMsg: Message = { id: Date.now().toString(), role: 'user', content: input };
    setMessages(prev => [...prev, userMsg]);
    setInput('');
    setIsTyping(true);

    // Simulate AI response based on keywords
    setTimeout(() => {
      setIsTyping(false);
      let reply = "I'm not exactly sure, but you might find the answer in our Help Center articles.";
      let link = undefined;

      const q = userMsg.content.toLowerCase();
      if (q.includes('payment') || q.includes('money')) {
        reply = "To accept payments, go to the Payments tab and enter the customer's email to send a secure link. It's very simple!";
        link = "/help/articles/payments";
      } else if (q.includes('store') || q.includes('setup')) {
        reply = "Setting up your store involves connecting your bank account and adding products. It only takes a few minutes.";
        link = "/help/articles/getting-started";
      } else if (q.includes('agent') || q.includes('ai')) {
        reply = "You can activate your own AI Support Agent from the 'AI Agents' menu. It will use your website's info to talk to customers.";
        link = "/help/articles/ai-agents";
      }

      setMessages(prev => [...prev, { id: Date.now().toString(), role: 'assistant', content: reply, link }]);
    }, 1200);
  };

  return (
    <div className={`flex flex-col ${inModal ? 'h-full bg-slate-50/50' : 'fixed bottom-20 right-4 w-80 h-96 bg-white/90 backdrop-blur-xl saturate-200 shadow-2xl rounded-2xl border border-slate-200 z-50 overflow-hidden'}`}>
      {!inModal && (
        <div className="bg-blue-600 text-white p-3 flex items-center justify-between shadow-md">
          <div className="flex items-center gap-2">
            <div className="w-2 h-2 bg-green-400 rounded-full animate-pulse"></div>
            <span className="font-medium text-sm" style={{ fontFamily: 'Outfit, sans-serif' }}>Ask Anything</span>
          </div>
        </div>
      )}

      <div className="flex-1 overflow-y-auto p-4 space-y-4">
        {messages.map(msg => (
          <div key={msg.id} className={`flex ${msg.role === 'user' ? 'justify-end' : 'justify-start'}`}>
            <div className={`max-w-[85%] rounded-2xl px-4 py-2 text-sm ${msg.role === 'user' ? 'bg-blue-600 text-white rounded-br-none' : 'bg-white border border-slate-200 text-slate-700 shadow-sm rounded-bl-none'}`} style={{ fontFamily: 'Inter, sans-serif' }}>
              <p>{msg.content}</p>
              {msg.link && (
                <a href={msg.link} className="inline-block mt-2 text-xs font-medium text-blue-500 hover:text-blue-700 hover:underline">
                  Read the full article →
                </a>
              )}
            </div>
          </div>
        ))}
        {isTyping && (
          <div className="flex justify-start">
            <div className="bg-white border border-slate-200 text-slate-500 shadow-sm rounded-2xl rounded-bl-none px-4 py-3 flex gap-1 items-center">
              <div className="w-1.5 h-1.5 bg-slate-400 rounded-full animate-bounce" style={{ animationDelay: '0ms' }}></div>
              <div className="w-1.5 h-1.5 bg-slate-400 rounded-full animate-bounce" style={{ animationDelay: '150ms' }}></div>
              <div className="w-1.5 h-1.5 bg-slate-400 rounded-full animate-bounce" style={{ animationDelay: '300ms' }}></div>
            </div>
          </div>
        )}
        <div ref={endRef} />
      </div>

      <form onSubmit={handleSend} className="p-3 bg-white border-t border-slate-200">
        <div className="relative">
          <input
            type="text"
            value={input}
            onChange={(e) => setInput(e.target.value)}
            placeholder="Type your question..."
            className="w-full bg-slate-50 border border-slate-200 rounded-full pl-4 pr-10 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
          />
          <button type="submit" disabled={!input.trim()} className="absolute right-1.5 top-1/2 -translate-y-1/2 p-1.5 bg-blue-600 text-white rounded-full disabled:opacity-50 disabled:bg-slate-400 transition-colors">
            <svg className="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M12 19l9 2-9-18-9 18 9-2zm0 0v-8"></path></svg>
          </button>
        </div>
      </form>
    </div>
  );
}
