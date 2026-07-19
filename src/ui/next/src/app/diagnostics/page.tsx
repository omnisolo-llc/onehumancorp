"use client";

import React, { useState, useEffect } from 'react';
import { AppShell } from '../components/AppShell';
import { Card, CardContent, CardHeader, CardTitle } from '../../components/ui/card';

export default function DiagnosticsPage() {
  const [loading, setLoading] = useState(true);
  const [healthData, setHealthData] = useState<any>(null);
  const [metricsData, setMetricsData] = useState<any>(null);

  useEffect(() => {
    async function loadData() {
      try {
        const [healthRes, metricsRes] = await Promise.all([
          fetch('/api/v1/health'),
          fetch('/api/v1/ui/dashboard/metrics')
        ]);

        if (healthRes.ok) {
          const healthJson = await healthRes.json();
          setHealthData(healthJson);
        }

        if (metricsRes.ok) {
          const metricsJson = await metricsRes.json();
          setMetricsData(metricsJson);
        }
      } catch (err) {
        console.error('Failed to load diagnostics', err);
      } finally {
        setLoading(false);
      }
    }

    loadData();
  }, []);

  if (loading) {
    return (
      <AppShell title="Diagnostics" subtitle="System operational and health telemetry.">
        <div className="flex items-center justify-center p-12">Loading...</div>
      </AppShell>
    );
  }

  return (
    <AppShell
      title="Diagnostics"
      subtitle="System operational and health telemetry."
    >
      <div id="diagnostics-screen" className="max-w-4xl mx-auto w-full flex flex-col gap-6 font-inter">
        <Card className="mb-6">
<CardHeader>
<CardTitle className="text-xl font-bold font-outfit text-gray-900 dark:text-white mb-4">
          Operational Telemetry</CardTitle>
</CardHeader>
<CardContent>
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4 mb-6">
            <div className="p-3 rounded-xl border border-gray-100 dark:border-zinc-800 bg-gray-50 dark:bg-zinc-800/50 text-sm">
               <span className="font-semibold text-gray-500">System Status:</span> {healthData?.status || 'Unknown'}
            </div>
            <div className="p-3 rounded-xl border border-gray-100 dark:border-zinc-800 bg-gray-50 dark:bg-zinc-800/50 text-sm">
               <span className="font-semibold text-gray-500">Mode:</span> {healthData?.mode || 'Unknown'}
            </div>
            <div className="p-3 rounded-xl border border-gray-100 dark:border-zinc-800 bg-gray-50 dark:bg-zinc-800/50 text-sm">
               <span className="font-semibold text-gray-500">Mesh Active:</span> {typeof healthData?.mesh_active === 'boolean' ? (healthData.mesh_active ? 'Yes' : 'No') : 'Unknown'}
            </div>
            <div className="p-3 rounded-xl border border-gray-100 dark:border-zinc-800 bg-gray-50 dark:bg-zinc-800/50 text-sm">
               <span className="font-semibold text-gray-500">Hybrid Mode Ready:</span> {typeof healthData?.hybrid_mode_ready === 'boolean' ? (healthData.hybrid_mode_ready ? 'Yes' : 'No') : 'Unknown'}
            </div>
          </div>

          <div className="space-y-4">
            <div className="p-4 bg-white dark:bg-zinc-800 rounded-xl border border-gray-100 dark:border-zinc-700 shadow-sm flex justify-between text-sm">
              <span className="font-medium text-gray-900 dark:text-white">Database Ping:</span> <span>{typeof healthData?.db_ping === 'number' ? `${healthData.db_ping} ms` : 'Unavailable'}</span>
            </div>
            <div className="p-4 bg-white dark:bg-zinc-800 rounded-xl border border-gray-100 dark:border-zinc-700 shadow-sm flex justify-between text-sm">
              <span className="font-medium text-gray-900 dark:text-white">Sync Backlog:</span> <span>{typeof healthData?.sync_backlog === 'number' ? healthData.sync_backlog : 'Unavailable'}</span>
            </div>
            <div className="p-4 bg-white dark:bg-zinc-800 rounded-xl border border-gray-100 dark:border-zinc-700 shadow-sm flex justify-between text-sm">
              <span className="font-medium text-gray-900 dark:text-white">Sync Errors:</span> <span>{typeof healthData?.sync_error_count === 'number' ? healthData.sync_error_count : 'Unavailable'}</span>
            </div>
          </div>
        </CardContent>
</Card>

        <Card className="mb-6">
<CardHeader>
<CardTitle className="text-xl font-bold font-outfit text-gray-900 dark:text-white mb-4">
          Business Telemetry</CardTitle>
</CardHeader>
<CardContent>
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4 mb-6">
            <div className="p-4 rounded-xl app-card ohc-growth-card">
                <h4 className="text-xs font-semibold text-gray-500 uppercase tracking-wider">Total Revenue</h4>
                <div className="text-2xl font-bold mt-1 text-gray-900 dark:text-white">{metricsData?.total_revenue ?? 'Unavailable'}</div>
            </div>
            <div className="p-4 rounded-xl app-card ohc-growth-card">
                <h4 className="text-xs font-semibold text-gray-500 uppercase tracking-wider">Total Sales</h4>
                <div className="text-2xl font-bold mt-1 text-gray-900 dark:text-white">{metricsData?.total_sales ?? 'Unavailable'}</div>
            </div>
          </div>

          <div className="h-40 bg-gray-50 dark:bg-zinc-800/50 rounded-xl flex items-center justify-center text-sm text-gray-500 border border-dashed border-gray-200 dark:border-zinc-700">
              Correlation data unavailable.
          </div>
        </CardContent>
</Card>

        <Card className="p-6">
          <CardContent className="flex flex-col md:flex-row items-center gap-4 p-0">
            <button disabled className="px-4 py-2.5 bg-gray-200 text-gray-500 font-semibold rounded-lg text-sm w-full md:w-auto border-none">Diagnostics actions unavailable</button>
            <div id="diagnostics-result" data-testid="diagnostics-result" className="text-sm font-semibold text-[#0f766e] dark:text-[#6ac5bd] mt-2 md:mt-0 flex-1 text-center md:text-left">
                Diagnostics actions are unavailable.
            </div>
          </CardContent>
        </Card>
      </div>
    </AppShell>
  );
}
