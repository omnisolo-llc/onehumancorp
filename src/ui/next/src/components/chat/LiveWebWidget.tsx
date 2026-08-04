"use client";

import React, { useState, useRef, useEffect } from 'react';
import { v4 as uuidv4 } from 'uuid';

type ChatMessage = {
  id: string;
  sender_type: 'customer' | 'agent';
  content: string;
  created_at: string;
};

export default function LiveWebWidget({ tenantId }: { tenantId: string }) {
  const [isOpen, setIsOpen] = useState(false);
  const [messages, setMessages] = useState<ChatMessage[]>([]);
  const [inputMessage, setInputMessage] = useState('');
  const [isTyping, setIsTyping] = useState(false);

  const messagesEndRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [messages]);

  const toggleWidget = () => {
    setIsOpen(!isOpen);
    // Simulating initial bot greeting when opened for the first time
    if (!isOpen && messages.length === 0) {
       setMessages([{
         id: uuidv4(),
         sender_type: 'agent',
         content: 'Hello! How can we help you today?',
         created_at: new Date().toISOString()
       }]);
    }
  };

  const handleSendMessage = async (e?: React.FormEvent) => {
    if (e) e.preventDefault();
    if (!inputMessage.trim()) return;

    const newMessage: ChatMessage = {
      id: uuidv4(),
      sender_type: 'customer',
      content: inputMessage.trim(),
      created_at: new Date().toISOString()
    };

    setMessages(prev => [...prev, newMessage]);
    setInputMessage('');
    setIsTyping(true);

    // Simulated response delay for the E2E verification
    setTimeout(() => {
      setIsTyping(false);
      const replyMessage: ChatMessage = {
        id: uuidv4(),
        sender_type: 'agent',
        content: "Thank you for reaching out! A human representative will be with you shortly.",
        created_at: new Date().toISOString()
      };
      setMessages(prev => [...prev, replyMessage]);
    }, 1500);
  };

  return (
    <div className="fixed bottom-6 right-6 z-50 font-inter">
      {isOpen && (
        <div className="w-[375px] h-[600px] mb-4 flex flex-col rounded-2xl overflow-hidden glassmorphism shadow-2xl border border-white/40 bg-white/65 backdrop-blur-[30px] backdrop-saturate-[2.1] relative transition-all">
          {/* Header */}
          <div className="bg-[#0066FF] text-white p-4 flex justify-between items-center rounded-t-2xl">
             <div>
                <h3 className="font-bold font-outfit text-lg">Live Support</h3>
                <p className="text-xs opacity-90 flex items-center gap-1">
                   <span className="w-2 h-2 rounded-full bg-[#34C759]"></span> We usually reply in a few minutes
                </p>
             </div>
             <button onClick={toggleWidget} className="opacity-80 hover:opacity-100 p-1" aria-label="Close Chat">
                <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" /></svg>
             </button>
          </div>

          {/* Messages Area */}
          <div className="flex-1 overflow-y-auto p-4 space-y-4">
             {messages.map((msg) => (
                <div key={msg.id} className={`flex ${msg.sender_type === 'customer' ? 'justify-end' : 'justify-start'}`}>
                   <div className={`max-w-[80%] p-3 text-sm shadow-sm ${msg.sender_type === 'customer' ? 'bg-[#0066FF] text-white rounded-2xl rounded-br-none' : 'bg-white border border-gray-100 text-gray-800 rounded-2xl rounded-bl-none'}`}>
                      {msg.content}
                   </div>
                </div>
             ))}
             {isTyping && (
                <div className="flex justify-start">
                   <div className="bg-white border border-gray-100 p-3 rounded-2xl rounded-bl-none shadow-sm flex gap-1 items-center h-10 w-16 justify-center">
                      <div className="w-2 h-2 bg-gray-400 rounded-full animate-bounce"></div>
                      <div className="w-2 h-2 bg-gray-400 rounded-full animate-bounce" style={{ animationDelay: '0.2s' }}></div>
                      <div className="w-2 h-2 bg-gray-400 rounded-full animate-bounce" style={{ animationDelay: '0.4s' }}></div>
                   </div>
                </div>
             )}
             <div ref={messagesEndRef} />
          </div>

          {/* Input Area */}
          <div className="p-4 bg-white/80 border-t border-gray-200">
             <form onSubmit={handleSendMessage} className="flex items-center gap-2">
                <input
                  type="text"
                  value={inputMessage}
                  onChange={(e) => setInputMessage(e.target.value)}
                  placeholder="Type a message..."
                  className="flex-1 bg-gray-50 border border-gray-200 rounded-full px-4 py-2 text-sm outline-none focus:border-[#0066FF] focus:ring-1 focus:ring-[#0066FF]"
                  data-testid="live-chat-input"
                />
                <button
                  type="submit"
                  disabled={!inputMessage.trim()}
                  className="bg-[#0066FF] text-white rounded-full w-9 h-9 flex items-center justify-center disabled:opacity-50 transition-colors"
                  data-testid="live-chat-send"
                  aria-label="Send Message"
                >
                  <svg className="w-4 h-4 translate-x-[-1px] translate-y-[1px]" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 19l9 2-9-18-9 18 9-2zm0 0v-8" /></svg>
                </button>
             </form>
          </div>
        </div>
      )}

      {/* Floating Button */}
      {!isOpen && (
        <button
          onClick={toggleWidget}
          className="bg-[#0066FF] hover:bg-blue-700 text-white rounded-full w-14 h-14 flex items-center justify-center shadow-lg transition-transform hover:scale-105"
          aria-label="Open Live Chat"
          data-testid="open-live-chat-btn"
        >
          <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8 10h.01M12 10h.01M16 10h.01M9 16H5a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v8a2 2 0 01-2 2h-5l-5 5v-5z" /></svg>
        </button>
      )}
    </div>
  );
}
