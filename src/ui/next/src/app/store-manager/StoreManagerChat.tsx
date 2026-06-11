'use client';

import React, { useState, useRef, useEffect } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { FiSend, FiMic, FiMoreVertical } from 'react-icons/fi';

interface Action {
  label: string;
  actionValue: string;
}

interface Message {
  id: string;
  role: 'user' | 'agent';
  text: string;
  timestamp: Date;
  actions?: Action[];
}

export function StoreManagerChat() {
  const [messages, setMessages] = useState<Message[]>([
    {
      id: '1',
      role: 'agent',
      text: "Good morning! You have 3 new orders. Should I schedule pickups?",
      timestamp: new Date(),
    }
  ]);
  const [input, setInput] = useState('');
  const [isTyping, setIsTyping] = useState(false);
  const messagesEndRef = useRef<HTMLDivElement>(null);

  const scrollToBottom = () => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  };

  useEffect(() => {
    scrollToBottom();
  }, [messages, isTyping]);

  const handleSend = async () => {
    if (!input.trim()) return;

    const userText = input;
    const userMsg: Message = {
      id: Date.now().toString(),
      role: 'user',
      text: userText,
      timestamp: new Date(),
    };

    setMessages(prev => [...prev, userMsg]);
    setInput('');
    setIsTyping(true);

    try {
      const response = await fetch('/api/store-manager', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ message: userText }),
      });

      if (response.ok) {
        const data = await response.json();
        setMessages(prev => [...prev, {
          id: Date.now().toString(),
          role: 'agent',
          text: data.text,
          timestamp: new Date(),
          actions: data.actions
        }]);
      } else {
        throw new Error('Network response was not ok');
      }
    } catch (error) {
      setMessages(prev => [...prev, {
        id: Date.now().toString(),
        role: 'agent',
        text: "I'm sorry, I couldn't process that request right now.",
        timestamp: new Date(),
      }]);
    } finally {
      setIsTyping(false);
    }
  };

  const handleAction = async (actionValue: string) => {
    // Add user action to chat
    setMessages(prev => [...prev, {
      id: Date.now().toString(),
      role: 'user',
      text: actionValue,
      timestamp: new Date(),
    }]);

    setIsTyping(true);

    try {
      const response = await fetch('/api/store-manager', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ message: actionValue }),
      });

      if (response.ok) {
        const data = await response.json();
        setMessages(prev => [...prev, {
          id: Date.now().toString(),
          role: 'agent',
          text: data.text || actionValue,
          timestamp: new Date(),
          actions: data.actions
        }]);
      } else {
        throw new Error('Network response was not ok');
      }
    } catch (error) {
      setMessages(prev => [...prev, {
        id: Date.now().toString(),
        role: 'agent',
        text: "I'm sorry, I couldn't process that request right now.",
        timestamp: new Date(),
      }]);
    } finally {
      setIsTyping(false);
    }
  };

  return (
    <div className="flex flex-col h-full bg-white relative">
      <div className="flex-grow overflow-y-auto p-4 space-y-4">
        <AnimatePresence initial={false}>
          {messages.map((msg) => (
            <motion.div
              key={msg.id}
              initial={{ opacity: 0, y: 10 }}
              animate={{ opacity: 1, y: 0 }}
              className={`flex ${msg.role === 'user' ? 'justify-end' : 'justify-start'}`}
            >
              <div
                className={`max-w-[85%] rounded-2xl p-3 ${
                  msg.role === 'user'
                    ? 'bg-blue-600 text-white rounded-br-sm'
                    : 'bg-gray-100 text-gray-800 rounded-bl-sm'
                }`}
              >
                <p className="text-[15px] leading-relaxed">{msg.text}</p>
                {msg.actions && (
                  <div className="mt-3 flex flex-col space-y-2">
                    {msg.actions.map((action, idx) => (
                      <button
                        key={idx}
                        onClick={() => handleAction(action.actionValue)}
                        className="bg-white text-blue-600 border border-blue-200 px-3 py-1.5 rounded-lg text-sm font-medium hover:bg-blue-50 transition-colors text-left"
                      >
                        {action.label}
                      </button>
                    ))}
                  </div>
                )}
                <span className={`text-[11px] mt-1 block opacity-70 ${msg.role === 'user' ? 'text-right' : ''}`}>
                  {msg.timestamp.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })}
                </span>
              </div>
            </motion.div>
          ))}
          {isTyping && (
             <motion.div
             initial={{ opacity: 0, y: 10 }}
             animate={{ opacity: 1, y: 0 }}
             className="flex justify-start"
           >
             <div className="bg-gray-100 rounded-2xl p-4 rounded-bl-sm flex space-x-1">
               <div className="w-2 h-2 bg-gray-400 rounded-full animate-bounce" style={{ animationDelay: '0ms' }} />
               <div className="w-2 h-2 bg-gray-400 rounded-full animate-bounce" style={{ animationDelay: '150ms' }} />
               <div className="w-2 h-2 bg-gray-400 rounded-full animate-bounce" style={{ animationDelay: '300ms' }} />
             </div>
           </motion.div>
          )}
        </AnimatePresence>
        <div ref={messagesEndRef} />
      </div>

      <div className="p-3 bg-white border-t border-gray-100 pb-safe">
        <div className="flex items-end bg-gray-100 rounded-2xl p-2 focus-within:ring-2 focus-within:ring-blue-100 transition-all">
          <button className="p-2 text-gray-400 hover:text-gray-600 rounded-full">
             <FiMoreVertical size={20} />
          </button>
          <textarea
            value={input}
            onChange={(e) => setInput(e.target.value)}
            onKeyDown={(e) => {
              if (e.key === 'Enter' && !e.shiftKey) {
                e.preventDefault();
                handleSend();
              }
            }}
            placeholder="Tell me what to do..."
            className="flex-grow bg-transparent border-none focus:ring-0 resize-none max-h-32 min-h-[40px] px-2 py-2 text-[15px] text-gray-800 placeholder-gray-400"
            rows={1}
          />
          {input.trim() ? (
            <button
              onClick={handleSend}
              className="p-2.5 bg-blue-600 text-white rounded-full shadow-sm hover:bg-blue-700 transition-colors ml-2 flex-shrink-0"
            >
              <FiSend size={18} className="ml-0.5" />
            </button>
          ) : (
            <button className="p-2.5 text-gray-400 hover:text-blue-600 rounded-full transition-colors ml-2 flex-shrink-0">
              <FiMic size={20} />
            </button>
          )}
        </div>
      </div>
    </div>
  );
}
