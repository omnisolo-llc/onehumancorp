import React from 'react';

const SwarmOverview = () => {
  return (
    <div style={{
      backdropFilter: 'blur(20px) saturate(200%)',
      background: 'rgba(255, 255, 255, 0.03)',
      fontFamily: '"Outfit", "Inter", sans-serif',
      padding: '20px',
      borderRadius: '12px',
      border: '1px solid rgba(255, 255, 255, 0.1)',
      color: '#fff',
      marginBottom: '20px'
    }}>
      <h2>Swarm Overview</h2>
      <div style={{ display: 'flex', gap: '20px' }}>
        <div>
          <h3>Active Agents</h3>
          <p data-testid="active-agents">12</p>
        </div>
        <div>
          <h3>Completed Tasks</h3>
          <p data-testid="completed-tasks">145</p>
        </div>
      </div>
    </div>
  );
};

export default SwarmOverview;
