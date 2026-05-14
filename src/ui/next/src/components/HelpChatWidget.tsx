'use client';
import React, { useState } from 'react';

export const HelpChatWidget: React.FC = () => {
  const [isOpen, setIsOpen] = useState(false);
  const [messages, setMessages] = useState<{role: string, text: string}[]>([]);
  const [input, setInput] = useState('');

  const toggleChat = () => setIsOpen(!isOpen);

  const handleSend = (e: React.FormEvent) => {
    e.preventDefault();
    if (!input.trim()) return;

    const userMsg = { role: 'user', text: input };
    setMessages([...messages, userMsg]);

    // Simulate RAG response
    setTimeout(() => {
      setMessages(prev => [...prev, {
        role: 'agent',
        text: 'I found an article that might help: [Read more →](/help/payments)'
      }]);
    }, 1000);

    setInput('');
  };

  return (
    <div style={{ position: 'fixed', bottom: '20px', right: '20px', zIndex: 9999 }}>
      {isOpen && (
        <div style={{
          width: '300px',
          height: '400px',
          backgroundColor: 'white',
          borderRadius: '12px',
          boxShadow: '0 10px 25px rgba(0,0,0,0.1)',
          display: 'flex',
          flexDirection: 'column',
          marginBottom: '16px',
          overflow: 'hidden',
          border: '1px solid #eee'
        }}>
          <div style={{ padding: '16px', backgroundColor: '#f8f9fa', borderBottom: '1px solid #eee', fontWeight: 'bold' }}>
            Ask OHC Support
          </div>
          <div style={{ flex: 1, padding: '16px', overflowY: 'auto', display: 'flex', flexDirection: 'column', gap: '8px' }}>
             {messages.length === 0 && <div style={{ color: '#666', textAlign: 'center', marginTop: '20px' }}>How can I help you today?</div>}
             {messages.map((msg, idx) => (
               <div key={idx} style={{
                 alignSelf: msg.role === 'user' ? 'flex-end' : 'flex-start',
                 backgroundColor: msg.role === 'user' ? '#0070f3' : '#f1f1f1',
                 color: msg.role === 'user' ? 'white' : 'black',
                 padding: '8px 12px',
                 borderRadius: '16px',
                 maxWidth: '80%'
               }}>
                 {msg.text}
               </div>
             ))}
          </div>
          <form onSubmit={handleSend} style={{ display: 'flex', padding: '12px', borderTop: '1px solid #eee' }}>
            <input
              type="text"
              value={input}
              onChange={e => setInput(e.target.value)}
              placeholder="Type your question..."
              style={{ flex: 1, padding: '8px', border: '1px solid #ccc', borderRadius: '4px', marginRight: '8px' }}
            />
            <button type="submit" style={{ padding: '8px 16px', backgroundColor: '#0070f3', color: 'white', border: 'none', borderRadius: '4px', cursor: 'pointer' }}>Send</button>
          </form>
        </div>
      )}
      <button
        onClick={toggleChat}
        style={{
          width: '56px',
          height: '56px',
          borderRadius: '28px',
          backgroundColor: '#000',
          color: '#fff',
          border: 'none',
          boxShadow: '0 4px 12px rgba(0,0,0,0.15)',
          cursor: 'pointer',
          fontSize: '24px',
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'center'
        }}
      >
        ?
      </button>
    </div>
  );
};
