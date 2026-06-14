"use client";

import { useState, useEffect, useRef } from 'react';
import { useSearchParams } from 'next/navigation';

interface Message {
  id: string;
  role: 'user' | 'agent';
  content: string;
}

export default function ChatIntakeWidget() {
  const searchParams = useSearchParams();
  const tenant = searchParams.get('tenant') || 'default';

  const [sessionId, setSessionId] = useState<string | null>(null);
  const [messages, setMessages] = useState<Message[]>([]);
  const [inputText, setInputText] = useState('');
  const [isLoading, setIsLoading] = useState(false);
  const [isCompleted, setIsCompleted] = useState(false);
  const messagesEndRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    // Start session
    const startSession = async () => {
      try {
        const res = await fetch(`/api/agents/client_intake/session?tenant=${tenant}`, {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ customer_info: {} })
        });
        const data = await res.json();
        setSessionId(data.session_id);
        setMessages([
          { id: '1', role: 'agent', content: data.initial_message }
        ]);
      } catch (err) {
        console.error(err);
      }
    };
    startSession();
  }, [tenant]);

  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [messages]);

  const handleSend = async () => {
    if (!inputText.trim() || !sessionId || isLoading || isCompleted) return;

    const userText = inputText;
    setInputText('');
    setMessages(prev => [...prev, { id: Date.now().toString(), role: 'user', content: userText }]);
    setIsLoading(true);

    try {
      const res = await fetch(`/api/agents/client_intake/session/${sessionId}/message?tenant=${tenant}`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ content: userText })
      });
      const data = await res.json();

      setMessages(prev => [...prev, { id: Date.now().toString(), role: 'agent', content: data.reply }]);

      if (data.status === 'completed') {
        setIsCompleted(true);
      }
    } catch (err) {
      console.error(err);
    } finally {
      setIsLoading(false);
    }
  };

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      handleSend();
    }
  };

  return (
    <div className="flex flex-col h-screen max-w-[375px] mx-auto bg-gray-50 border-x border-gray-200">
      <header className="bg-white p-4 border-b border-gray-200 flex items-center justify-between sticky top-0 z-10 shadow-sm">
        <div>
          <h1 className="font-bold text-gray-900">Virtual Assistant</h1>
          <p className="text-xs text-green-500 flex items-center gap-1">
            <span className="w-2 h-2 rounded-full bg-green-500 inline-block"></span> Online
          </p>
        </div>
      </header>

      <div className="flex-1 overflow-y-auto p-4 space-y-4">
        {messages.map((msg, i) => (
          <div key={i} className={`flex ${msg.role === 'user' ? 'justify-end' : 'justify-start'}`}>
            <div className={`max-w-[80%] p-3 rounded-2xl ${
              msg.role === 'user'
                ? 'bg-blue-600 text-white rounded-br-none'
                : 'bg-white border border-gray-200 text-gray-800 rounded-bl-none shadow-sm'
            }`}>
              <p className="text-sm">{msg.content}</p>
            </div>
          </div>
        ))}
        {isLoading && (
          <div className="flex justify-start">
            <div className="bg-white border border-gray-200 p-3 rounded-2xl rounded-bl-none shadow-sm flex gap-1 items-center">
              <div className="w-2 h-2 bg-gray-400 rounded-full animate-bounce"></div>
              <div className="w-2 h-2 bg-gray-400 rounded-full animate-bounce" style={{ animationDelay: '0.2s' }}></div>
              <div className="w-2 h-2 bg-gray-400 rounded-full animate-bounce" style={{ animationDelay: '0.4s' }}></div>
            </div>
          </div>
        )}
        <div ref={messagesEndRef} />
      </div>

      <div className="p-4 bg-white border-t border-gray-200">
        <div className="relative flex items-center">
          <textarea
            value={inputText}
            onChange={(e) => setInputText(e.target.value)}
            onKeyDown={handleKeyDown}
            placeholder={isCompleted ? "Session completed" : "Type your message..."}
            disabled={isCompleted || isLoading}
            className="w-full bg-gray-100 border-transparent focus:bg-white focus:border-blue-500 focus:ring-2 focus:ring-blue-200 rounded-full py-3 px-4 pr-12 resize-none h-[48px] overflow-hidden text-sm"
            rows={1}
          />
          <button
            onClick={handleSend}
            disabled={!inputText.trim() || isLoading || isCompleted}
            className="absolute right-2 p-1.5 bg-blue-600 text-white rounded-full disabled:opacity-50 disabled:bg-gray-400 transition-colors"
          >
            <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M5 12h14M12 5l7 7-7 7"></path></svg>
          </button>
        </div>
      </div>
    </div>
  );
}
