"use client";
import React, { useState } from 'react';

const articles = [
  { title: 'Getting Started', content: 'Welcome to OHC! To begin, set up your profile and create your first store.' },
  { title: 'My Store', content: 'Manage your products, view inventory, and update your store settings easily.' },
  { title: 'Payments', content: 'Link your bank account to start receiving money securely and instantly.' },
  { title: 'AI Agents', content: 'Activate your AI Support Agent to help answer customer questions 24/7.' }
];

export const HelpCenter: React.FC = () => {
  const [search, setSearch] = useState('');

  const filtered = articles.filter(a => a.title.toLowerCase().includes(search.toLowerCase()) || a.content.toLowerCase().includes(search.toLowerCase()));

  return (
    <div style={{ padding: '40px', maxWidth: '800px', margin: '0 auto', fontFamily: 'Outfit, sans-serif' }}>
      <h1 style={{ fontSize: '32px', marginBottom: '20px' }}>How can we help you today?</h1>
      <input
        type="text"
        placeholder="Search for answers..."
        value={search}
        onChange={(e) => setSearch(e.target.value)}
        style={{ width: '100%', padding: '15px', fontSize: '16px', borderRadius: '12px', border: '1px solid #ddd', marginBottom: '30px', boxShadow: '0 2px 8px rgba(0,0,0,0.05)' }}
      />

      <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(300px, 1fr))', gap: '20px' }}>
        {filtered.map((a, i) => (
          <div key={i} style={{ padding: '20px', background: '#fff', borderRadius: '12px', border: '1px solid #eaeaea', boxShadow: '0 4px 12px rgba(0,0,0,0.05)' }}>
            <h3 style={{ marginTop: 0 }}>{a.title}</h3>
            <p style={{ color: '#555', lineHeight: '1.5' }}>{a.content}</p>
          </div>
        ))}
      </div>

      <div style={{ marginTop: '50px' }}>
        <h2>Video Tutorials</h2>
        <div style={{ display: 'flex', gap: '20px', overflowX: 'auto', paddingBottom: '10px' }}>
          {[1, 2, 3].map(i => (
            <div key={i} style={{ minWidth: '200px', height: '350px', background: '#000', borderRadius: '12px', display: 'flex', alignItems: 'center', justifyContent: 'center', color: '#fff', flexDirection: 'column' }}>
              <div style={{ width: '40px', height: '40px', borderRadius: '20px', background: 'rgba(255,255,255,0.2)', display: 'flex', alignItems: 'center', justifyContent: 'center', marginBottom: '10px' }}>▶</div>
              <span>Video {i}</span>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
};
