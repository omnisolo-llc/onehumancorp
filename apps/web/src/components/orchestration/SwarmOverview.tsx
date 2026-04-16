import React from 'react';
import { theme } from '../../styles/theme';

const SwarmOverview = () => {
  return (
    <div style={{ ...theme.glassmorphism, ...theme.typography, padding: '24px', borderRadius: '16px', color: theme.colors.text, marginBottom: '24px' }}>
      <h2 style={{ marginBottom: '16px', fontWeight: 600 }}>Swarm Overview</h2>
      <div style={{ display: 'flex', gap: '32px' }}>
        <div style={{ flex: 1, padding: '16px', background: 'rgba(0,0,0,0.2)', borderRadius: '12px' }}>
          <h3 style={{ fontSize: '14px', color: 'rgba(255,255,255,0.7)', marginBottom: '8px' }}>Active Agents</h3>
          <p data-testid="active-agents" style={{ fontSize: '32px', fontWeight: 'bold' }}>12</p>
        </div>
        <div style={{ flex: 1, padding: '16px', background: 'rgba(0,0,0,0.2)', borderRadius: '12px' }}>
          <h3 style={{ fontSize: '14px', color: 'rgba(255,255,255,0.7)', marginBottom: '8px' }}>Completed Tasks</h3>
          <p data-testid="completed-tasks" style={{ fontSize: '32px', fontWeight: 'bold' }}>145</p>
        </div>
      </div>
    </div>
  );
};

export default SwarmOverview;
