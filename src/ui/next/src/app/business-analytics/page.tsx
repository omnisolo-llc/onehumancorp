"use client";

import React, { useEffect, useState } from 'react';
import { AppShell } from '../components/AppShell';

type Metrics = {
  total_sales: number;
  active_customers: number;
  pending_orders: number;
  total_campaigns_sent: number;
};

export default function BusinessAnalytics() {
  const [metrics, setMetrics] = useState<Metrics | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    fetch('/api/v1/ui/dashboard/metrics')
      .then((response) => {
        if (!response.ok) throw new Error('Business metrics are unavailable.');
        return response.json();
      })
      .then((data) => {
        if (typeof data.total_sales !== 'number' || typeof data.active_customers !== 'number'
          || typeof data.pending_orders !== 'number' || typeof data.total_campaigns_sent !== 'number') {
          throw new Error('Business metrics are unavailable.');
        }
        setMetrics(data);
      })
      .catch(() => setError('Business metrics are unavailable.'));
  }, []);

  return (
    <AppShell title="Business Analytics" subtitle="Core store performance metrics." actions={[{ label: 'Back to Dashboard', href: '/dashboard' }]}>
      <div className="flex-1 max-w-6xl mx-auto w-full flex flex-col gap-8 font-inter">
        {error && <p className="text-sm text-red-600" role="status">{error}</p>}
        {!metrics && !error && <p className="text-sm text-gray-500">Loading business metrics…</p>}
        {metrics && (
          <section>
            <h2 className="text-xl font-bold font-outfit text-gray-900 dark:text-white mb-4">Core Performance</h2>
            <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
              <Metric label="Recorded Revenue" value={metrics.total_sales.toLocaleString(undefined, { style: 'currency', currency: 'USD' })} />
              <Metric label="Active Customers" value={String(metrics.active_customers)} />
              <Metric label="Pending Orders" value={String(metrics.pending_orders)} />
              <Metric label="Campaigns Sent" value={String(metrics.total_campaigns_sent)} />
            </div>
          </section>
        )}
        <section className="app-card p-6 rounded-2xl border border-gray-200 bg-white/70">
          <h2 className="text-xl font-bold font-outfit text-gray-900">Predictive AI Growth Trends</h2>
          <p className="mt-2 text-sm text-gray-600">Forecasts and cohort analytics are unavailable because no predictive analytics API is connected.</p>
        </section>
      </div>
    </AppShell>
  );
}

function Metric({ label, value }: { label: string; value: string }) {
  return (
    <div className="app-card p-5 rounded-2xl border border-white/40 bg-white/70">
      <div className="text-sm font-medium text-gray-500 mb-1">{label}</div>
      <div className="text-2xl font-bold font-outfit text-gray-900">{value}</div>
    </div>
  );
}
