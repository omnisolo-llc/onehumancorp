import React, { useEffect, useState } from 'react';
import { theme } from '../../styles/theme';

interface BridgeStatus {
  [orgId: string]: string;
}

const BridgeStatusWidget: React.FC = () => {
  const [status, setStatus] = useState<BridgeStatus>({});

  useEffect(() => {
    // In a real app, this would fetch from /api/v1/mesh/bridge/status
    // Polling or websocket to get status
    const fetchStatus = () => {
      fetch('/api/v1/mesh/bridge/status')
        .then(res => res.json())
        .then(data => setStatus(data.status || {}))
        .catch(err => console.error("Failed to fetch bridge status", err));
    };

    fetchStatus();
    const interval = setInterval(fetchStatus, 5000);
    return () => clearInterval(interval);
  }, []);

  return (
    <div style={{
      ...theme.glassmorphism,
      fontFamily: theme.typography.fontFamily,
      borderRadius: '16px',
      padding: '20px',
      margin: '20px 0'
    }}>
      <h2 style={{ color: '#fff', fontSize: '1.2rem', marginBottom: '16px' }}>Universal Mesh Bridge</h2>
      {Object.keys(status).length === 0 ? (
        <p style={{ color: '#888' }}>No active bridges to remote swarms.</p>
      ) : (
        <ul style={{ listStyle: 'none', padding: 0 }}>
          {Object.entries(status).map(([orgId, state]) => (
            <li key={orgId} style={{ display: 'flex', justifyContent: 'space-between', marginBottom: '10px' }}>
              <span style={{ color: '#ccc' }}>Org: {orgId}</span>
              <span style={{
                color: state === 'ACTIVE' ? '#4ade80' : '#f87171',
                fontWeight: 'bold',
              }}>
                {state}
              </span>
            </li>
          ))}
        </ul>
      )}
    </div>
  );
};

export default BridgeStatusWidget;
