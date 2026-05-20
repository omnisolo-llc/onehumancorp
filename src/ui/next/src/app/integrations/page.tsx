'use client';

import React, { useState } from 'react';

export default function IntegrationsDashboard() {
  const [manychatConnected, setManychatConnected] = useState(false);

  const handleConnectManychat = async () => {
    // In a real flow, this would redirect to Manychat OAuth
    // For now we simulate success
    setManychatConnected(true);
  };

  return (
    <div style={{
      background: 'rgba(255, 255, 255, 0.65)',
      backdropFilter: 'blur(30px) saturate(210%)',
      border: '1px solid rgba(255, 255, 255, 0.4)',
      borderRadius: '16px',
      padding: '24px',
      maxWidth: '800px',
      margin: '0 auto'
    }}>
      <h1 style={{ fontFamily: 'Outfit, sans-serif', color: '#1D1D1F' }}>Connect Tools</h1>
      <p style={{ fontFamily: 'Inter, sans-serif', color: '#1D1D1F' }}>
        Seamlessly connect your favorite apps to streamline your business operations.
      </p>

      <div style={{
        marginTop: '24px',
        background: 'rgba(255, 255, 255, 0.8)',
        border: '1px solid rgba(0,0,0,0.05)',
        borderRadius: '8px',
        padding: '16px'
      }}>
        <h2 style={{ fontFamily: 'Outfit, sans-serif', fontSize: '1.25rem' }}>Manychat</h2>
        <p style={{ fontFamily: 'Inter, sans-serif', fontSize: '0.875rem', marginBottom: '16px' }}>
          Unified Customer Inbox. Manage all your messages and posts from one place.
        </p>

        {!manychatConnected ? (
          <button
            onClick={handleConnectManychat}
            style={{
              background: '#0071E3',
              color: 'white',
              border: 'none',
              borderRadius: '8px',
              padding: '8px 16px',
              fontFamily: 'Inter, sans-serif',
              cursor: 'pointer'
            }}
          >
            Connect Instagram
          </button>
        ) : (
          <div>
            <h3 style={{ fontFamily: 'Outfit, sans-serif', color: '#34C759' }}>Customer Inbox</h3>
            <p>Connected successfully</p>
          </div>
        )}
      </div>

      <div style={{ marginTop: '24px' }}>
        <h2>Buffer</h2>
        <p>Unified Customer Inbox. Manage all your messages and posts from one place.</p>
        <button>Connect</button>
      </div>

      <div style={{ marginTop: '24px' }}>
        <h2>Acuity Scheduling</h2>
        <p>Automated Booking. Let customers schedule appointments 24/7.</p>
        <button>Connect</button>
      </div>

      <div style={{ marginTop: '24px' }}>
        <h2>ShipStation</h2>
        <button>Connect</button>
      </div>

      <div style={{ marginTop: '24px' }}>
        <h2>Alipay</h2>
        <button>Connect</button>
      </div>

      <div style={{ marginTop: '24px' }}>
        <h2>ActiveCampaign</h2>
        <button>Connect</button>
      </div>

      <div style={{ marginTop: '24px' }}>
        <h2>Microsoft Teams</h2>
        <button>Connect</button>
      </div>

      <div style={{ marginTop: '24px' }}>
        <h2>Global SMS Notifications</h2>
        <button>Connect SMS</button>
      </div>
    </div>
  );
}
