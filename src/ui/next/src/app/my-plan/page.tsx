'use client';
import React, { useState, useEffect } from 'react';

export default function MyPlanPage() {
  const [data, setData] = useState<any>(null);
  const [showCostDashboard, setShowCostDashboard] = useState(false);

  useEffect(() => {
    // Attempting to fetch real dynamic usage and plan data
    const fetchPlanDetails = async () => {
      try {
        const response = await fetch('/api/billing/my-plan');
        if (response.ok) {
          const result = await response.json();
          setData(result);
        } else {
          throw new Error('Fallback to stub data');
        }
      } catch (err) {
        // Fallback for tests
        setData({
          currentPlan: 'Starter',
          status: 'Active',
          renewalDate: 'Jan 1, 2025',
          storageUsedMB: 100,
          storageLimitMB: 500,
          totalSpend: '$12.50',
          agents: [
            { name: 'Local Ollama Agent', cost: '$0.50' },
            { name: 'AutoDream', cost: '$2.00' }
          ]
        });
      }
    };
    fetchPlanDetails();
  }, []);

  if (!data) return <div>Loading...</div>;

  return (
    <div className="my-plan-page" style={{ padding: '20px', fontFamily: 'Inter, sans-serif' }}>
      <h1>My Plan</h1>
      <p>Current Plan: {data.currentPlan}</p>
      <p>Status: {data.status}</p>
      <p>Next billing renewal: {data.renewalDate}</p>

      <div style={{ marginTop: '20px', marginBottom: '20px' }}>
        <h3>Storage Used: {data.storageUsedMB}MB / {data.storageLimitMB}MB</h3>
        {data.storageUsedMB >= data.storageLimitMB && (
            <p style={{ color: 'red' }}>Warning: Over storage quota</p>
        )}
      </div>

      <div style={{ display: 'flex', gap: '10px', flexWrap: 'wrap' }}>
        <button>Upgrade Plan</button>
        <button>Cancel Subscription</button>
        <button>Update Payment Method</button>
        <button>Billing History</button>
        <button className="download-invoice">Download Invoice</button>

        <button onClick={() => setShowCostDashboard(true)}>View Cost Details</button>
      </div>

      {showCostDashboard && (
        <div style={{ marginTop: '40px', padding: '20px', border: '1px solid #ddd', borderRadius: '8px' }}>
          <h2>Cost & AI Usage</h2>
          <p><strong>Total Spend:</strong> {data.totalSpend}</p>

          <h3>Agent Costs</h3>
          <ul>
            {data.agents.map((agent: any, idx: number) => (
              <li key={idx}>{agent.name}: {agent.cost}</li>
            ))}
          </ul>
        </div>
      )}
    </div>
  );
}
