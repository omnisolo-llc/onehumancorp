import React, { useEffect, useState } from 'react';
import { theme } from '../../styles/theme';

const AutoDreamPipelineWidget = () => {
  const [progress, setProgress] = useState(0);

  useEffect(() => {
    const interval = setInterval(() => {
      setProgress((prev) => (prev >= 1 ? 0 : prev + 0.05));
    }, 150);
    return () => clearInterval(interval);
  }, []);

  const renderNode = (label: string, icon: string, color: string) => (
    <div style={{ display: 'flex', flexDirection: 'column', alignItems: 'center' }}>
      <div style={{
        width: '48px', height: '48px', borderRadius: '50%',
        backgroundColor: 'rgba(255,255,255,0.05)',
        border: `2px solid ${color}`,
        display: 'flex', justifyContent: 'center', alignItems: 'center',
        boxShadow: `0 0 10px ${color}`
      }}>
        <span style={{ fontSize: '24px' }}>{icon}</span>
      </div>
      <span style={{ marginTop: '8px', fontSize: '12px', color: 'rgba(255,255,255,0.7)' }}>{label}</span>
    </div>
  );

  const renderConnection = (color: string) => (
    <div style={{ flex: 1, height: '2px', backgroundColor: 'rgba(255,255,255,0.1)', margin: '24px 16px 0', position: 'relative' }}>
      <div style={{
        position: 'absolute',
        top: '-3px',
        left: `${progress * 100}%`,
        width: '8px',
        height: '8px',
        backgroundColor: color,
        borderRadius: '50%',
        boxShadow: `0 0 5px ${color}`,
        transition: 'left 0.15s linear'
      }} />
    </div>
  );

  return (
    <div data-testid="autodream-pipeline" style={{ ...theme.glassmorphism, padding: '24px', borderRadius: '16px', color: theme.colors.text, marginBottom: '24px' }}>
      <h2 style={{ marginBottom: '24px', fontWeight: 600 }}>AutoDream Pipeline Stream</h2>
      <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'flex-start' }}>
        {renderNode('Extract', '📄', '#3498db')}
        {renderConnection('#3498db')}
        {renderNode('Analyze', '🧠', '#9b59b6')}
        {renderConnection('#9b59b6')}
        {renderNode('Embed', '✨', '#00bcd4')}
        {renderConnection('#00bcd4')}
        {renderNode('Store', '💾', '#2ecc71')}
      </div>
    </div>
  );
};

export default AutoDreamPipelineWidget;
