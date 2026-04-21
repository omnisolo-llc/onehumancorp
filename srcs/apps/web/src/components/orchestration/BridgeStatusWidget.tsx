import React from 'react';

const BridgeStatusWidget: React.FC = () => {
  return (
    <div className="bridge-card">
      <style>{`
        .bridge-card {
          backdrop-filter: blur(20px) saturate(200%);
          background: rgba(255, 255, 255, 0.03);
          font-family: 'Outfit', 'Inter', sans-serif;
          border: 1px solid rgba(255, 255, 255, 0.1);
          border-radius: 16px;
          padding: 20px;
          color: white;
        }
      `}</style>
      <h2>Bridge Status</h2>
      <p>Status: ACTIVE</p>
    </div>
  );
};

export default BridgeStatusWidget;
