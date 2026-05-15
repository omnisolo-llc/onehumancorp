'use client';
import React, { useState } from 'react';

export default function WalkthroughPage() {
  const [activeTab, setActiveTab] = useState(0);

  return (
    <div style={{ minHeight: '100vh', background: '#050505', color: '#fff', paddingBottom: '60px' }}>
      <header style={{
        fontFamily: 'Outfit, sans-serif',
        backdropFilter: 'blur(20px) saturate(200%)',
        background: 'rgba(255, 255, 255, 0.03)',
        borderBottom: '1px solid rgba(255, 255, 255, 0.1)',
        padding: '20px 40px',
        display: 'flex',
        justifyContent: 'space-between',
        alignItems: 'center',
        position: 'sticky',
        top: 0,
        zIndex: 100
      }}>
        <div>
          <h1 style={{ margin: 0, fontSize: '24px', color: '#fff' }}>KAIROS Walkthrough</h1>
          <p style={{ margin: '5px 0 0 0', fontSize: '14px', color: '#ccc', fontFamily: 'Inter, sans-serif' }}>
            Interactive Documentation & System Architecture Explorer
          </p>
        </div>
        <a href="/" style={{
          padding: '10px 20px', background: 'rgba(255,255,255,0.1)', color: '#fff',
          textDecoration: 'none', borderRadius: '8px', fontSize: '14px', transition: 'background 0.2s', fontFamily: 'Outfit, sans-serif'
        }}>
          ← Back to Home
        </a>
      </header>

      <main style={{ padding: '0 40px', maxWidth: '1000px', margin: '0 auto', marginTop: '40px' }}>
        <section id="architecture" style={{ background: 'rgba(255,255,255,0.03)', padding: '30px', borderRadius: '16px', border: '1px solid rgba(255,255,255,0.1)' }}>
          <h2 style={{ fontFamily: 'Outfit, sans-serif', marginTop: 0 }}>1. The KAIROS Triad Architecture</h2>
          <svg viewBox="0 0 600 500" width="100%" height="auto" style={{ border: '1px solid rgba(255,255,255,0.1)', background: 'rgba(0,0,0,0.3)', borderRadius: '12px' }}>
             <circle cx="300" cy="250" r="50" fill="#9C27B0" />
             <text x="300" y="255" fill="#fff" textAnchor="middle">KAIROS Orchestrator</text>
          </svg>
        </section>

        <section id="api" style={{ background: 'rgba(255,255,255,0.03)', padding: '30px', borderRadius: '16px', border: '1px solid rgba(255,255,255,0.1)', marginTop: '40px' }}>
           <h2 style={{ fontFamily: 'Outfit, sans-serif', marginTop: 0 }}>2. Interactive API Explorer</h2>
           <div onClick={() => setActiveTab(1)} style={{ padding: '10px', background: 'rgba(255,255,255,0.05)', borderRadius: '8px', cursor: 'pointer' }}>/api/v1/kairos/mesh/health</div>
           <div onClick={() => setActiveTab(2)} style={{ padding: '10px', background: 'rgba(255,255,255,0.05)', borderRadius: '8px', cursor: 'pointer', marginTop: '10px' }}>/api/v1/kairos/memory/consolidate</div>
           <p style={{ marginTop: '20px' }}>
             {activeTab === 1 ? 'Retrieves the health status of all connected Swarm agents.' :
              activeTab === 2 ? 'Forces an AutoDream consolidation cycle immediately.' :
              'Select an endpoint above.'}
           </p>
        </section>

        <section id="timeline" style={{ background: 'rgba(255,255,255,0.03)', padding: '30px', borderRadius: '16px', border: '1px solid rgba(255,255,255,0.1)', marginTop: '40px' }}>
           <h2 style={{ fontFamily: 'Outfit, sans-serif', marginTop: 0 }}>3. Distributed Event Timeline</h2>
           <ul>
             <li>Lock acquired</li>
             <li>AutoDream vector generated</li>
           </ul>
        </section>
      </main>
    </div>
  );
}
