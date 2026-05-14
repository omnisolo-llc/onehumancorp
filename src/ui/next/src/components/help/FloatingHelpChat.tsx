import React, { useState } from 'react';

// Floating "Ask anything" chat button and window
export const FloatingHelpChat: React.FC = () => {
  const [isOpen, setIsOpen] = useState(false);
  const [query, setQuery] = useState('');
  const [messages, setMessages] = useState<{role: 'user'|'agent', text: string}[]>([
    { role: 'agent', text: 'Hi there! What do you need help with today?' }
  ]);

  const handleSend = async () => {
    if (!query.trim()) return;

    setMessages(prev => [...prev, { role: 'user', text: query }]);
    const currentQuery = query;
    setQuery('');

    // In production, this hits the ask_help_chat gRPC endpoint
    // const response = await fetch('/api/help/chat', { method: 'POST', body: JSON.stringify({ query: currentQuery }) });
    // const data = await response.json();

    // Simulate agent response for now
    setTimeout(() => {
      setMessages(prev => [...prev, {
        role: 'agent',
        text: `I can help with "${currentQuery}". Have you checked our Getting Started guide?`
      }]);
    }, 1000);
  };

  return (
    <div style={{ position: 'fixed', bottom: '20px', right: '20px', zIndex: 9999, fontFamily: 'Inter, sans-serif' }}>
      {isOpen && (
        <div style={{
          position: 'absolute',
          bottom: '70px',
          right: '0',
          width: '320px',
          height: '400px',
          background: 'white',
          borderRadius: '12px',
          boxShadow: '0 8px 30px rgba(0,0,0,0.15)',
          display: 'flex',
          flexDirection: 'column',
          overflow: 'hidden'
        }}>
          {/* Header */}
          <div style={{ background: '#0056b3', color: 'white', padding: '16px', display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
            <h3 style={{ margin: 0, fontSize: '16px', fontFamily: 'Outfit, sans-serif' }}>OHC Help Assistant</h3>
            <button onClick={() => setIsOpen(false)} style={{ background: 'none', border: 'none', color: 'white', cursor: 'pointer', fontSize: '20px' }}>×</button>
          </div>

          {/* Message List */}
          <div style={{ flex: 1, padding: '16px', overflowY: 'auto', display: 'flex', flexDirection: 'column', gap: '12px', background: '#f8f9fa' }}>
            {messages.map((m, i) => (
              <div key={i} style={{
                alignSelf: m.role === 'user' ? 'flex-end' : 'flex-start',
                background: m.role === 'user' ? '#0056b3' : '#e9ecef',
                color: m.role === 'user' ? 'white' : '#333',
                padding: '10px 14px',
                borderRadius: '18px',
                maxWidth: '80%',
                fontSize: '14px'
              }}>
                {m.text}
              </div>
            ))}
          </div>

          {/* Input Area */}
          <div style={{ padding: '12px', borderTop: '1px solid #eee', display: 'flex', gap: '8px', background: 'white' }}>
            <input
              type="text"
              value={query}
              onChange={e => setQuery(e.target.value)}
              onKeyPress={e => e.key === 'Enter' && handleSend()}
              placeholder="Ask a question..."
              style={{ flex: 1, padding: '10px', borderRadius: '20px', border: '1px solid #ccc', outline: 'none' }}
            />
            <button
              onClick={handleSend}
              style={{ background: '#0056b3', color: 'white', border: 'none', borderRadius: '20px', padding: '0 16px', cursor: 'pointer', fontWeight: 'bold' }}
            >
              Send
            </button>
          </div>
        </div>
      )}

      {/* Floating Button */}
      <button
        onClick={() => setIsOpen(!isOpen)}
        style={{
          width: '60px',
          height: '60px',
          borderRadius: '30px',
          background: '#0056b3',
          color: 'white',
          border: 'none',
          boxShadow: '0 4px 12px rgba(0,0,0,0.2)',
          cursor: 'pointer',
          fontSize: '24px',
          display: 'flex',
          justifyContent: 'center',
          alignItems: 'center'
        }}
        aria-label="Open Help Chat"
      >
        ?
      </button>
    </div>
  );
};
