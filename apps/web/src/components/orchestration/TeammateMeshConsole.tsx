import React, { useEffect, useState, useRef } from 'react';
import { theme } from '../../styles/theme';

const TeammateMeshConsole = () => {
  const [messages, setMessages] = useState<string[]>([]);
  const scrollRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (scrollRef.current) {
      scrollRef.current.scrollTop = scrollRef.current.scrollHeight;
    }
  }, [messages]);

  useEffect(() => {
    // Simulating WebSocket connection
    const socket = new WebSocket('ws://localhost:8080/api/mesh/stream');

    socket.onmessage = (event) => {
      setMessages((prevMessages) => [...prevMessages, event.data]);
    };

    return () => socket.close();
  }, []);

  return (
    <div style={{ ...theme.glassmorphism, ...theme.typography, padding: '24px', borderRadius: '16px', color: theme.colors.text, marginTop: '24px' }}>
      <h2 style={{ marginBottom: '16px', fontWeight: 600 }}>Teammate Mesh Console</h2>
      <div
        ref={scrollRef}
        style={{
          height: '250px',
          overflowY: 'auto',
          background: 'rgba(0,0,0,0.3)',
          padding: '16px',
          borderRadius: '12px',
          border: '1px solid rgba(255,255,255,0.05)',
          display: 'flex',
          flexDirection: 'column',
          gap: '8px'
        }}
      >
        {messages.length === 0 ? (
          <div style={{
            display: 'flex',
            height: '100%',
            alignItems: 'center',
            justifyContent: 'center',
            color: 'rgba(255,255,255,0.5)',
            fontStyle: 'italic'
          }}>
            Waiting for messages...
          </div>
        ) : messages.map((msg, idx) => (
          <div key={idx} style={{
            fontFamily: 'monospace',
            fontSize: '13px',
            lineHeight: '1.4',
            padding: '4px 0',
            borderBottom: idx < messages.length - 1 ? '1px solid rgba(255,255,255,0.05)' : 'none'
          }}>
            <span style={{ color: theme.colors.executing, marginRight: '8px' }}>&gt;</span>
            {msg}
          </div>
        ))}
      </div>
    </div>
  );
};

export default TeammateMeshConsole;
