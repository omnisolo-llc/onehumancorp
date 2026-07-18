"use client";

import React, { useEffect, useState } from 'react';
import { AppShell } from '../components/AppShell';

type DashboardMetrics = {
  total_sales: number;
  active_customers: number;
  pending_orders: number;
  total_campaigns_sent: number;
};

export default function AnalyticsPage() {
  const [metrics, setMetrics] = useState<DashboardMetrics | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    fetch('/api/v1/ui/dashboard/metrics')
      .then((response) => {
        if (!response.ok) throw new Error('Analytics data is unavailable.');
        return response.json();
      })
      .then((data) => {
        const keys: Array<keyof DashboardMetrics> = ['total_sales', 'active_customers', 'pending_orders', 'total_campaigns_sent'];
        if (!keys.every((key) => typeof data[key] === 'number')) throw new Error('Analytics data is unavailable.');
        setMetrics(data);
      })
      .catch(() => setError('Analytics data is unavailable.'));
  }, []);

  return (
    <AppShell title="Business Analytics">
      <div className="mx-auto max-w-5xl space-y-8 font-inter">
        <header className="mb-8 p-6 bg-gradient-to-r from-indigo-50/50 to-purple-50/50 rounded-3xl border border-indigo-100/40 shadow-sm">
          <h1 className="text-3xl font-extrabold font-outfit text-gray-900">Business Analytics</h1>
          <p className="mt-2 text-sm text-gray-500">Recorded store metrics from the OHC dashboard service.</p>
        </header>

        {error && <p className="text-sm text-red-600" role="status">{error}</p>}
        {!metrics && !error && <p className="text-sm text-gray-500">Loading analytics…</p>}

        {metrics && (
          <section className="space-y-4">
            <h2 className="text-xl font-bold font-outfit text-gray-900">Core Metrics</h2>
            <div className="grid grid-cols-1 md:grid-cols-4 gap-6">
              <Metric label="Recorded Revenue" value={metrics.total_sales.toLocaleString(undefined, { style: 'currency', currency: 'USD' })} />
              <Metric label="Active Customers" value={metrics.active_customers.toLocaleString()} />
              <Metric label="Pending Orders" value={metrics.pending_orders.toLocaleString()} />
              <Metric label="Campaigns Sent" value={metrics.total_campaigns_sent.toLocaleString()} />
            </div>
          </section>
        )}

        <section className="app-card p-6 rounded-2xl border border-gray-200 bg-white/70">
          <h2 className="text-xl font-bold font-outfit text-gray-900">Advanced AI Insights</h2>
          <p className="mt-2 text-sm text-gray-600">Predictive analytics are unavailable because no forecast API is connected.</p>
        </section>
      </div>
    </AppShell>
  );
}

function Metric({ label, value }: { label: string; value: string }) {
  return (
    <div className="app-card p-6 rounded-2xl border border-indigo-100/40 bg-white/70">
      <div className="text-xs font-bold uppercase tracking-wider text-gray-400">{label}</div>
      <div className="text-3xl font-extrabold font-outfit text-gray-900 mt-2">{value}</div>
    </div>
  );
}
