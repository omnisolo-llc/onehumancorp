import React from 'react';
import CostDashboard from './CostDashboard';

export default function BillingPage() {
  return (
    <div style={{ maxWidth: '1200px', margin: '0 auto', padding: '2rem', fontFamily: 'Inter, sans-serif' }}>
      <h1 style={{ fontSize: '2.5rem', fontWeight: 'bold', marginBottom: '2rem' }}>Billing & Plans</h1>

      <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(250px, 1fr))', gap: '2rem', marginBottom: '4rem' }}>
        <div style={{ border: '1px solid #e2e8f0', borderRadius: '0.5rem', padding: '2rem', textAlign: 'center' }}>
          <h2 style={{ fontSize: '1.5rem', fontWeight: 'bold' }}>Free</h2>
          <p style={{ fontSize: '2rem', fontWeight: 'bold', margin: '1rem 0' }}>$0<span style={{ fontSize: '1rem', color: '#718096' }}>/mo</span></p>
          <ul style={{ listStyleType: 'none', padding: 0, margin: '0 0 2rem 0' }}>
            <li style={{ marginBottom: '0.5rem' }}>100 AI actions</li>
            <li style={{ marginBottom: '0.5rem' }}>500MB Storage</li>
            <li style={{ marginBottom: '0.5rem' }}>1 Agent</li>
          </ul>
          <button style={{ width: '100%', padding: '0.5rem 1rem', border: '1px solid #cbd5e0', borderRadius: '0.25rem', backgroundColor: '#f7fafc', cursor: 'pointer' }}>Current Plan</button>
        </div>

        <div style={{ border: '2px solid #4299e1', borderRadius: '0.5rem', padding: '2rem', textAlign: 'center', position: 'relative' }}>
          <div style={{ position: 'absolute', top: '-12px', left: '50%', transform: 'translateX(-50%)', backgroundColor: '#4299e1', color: 'white', padding: '0.25rem 0.5rem', borderRadius: '0.25rem', fontSize: '0.875rem' }}>Most Popular</div>
          <h2 style={{ fontSize: '1.5rem', fontWeight: 'bold' }}>Starter</h2>
          <p style={{ fontSize: '2rem', fontWeight: 'bold', margin: '1rem 0' }}>$9<span style={{ fontSize: '1rem', color: '#718096' }}>/mo</span></p>
          <ul style={{ listStyleType: 'none', padding: 0, margin: '0 0 2rem 0' }}>
            <li style={{ marginBottom: '0.5rem' }}>1,000 AI actions</li>
            <li style={{ marginBottom: '0.5rem' }}>5GB Storage</li>
            <li style={{ marginBottom: '0.5rem' }}>3 Agents</li>
          </ul>
          <button style={{ width: '100%', padding: '0.5rem 1rem', border: 'none', borderRadius: '0.25rem', backgroundColor: '#4299e1', color: 'white', cursor: 'pointer' }}>Upgrade to Starter</button>
        </div>

        <div style={{ border: '1px solid #e2e8f0', borderRadius: '0.5rem', padding: '2rem', textAlign: 'center' }}>
          <h2 style={{ fontSize: '1.5rem', fontWeight: 'bold' }}>Pro</h2>
          <p style={{ fontSize: '2rem', fontWeight: 'bold', margin: '1rem 0' }}>$29<span style={{ fontSize: '1rem', color: '#718096' }}>/mo</span></p>
          <ul style={{ listStyleType: 'none', padding: 0, margin: '0 0 2rem 0' }}>
            <li style={{ marginBottom: '0.5rem' }}>Unlimited AI actions</li>
            <li style={{ marginBottom: '0.5rem' }}>50GB Storage</li>
            <li style={{ marginBottom: '0.5rem' }}>10 Agents</li>
          </ul>
          <button style={{ width: '100%', padding: '0.5rem 1rem', border: 'none', borderRadius: '0.25rem', backgroundColor: '#4299e1', color: 'white', cursor: 'pointer' }}>Upgrade to Pro</button>
        </div>
      </div>

      <h2 style={{ fontSize: '2rem', fontWeight: 'bold', marginBottom: '1.5rem' }}>My Plan Details</h2>
      <CostDashboard />
    </div>
  );
}
