import React from 'react';

// API Documentation Page (For Advanced Users)
export default function ApiDocs() {
  return (
    <div style={{ fontFamily: 'Inter, sans-serif', padding: '40px', maxWidth: '800px', margin: '0 auto' }}>
      <h1 style={{ fontFamily: 'Outfit, sans-serif', fontSize: '32px', marginBottom: '16px' }}>OHC API Reference</h1>
      <div style={{ background: '#fff3cd', border: '1px solid #ffeeba', color: '#856404', padding: '12px 16px', borderRadius: '8px', marginBottom: '32px' }}>
        <strong>Advanced feature:</strong> This section is intended for developers building custom integrations.
      </div>

      <p style={{ lineHeight: '1.6', color: '#444', marginBottom: '32px' }}>
        The OHC API is organized around REST. Our API has predictable resource-oriented URLs, returns JSON-encoded responses, and uses standard HTTP response codes, authentication, and verbs.
      </p>

      <h2 style={{ fontSize: '24px', borderBottom: '1px solid #eee', paddingBottom: '8px', marginBottom: '16px' }}>Authentication</h2>
      <p style={{ lineHeight: '1.6', color: '#444', marginBottom: '16px' }}>
        Authenticate your API requests by including your API key in the Authorization header.
      </p>
      <pre style={{ background: '#f8f9fa', padding: '16px', borderRadius: '8px', overflowX: 'auto', border: '1px solid #eaeaea' }}>
        <code>Authorization: Bearer YOUR_API_KEY</code>
      </pre>

      {/* Mock Swagger-like UI block */}
      <div style={{ marginTop: '40px', border: '1px solid #eaeaea', borderRadius: '8px', overflow: 'hidden' }}>
        <div style={{ background: '#f4f4f4', padding: '12px 16px', fontWeight: 'bold', borderBottom: '1px solid #eaeaea' }}>
          Endpoints
        </div>
        <div style={{ padding: '16px', borderBottom: '1px solid #eaeaea', display: 'flex', alignItems: 'center' }}>
          <span style={{ background: '#28a745', color: 'white', padding: '4px 8px', borderRadius: '4px', fontSize: '12px', fontWeight: 'bold', marginRight: '16px' }}>GET</span>
          <span style={{ fontFamily: 'monospace', fontSize: '14px' }}>/v1/store/products</span>
          <span style={{ marginLeft: 'auto', color: '#666', fontSize: '14px' }}>List all products</span>
        </div>
        <div style={{ padding: '16px', borderBottom: '1px solid #eaeaea', display: 'flex', alignItems: 'center' }}>
          <span style={{ background: '#007bff', color: 'white', padding: '4px 8px', borderRadius: '4px', fontSize: '12px', fontWeight: 'bold', marginRight: '16px' }}>POST</span>
          <span style={{ fontFamily: 'monospace', fontSize: '14px' }}>/v1/store/products</span>
          <span style={{ marginLeft: 'auto', color: '#666', fontSize: '14px' }}>Create a product</span>
        </div>
      </div>

      <div style={{ marginTop: '40px', textAlign: 'center' }}>
        <a href="/help" style={{ color: '#0056b3', textDecoration: 'none' }}>← Back to Help Center</a>
      </div>
    </div>
  );
}
