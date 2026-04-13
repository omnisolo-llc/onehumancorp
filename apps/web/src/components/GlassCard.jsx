import React from 'react';
import '../styles/GlassCard.css';

export function GlassCard({ children }) {
  return (
    <div className="glass-card">
      {children}
    </div>
  );
}
