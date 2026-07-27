import React, { useEffect, useState, useRef } from 'react';
import { v4 as uuidv4 } from 'uuid';

interface Message {
  id: string;
  tenant_id: string;
  conversation_id: string;
  sender_type: string;
  content: string;
  created_at: string;
}

export const NativeChatInboxPlaceholder: React.FC = () => {
  const [messages, setMessages] = useState<Message[]>([]);
  const [inputValue, setInputValue] = useState('');
  const wsRef = useRef<WebSocket | null>(null);

  // Hardcoded for acceptance criteria simulation
  const tenantId = '00000000-0000-0000-0000-000000000000';
  const conversationId = '11111111-1111-1111-1111-111111111111';

  useEffect(() => {
    // In a real app this would point to the backend WS endpoint, e.g. ws://localhost:3000/ws/chat
    // For now we simulate the connection state
    const ws = new WebSocket('ws://localhost:3000/ws/chat');

    ws.onopen = () => {
      console.log('Connected to chat websocket');
    };

    ws.onmessage = (event) => {
      try {
        const data = JSON.parse(event.data);
        if (data.type === 'MessageCreated') {
           setMessages(prev => [...prev, data.payload.message]);
        }
      } catch (e) {
        console.error('Error parsing WS message', e);
      }
    };

    ws.onerror = (error) => {
        console.error('WebSocket Error', error);
    };

    wsRef.current = ws;

    return () => {
      ws.close();
    };
  }, []);

  const handleSend = () => {
    if (!inputValue.trim() || !wsRef.current) return;

    const newMessage: Message = {
      id: uuidv4(),
      tenant_id: tenantId,
      conversation_id: conversationId,
      sender_type: 'agent',
      content: inputValue,
      created_at: new Date().toISOString()
    };

    // Optimistic update
    setMessages(prev => [...prev, { ...newMessage, status: 'Sending...' } as any]);

    if (wsRef.current.readyState === WebSocket.OPEN) {
       wsRef.current.send(JSON.stringify(newMessage));
    }

    setInputValue('');
  };

  return (
    <div style={{ maxWidth: '375px', margin: '0 auto', border: '1px solid #ccc', height: '100vh', display: 'flex', flexDirection: 'column' }}>
      <header style={{ padding: '16px', borderBottom: '1px solid #eee', fontWeight: 'bold' }}>
        Inbox (Native)
      </header>

      <main style={{ flex: 1, overflowY: 'auto', padding: '16px', display: 'flex', flexDirection: 'column', gap: '8px' }}>
        {messages.length === 0 && <div style={{ color: '#888', textAlign: 'center', marginTop: '20px' }}>No messages yet.</div>}
        {messages.map((msg, idx) => (
          <div key={idx} style={{
            alignSelf: msg.sender_type === 'agent' ? 'flex-end' : 'flex-start',
            backgroundColor: msg.sender_type === 'agent' ? '#007aff' : '#e5e5ea',
            color: msg.sender_type === 'agent' ? 'white' : 'black',
            padding: '8px 12px',
            borderRadius: '16px',
            maxWidth: '80%',
            wordBreak: 'break-word'
          }}>
            {msg.content}
            {(msg as any).status && <div style={{ fontSize: '10px', opacity: 0.7, marginTop: '4px' }}>{(msg as any).status}</div>}
          </div>
        ))}
      </main>

      <footer style={{ padding: '16px', borderTop: '1px solid #eee', display: 'flex', gap: '8px' }}>
        <input
          type="text"
          value={inputValue}
          onChange={e => setInputValue(e.target.value)}
          placeholder="Type a message..."
          style={{ flex: 1, padding: '12px', borderRadius: '24px', border: '1px solid #ccc' }}
          onKeyPress={(e) => e.key === 'Enter' && handleSend()}
        />
        <button
          onClick={handleSend}
          style={{ padding: '12px 16px', minWidth: '44px', minHeight: '44px', backgroundColor: '#007aff', color: 'white', borderRadius: '24px', border: 'none', fontWeight: 'bold' }}
        >
          Send
        </button>
      </footer>
    </div>
  );
};
