import React, { useEffect, useState } from 'react';

const TeammateMeshConsole = () => {
  const [messages, setMessages] = useState<string[]>([]);

  useEffect(() => {
    // Simulating WebSocket connection
    const socket = new WebSocket('ws://localhost:8080/api/mesh/stream');

    socket.onmessage = (event) => {
      setMessages((prevMessages) => [...prevMessages, event.data]);
    };

    return () => socket.close();
  }, []);

  return (
    <div style={{
      backdropFilter: 'blur(20px) saturate(200%)',
      background: 'rgba(255, 255, 255, 0.03)',
      fontFamily: '"Outfit", "Inter", sans-serif',
      padding: '24px',
      borderRadius: '16px',
      border: '1px solid rgba(255, 255, 255, 0.1)',
      color: '#fff',
      marginTop: '24px',
      boxShadow: '0 4px 30px rgba(0, 0, 0, 0.1)'
    }}>
      <h2>Teammate Mesh Console</h2>
      <div style={{ height: '200px', overflowY: 'auto', background: 'rgba(0,0,0,0.5)', padding: '10px' }}>
        {messages.length === 0 ? <p>Waiting for messages...</p> : messages.map((msg, idx) => (
          <p key={idx}>{msg}</p>
        ))}
      </div>
    </div>
  );
};

export default TeammateMeshConsole;
