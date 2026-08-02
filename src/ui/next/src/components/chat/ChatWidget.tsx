import React, { useState, useEffect, useRef } from 'react';
import { motion, AnimatePresence } from 'framer-motion';

interface ChatMessage {
  id: string;
  content: string;
  sender_type: 'Contact' | 'Agent' | 'Bot';
  created_at: number;
}

export const ChatWidget: React.FC = () => {
  const [isOpen, setIsOpen] = useState(false);
  const [messages, setMessages] = useState<ChatMessage[]>([]);
  const [input, setInput] = useState('');
  const wsRef = useRef<WebSocket | null>(null);
  const messagesEndRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (isOpen && !wsRef.current) {
      // Connect to the native Rust chat system
      const ws = new WebSocket(`ws://${window.location.host}/api/v1/chat/ws`);

      ws.onmessage = (event) => {
        const newMessage: ChatMessage = {
          id: crypto.randomUUID(),
          content: event.data,
          sender_type: 'Agent',
          created_at: Date.now(),
        };
        setMessages((prev) => [...prev, newMessage]);
      };

      wsRef.current = ws;
    }

    return () => {
      if (!isOpen && wsRef.current) {
        wsRef.current.close();
        wsRef.current = null;
      }
    };
  }, [isOpen]);

  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [messages]);

  const sendMessage = () => {
    if (!input.trim() || !wsRef.current) return;

    wsRef.current.send(input);

    const newMessage: ChatMessage = {
      id: crypto.randomUUID(),
      content: input,
      sender_type: 'Contact',
      created_at: Date.now(),
    };

    setMessages((prev) => [...prev, newMessage]);
    setInput('');
  };

  return (
    <div className="fixed bottom-4 right-4 z-50">
      <AnimatePresence>
        {isOpen && (
          <motion.div
            initial={{ opacity: 0, y: 20, scale: 0.95 }}
            animate={{ opacity: 1, y: 0, scale: 1 }}
            exit={{ opacity: 0, y: 20, scale: 0.95 }}
            className="mb-4 w-80 bg-white/80 backdrop-blur-md border border-gray-200/50 rounded-2xl shadow-xl overflow-hidden flex flex-col"
            style={{ height: '400px' }}
          >
            <div className="bg-blue-600 p-4 text-white font-medium flex justify-between items-center">
              <span>Chat with us</span>
              <button onClick={() => setIsOpen(false)} className="text-white/80 hover:text-white">✕</button>
            </div>

            <div className="flex-1 overflow-y-auto p-4 space-y-3">
              {messages.map((msg) => (
                <div key={msg.id} className={`flex ${msg.sender_type === 'Contact' ? 'justify-end' : 'justify-start'}`}>
                  <div className={`max-w-[80%] p-3 rounded-2xl text-sm ${msg.sender_type === 'Contact' ? 'bg-blue-600 text-white rounded-br-sm' : 'bg-gray-100 text-gray-800 rounded-bl-sm'}`}>
                    {msg.content}
                  </div>
                </div>
              ))}
              <div ref={messagesEndRef} />
            </div>

            <div className="p-3 bg-white/50 border-t border-gray-100/50 flex gap-2">
              <input
                type="text"
                value={input}
                onChange={(e) => setInput(e.target.value)}
                onKeyDown={(e) => e.key === 'Enter' && sendMessage()}
                className="flex-1 bg-white/80 border border-gray-200/50 rounded-full px-4 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500/50"
                placeholder="Type a message..."
              />
              <button
                onClick={sendMessage}
                className="bg-blue-600 text-white rounded-full w-9 h-9 flex items-center justify-center hover:bg-blue-700 transition-colors"
              >
                ↑
              </button>
            </div>
          </motion.div>
        )}
      </AnimatePresence>

      <button
        onClick={() => setIsOpen(!isOpen)}
        className="w-14 h-14 bg-blue-600 text-white rounded-full shadow-lg flex items-center justify-center hover:bg-blue-700 transition-colors ml-auto block"
        aria-label="Toggle chat"
      >
        <span className="text-2xl">{isOpen ? '✕' : '💬'}</span>
      </button>
    </div>
  );
};
