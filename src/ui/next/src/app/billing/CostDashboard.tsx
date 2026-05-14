import React, { useState, useEffect } from 'react';

interface PlanDetails {
  currentTier: string;
  aiActionsUsed: number;
  aiActionsLimit: number;
  storageUsedBytes: number;
  storageLimitBytes: number;
  estimatedNextBillUsd: number;
}

export default function CostDashboard() {
  const [plan, setPlan] = useState<PlanDetails | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    // Mocking an API call to the backend
    setTimeout(() => {
      setPlan({
        currentTier: 'Free',
        aiActionsUsed: 85,
        aiActionsLimit: 100,
        storageUsedBytes: 250 * 1024 * 1024,
        storageLimitBytes: 500 * 1024 * 1024,
        estimatedNextBillUsd: 0.0,
      });
      setLoading(false);
    }, 1000);
  }, []);

  if (loading) {
    return <div style={{ padding: '2rem', textAlign: 'center' }}>Loading your plan details...</div>;
  }

  if (!plan) {
    return <div>Failed to load plan details.</div>;
  }

  const aiPercentage = Math.min(100, (plan.aiActionsUsed / plan.aiActionsLimit) * 100);
  const storagePercentage = Math.min(100, (plan.storageUsedBytes / plan.storageLimitBytes) * 100);

  return (
    <div style={{ backgroundColor: '#f8fafc', padding: '2rem', borderRadius: '0.5rem', boxShadow: '0 4px 6px -1px rgba(0, 0, 0, 0.1)' }}>
      <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '2rem' }}>
        <div>
          <h3 style={{ fontSize: '1.25rem', color: '#4a5568', margin: 0 }}>Current Plan</h3>
          <p style={{ fontSize: '1.5rem', fontWeight: 'bold', margin: '0.5rem 0 0 0', color: '#2d3748' }}>{plan.currentTier}</p>
        </div>
        <div style={{ textAlign: 'right' }}>
          <h3 style={{ fontSize: '1.25rem', color: '#4a5568', margin: 0 }}>Estimated Next Bill</h3>
          <p style={{ fontSize: '1.5rem', fontWeight: 'bold', margin: '0.5rem 0 0 0', color: '#48bb78' }}>${plan.estimatedNextBillUsd.toFixed(2)}</p>
        </div>
      </div>

      <div style={{ marginBottom: '2rem' }}>
        <div style={{ display: 'flex', justifyContent: 'space-between', marginBottom: '0.5rem' }}>
          <span style={{ fontWeight: '500' }}>AI Actions</span>
          <span>{plan.aiActionsUsed} / {plan.aiActionsLimit} used</span>
        </div>
        <div style={{ width: '100%', backgroundColor: '#e2e8f0', borderRadius: '9999px', height: '0.75rem' }}>
          <div
            style={{
              backgroundColor: aiPercentage > 90 ? '#e53e3e' : '#4299e1',
              height: '100%',
              borderRadius: '9999px',
              width: `${aiPercentage}%`
            }}
          ></div>
        </div>
        {aiPercentage > 90 && (
          <p style={{ color: '#e53e3e', fontSize: '0.875rem', marginTop: '0.5rem' }}>
            You are approaching your AI action limit. Consider upgrading your plan to avoid interruption.
          </p>
        )}
      </div>

      <div>
        <div style={{ display: 'flex', justifyContent: 'space-between', marginBottom: '0.5rem' }}>
          <span style={{ fontWeight: '500' }}>Storage</span>
          <span>{(plan.storageUsedBytes / (1024 * 1024)).toFixed(1)} MB / {(plan.storageLimitBytes / (1024 * 1024)).toFixed(0)} MB used</span>
        </div>
        <div style={{ width: '100%', backgroundColor: '#e2e8f0', borderRadius: '9999px', height: '0.75rem' }}>
          <div
            style={{
              backgroundColor: storagePercentage > 90 ? '#e53e3e' : '#48bb78',
              height: '100%',
              borderRadius: '9999px',
              width: `${storagePercentage}%`
            }}
          ></div>
        </div>
      </div>
    </div>
  );
}
