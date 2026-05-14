import React from 'react';
import { HelpCenterWidget } from '../components/help/HelpCenterWidget';

export default function Home() {
  return (
    <main style={{ padding: '40px', fontFamily: 'Inter, sans-serif' }}>
      <h1>Small Business Dashboard</h1>
      <p>Welcome to your OneHumanCorp dashboard.</p>

      <div style={{ marginTop: '40px' }}>
        <h2>Quick Actions</h2>
        <button id="add-product-btn" style={{ padding: '10px 20px', marginRight: '10px' }}>Add Product</button>
        <button id="connect-bank-btn" style={{ padding: '10px 20px', marginRight: '10px' }}>Connect Bank</button>
      </div>

      {/* The Help Center widget is integrated here to satisfy user requirements for an in-app help portal */}
      <HelpCenterWidget />
    </main>
  );
}
