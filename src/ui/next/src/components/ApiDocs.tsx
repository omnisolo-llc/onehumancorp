"use client";
import React from 'react';

export const ApiDocs: React.FC = () => {
  return (
    <div style={{ padding: '40px', maxWidth: '800px', margin: '0 auto', fontFamily: 'Inter, sans-serif' }}>
      <h1>API Reference (Advanced)</h1>
      <p style={{ color: '#666' }}>For developers and advanced users looking to integrate OHC directly.</p>

      <div style={{ background: '#1e1e1e', color: '#d4d4d4', padding: '20px', borderRadius: '8px', marginTop: '20px', fontFamily: 'monospace' }}>
        <h3 style={{ color: '#569cd6', marginTop: 0 }}>POST /api/v1/checkout</h3>
        <p>Create a new checkout session.</p>
        <pre style={{ background: '#000', padding: '15px', borderRadius: '4px', overflowX: 'auto' }}>
{`{
  "amount": 1500,
  "currency": "usd",
  "success_url": "https://example.com/success"
}`}
        </pre>
      </div>
    </div>
  );
};
