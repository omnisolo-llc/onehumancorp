'use client';

import React, { useEffect, useState } from 'react';
import Link from 'next/link';

interface ObservabilityMetrics {
  active_agents: number;
  pending_missions: number;
  avg_task_latency_ms: number;
  db_mode: string;
}

export default function SwarmObservabilityDashboard() {
  const [metrics, setMetrics] = useState<ObservabilityMetrics | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    async function fetchMetrics() {
      try {
        const response = await fetch('/api/v1/observability/metrics');
        if (response.ok) {
          const data = await response.json();
          setMetrics(data);
        }
      } catch (error) {
        console.error('Failed to fetch observability metrics:', error);
      } finally {
        setLoading(false);
      }
    }

    fetchMetrics();

    // Simulate real-time updates
    const interval = setInterval(fetchMetrics, 5000);
    return () => clearInterval(interval);
  }, []);

  return (
    <div className="min-h-screen p-8 font-inter" style={{ backgroundColor: '#16161A', color: '#F5F5F7' }}>
      <header className="mb-8 flex items-center justify-between">
        <div className="flex items-center gap-4">
          <Link href="/dashboard" className="text-blue-500 hover:text-blue-400 transition-colors">
            &lt; Back to Dashboard
          </Link>
          <h1 className="text-3xl font-bold font-outfit text-white">Swarm Observability Dashboard</h1>
        </div>
      </header>

      {loading && !metrics ? (
        <div className="flex items-center justify-center h-64">
          <div className="w-8 h-8 rounded-full border-2 border-white/20 border-t-blue-500 animate-spin"></div>
        </div>
      ) : (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
          <section className="shadow-lg rounded-[16px] p-6 transition-all duration-300 hover:transform hover:-translate-y-1" style={{ background: 'rgba(255, 255, 255, 0.05)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.1)' }}>
            <h2 className="text-sm font-semibold font-outfit mb-2 uppercase tracking-wider text-gray-400">Active Agents</h2>
            <div className="text-4xl font-bold text-white flex items-baseline gap-2">
              {metrics?.active_agents ?? '-'}
              <span className="text-sm font-normal text-green-400">Online</span>
            </div>
            <div className="text-xs text-gray-500 mt-4">Current swarm workforce count</div>
          </section>

          <section className="shadow-lg rounded-[16px] p-6 transition-all duration-300 hover:transform hover:-translate-y-1" style={{ background: 'rgba(255, 255, 255, 0.05)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.1)' }}>
            <h2 className="text-sm font-semibold font-outfit mb-2 uppercase tracking-wider text-gray-400">Pending Missions</h2>
            <div className="text-4xl font-bold text-white">
              {metrics?.pending_missions ?? '-'}
            </div>
            <div className="text-xs text-gray-500 mt-4">Tasks awaiting agent pickup</div>
          </section>

          <section className="shadow-lg rounded-[16px] p-6 transition-all duration-300 hover:transform hover:-translate-y-1" style={{ background: 'rgba(255, 255, 255, 0.05)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.1)' }}>
            <h2 className="text-sm font-semibold font-outfit mb-2 uppercase tracking-wider text-gray-400">Avg Task Latency</h2>
            <div className="text-4xl font-bold text-white flex items-baseline gap-1">
              {metrics?.avg_task_latency_ms ?? '-'} <span className="text-lg">ms</span>
            </div>
            <div className="text-xs text-gray-500 mt-4">Across all distributed operations</div>
          </section>

          <section className="shadow-lg rounded-[16px] p-6 transition-all duration-300 hover:transform hover:-translate-y-1" style={{ background: 'rgba(255, 255, 255, 0.05)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.1)' }}>
            <h2 className="text-sm font-semibold font-outfit mb-2 uppercase tracking-wider text-gray-400">Database Mode</h2>
            <div className="text-4xl font-bold text-white capitalize">
              {metrics?.db_mode ?? '-'}
            </div>
            <div className="text-xs text-gray-500 mt-4">Current architecture operation mode</div>
          </section>
        </div>
      )}

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}
