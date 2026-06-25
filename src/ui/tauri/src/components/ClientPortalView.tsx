import React, { useState } from 'react';

export const ClientPortalView = () => {
  const [magicLink, setMagicLink] = useState('');

  const generateLink = async () => {
    // In a real app, this would call the /api/v1/portals/:id/magic-link endpoint
    setMagicLink('https://portal.onehumancorp.com/access?token=demo-token');
  };

  return (
    <div style={{ padding: '20px', fontFamily: 'Inter, sans-serif' }}>
      <h2>Client Portal Management</h2>
      <p>Share a secure, branded portal link with your clients.</p>

      <button
        onClick={generateLink}
        style={{
          background: 'rgba(255, 255, 255, 0.1)',
          backdropFilter: 'blur(20px)',
          border: '1px solid rgba(255, 255, 255, 0.2)',
          padding: '10px 20px',
          borderRadius: '8px',
          color: 'inherit',
          cursor: 'pointer'
        }}
      >
        Generate Magic Link
      </button>

      {magicLink && (
        <div style={{ marginTop: '20px', padding: '15px', background: 'rgba(0,0,0,0.05)', borderRadius: '8px' }}>
          <strong>Magic Link:</strong> <a href={magicLink}>{magicLink}</a>
        </div>
      )}
    </div>
  );
};
