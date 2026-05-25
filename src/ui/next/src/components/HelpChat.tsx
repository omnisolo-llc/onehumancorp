"use client";
import { invoke } from "@tauri-apps/api/core";

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
    setInputValue("");    try {
      if (typeof window !== "undefined" && (window as any).__TAURI_INTERNALS__) {
          const data: any = await invoke("chat_help", { message: inputValue });
          setMessages(prev => [...prev, { id: (Date.now() + 1).toString(), sender: 'agent', text: data.reply, link: data.link }]);
      } else {
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
      }
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
      <div className="fixed bottom-6 right-6 z-50">
        {!isOpen && (
          <button
            onClick={() => setIsOpen(true)}
            className="bg-gray-900 text-white p-4 rounded-full shadow-2xl hover:shadow-xl hover:scale-105 transition-all flex items-center justify-center gap-2 group"
          >
            <span className="text-xl">✨</span>
            <span className="font-outfit font-bold max-w-0 overflow-hidden group-hover:max-w-xs transition-all duration-300 whitespace-nowrap px-0 group-hover:px-2">Ask anything</span>
          </button>
        )}
      </div>

      {/* Chat Interface */}
      {isOpen && (
        <div className="fixed bottom-6 right-6 z-[60] w-[350px] max-w-[calc(100vw-48px)] bg-white rounded-2xl shadow-2xl flex flex-col overflow-hidden border border-gray-200 animate-slide-up-chat">
          {/* Header */}
          <div className="bg-gray-900 text-white p-4 flex justify-between items-center">
            <div className="flex items-center gap-2">
              <span className="text-xl">✨</span>
              <div>
                <h3 className="font-bold font-outfit text-sm">Help Agent</h3>
                <p className="text-xs text-gray-400 font-inter">Always here to help</p>
              </div>
            </div>
            <button onClick={() => setIsOpen(false)} className="text-gray-400 hover:text-white transition-colors">
              <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" /></svg>
            </button>
          </div>

          {/* Messages */}
          <div className="flex-1 p-4 overflow-y-auto h-[350px] bg-gray-50 flex flex-col gap-4 font-inter text-sm">
            {messages.map(msg => (
              <div key={msg.id} className={`flex flex-col ${msg.sender === 'user' ? 'items-end' : 'items-start'}`}>
                <div className={`px-4 py-2.5 rounded-2xl max-w-[85%] leading-relaxed ${
                  msg.sender === 'user'
                    ? 'bg-blue-600 text-white rounded-br-sm shadow-sm'
                    : 'bg-white border border-gray-200 text-gray-800 rounded-bl-sm shadow-sm'
                }`}>
                  {msg.text}
                </div>
                {msg.link && (
                  <a href={msg.link.url} className="mt-2 ml-1 text-blue-600 hover:text-blue-800 text-xs font-semibold hover:underline bg-blue-50 px-3 py-1.5 rounded-full border border-blue-100 flex items-center shadow-sm">
                    {msg.link.title}
                  </a>
                )}
              </div>
            ))}
            <div ref={messagesEndRef} />
          </div>

          {/* Input */}
          <form onSubmit={handleSend} className="p-3 bg-white border-t border-gray-100 flex gap-2">
            <input
              type="text"
              value={inputValue}
              onChange={(e) => setInputValue(e.target.value)}
              placeholder="Ask me anything..."
              className="flex-1 bg-gray-50 border border-gray-200 rounded-xl px-4 py-2.5 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500 font-inter"
            />
            <button
              type="submit"
              disabled={!inputValue.trim()}
              className="bg-blue-600 text-white p-2.5 rounded-xl disabled:opacity-50 disabled:cursor-not-allowed hover:bg-blue-700 transition-colors shadow-sm"
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