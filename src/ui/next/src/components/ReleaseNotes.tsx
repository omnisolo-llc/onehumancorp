"use client";
import React from 'react';

export const ReleaseNotes: React.FC = () => {
  return (
    <div style={{ padding: '40px', maxWidth: '800px', margin: '0 auto', fontFamily: 'Outfit, sans-serif' }}>
      <h1>What's New in OHC</h1>

      <div style={{ marginTop: '30px', borderLeft: '3px solid #0070f3', paddingLeft: '20px' }}>
        <h2 style={{ marginBottom: '5px' }}>Easier Store Setup</h2>
        <span style={{ color: '#888', fontSize: '14px' }}>October 2023</span>
        <p style={{ marginTop: '10px', lineHeight: '1.6' }}>We have redesigned the store setup process to be faster and simpler. Now you can get your store online in just 3 easy steps. No technical knowledge required!</p>
      </div>

      <div style={{ marginTop: '30px', borderLeft: '3px solid #0070f3', paddingLeft: '20px' }}>
        <h2 style={{ marginBottom: '5px' }}>AI Helper</h2>
        <span style={{ color: '#888', fontSize: '14px' }}>September 2023</span>
        <p style={{ marginTop: '10px', lineHeight: '1.6' }}>Say hello to your new AI assistant. Click the chat button in the bottom right to ask any question about using OHC.</p>
      </div>
    </div>
  );
};
