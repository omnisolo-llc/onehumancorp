'use client';
import React, { useState, useEffect } from 'react';

export default function HomePage() {
  const [mounted, setMounted] = useState(false);

  useEffect(() => {
    setMounted(true);
  }, []);

  if (!mounted) return null;

  return (
    <div style={{
      minHeight: '100vh',
      background: '#0a0a0a',
      color: '#fff',
      fontFamily: 'Inter, sans-serif',
      display: 'flex',
      flexDirection: 'column',
      alignItems: 'center',
      justifyContent: 'center',
      padding: '20px'
    }}>
      <style>{`
        @keyframes fadeUp {
          from { opacity: 0; transform: translateY(20px); }
          to { opacity: 1; transform: translateY(0); }
        }
        .animate-card {
          animation: fadeUp 0.6s cubic-bezier(0.4, 0, 0.2, 1) forwards;
        }
      `}</style>

      <div className="animate-card" style={{
        backdropFilter: 'blur(20px) saturate(200%)',
        background: 'rgba(255, 255, 255, 0.05)',
        border: '1px solid rgba(255,255,255,0.1)',
        borderRadius: '16px',
        padding: '50px',
        textAlign: 'center',
        maxWidth: '600px',
        boxShadow: '0 20px 40px rgba(0,0,0,0.5)'
      }}>
        <div style={{
          width: '60px',
          height: '60px',
          background: 'linear-gradient(135deg, #4fc3f7, #2196f3)',
          borderRadius: '12px',
          margin: '0 auto 20px auto',
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'center',
          boxShadow: '0 10px 20px rgba(33, 150, 243, 0.3)'
        }}>
          <span style={{ fontSize: '24px', fontWeight: 'bold', fontFamily: 'Outfit, sans-serif' }}>OHC</span>
        </div>

        <h1 style={{ fontFamily: 'Outfit, sans-serif', fontSize: '36px', margin: '0 0 10px 0', letterSpacing: '-0.5px' }}>OneHumanCorp</h1>
        <p style={{ color: '#aaa', marginBottom: '40px', fontSize: '16px', lineHeight: '1.5' }}>
          Your Hybrid AI Swarm OS. Orchestrate autonomous agents, manage state, and build intelligent business workflows locally or in the cloud.
        </p>

        <div style={{ display: 'flex', flexDirection: 'column', gap: '15px' }}>
          <a href="/walkthroughs/kairos_orchestration" id="nav-walkthrough" style={{
            padding: '16px 24px',
            borderRadius: '12px',
            border: '1px solid rgba(79, 195, 247, 0.3)',
            background: 'rgba(79, 195, 247, 0.1)',
            color: '#4fc3f7',
            cursor: 'pointer',
            fontFamily: 'Outfit, sans-serif',
            fontWeight: '600',
            fontSize: '16px',
            textDecoration: 'none',
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'space-between',
            transition: 'all 0.2s cubic-bezier(0.4, 0, 0.2, 1)'
          }}
          onMouseEnter={e => e.currentTarget.style.background = 'rgba(79, 195, 247, 0.2)'}
          onMouseLeave={e => e.currentTarget.style.background = 'rgba(79, 195, 247, 0.1)'}
          >
            <span>KAIROS Orchestration Walkthrough</span>
            <span>→</span>
          </a>
          <a href="/business_manager" id="nav-dashboard" style={{
            padding: '16px 24px',
            borderRadius: '12px',
            border: '1px solid rgba(255, 255, 255, 0.1)',
            background: 'rgba(255, 255, 255, 0.05)',
            color: '#fff',
            cursor: 'pointer',
            fontFamily: 'Outfit, sans-serif',
            fontWeight: '600',
            fontSize: '16px',
            textDecoration: 'none',
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'center',
            transition: 'all 0.2s cubic-bezier(0.4, 0, 0.2, 1)'
          }}>
            <button style={{ background: 'transparent', border: 'none', color: '#fff', fontSize: '16px', fontFamily: 'Outfit, sans-serif', cursor: 'pointer' }}>Open Business Manager</button>
          </a>
        </div>
      </div>

      <div style={{ marginTop: '40px', color: '#666', fontSize: '12px' }}>
        v0.4.41 • Hybrid Agentic Master Blueprint
      </div>
    </div>
  );
}
