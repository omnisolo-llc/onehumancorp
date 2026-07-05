"use client";

import React, { useState } from 'react';
import { AppShell } from '../components/AppShell';

export default function ScalingPage() {
  const [instances, setInstances] = useState(3);
  const [message, setMessage] = useState('No optimization needed.');
  const [taskPayload, setTaskPayload] = useState('Analyze dataset');
  const [loading, setLoading] = useState(false);
  const [results, setResults] = useState<string[]>([]);
  const minInstances = 1;
  const maxInstances = 1000;

  const updateScale = (nextInstances: number) => {
    const boundedInstances = Math.min(maxInstances, Math.max(minInstances, nextInstances));
    setInstances(boundedInstances);
    setMessage(
      boundedInstances === instances
        ? `Scale is already at the ${boundedInstances === minInstances ? 'minimum' : 'maximum'} bound.`
        : `Scaling configuration updated to ${boundedInstances} instances.`,
    );
  };

  const runDeployment = async () => {
    setLoading(true);
    setResults([]);
    try {
      const res = await fetch('/api/scaling', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ count: instances, message: taskPayload }),
      });
      if (!res.ok) {
        throw new Error('Failed to run scalable deployment');
      }
      const data = await res.json();
      setResults(data.outputs || []);
    } catch (e: any) {
      setResults([`Error: ${e.message}`]);
    } finally {
      setLoading(false);
    }
  };

  return (
    <AppShell
      title="Scalable Multi-Agent Deployment"
      subtitle="Single-user CLI to 1000+ agent cloud deployments."
      statusItems={[
        { label: 'Current Scale', value: String(instances), tone: instances > 10 ? 'warn' : 'good' },
        { label: 'Mode', value: instances > 10 ? 'Cloud Distributed' : 'Local CLI', tone: 'neutral' },
      ]}
    >
      <section id="scaling-screen" className="">
        <div className="app-panel-header">
          <div>
            <div className="app-panel-title">Instance Range</div>
            <div className="app-list-subtitle">Scale up to 1000 agents to execute task.</div>
          </div>
          <span className="app-badge good">Autoscale Ready</span>
        </div>

        <div className="glass-panel glassmorphism bg-white backdrop-blur-[30px] saturate-[210%] border border-white/40 p-6 mt-4">
          <div className="glassmorphism bg-white backdrop-blur-[30px] saturate-[210%] border border-white/40 p-6 mb-6">
            <div className="app-metric-label">Current Scale</div>
            <div className="mt-2 text-4xl font-bold text-gray-900">{instances} agents</div>
            <p className="mt-2 text-sm text-gray-600" role="status">{message}</p>
          </div>

          <div className="mt-6 flex flex-wrap gap-3 mb-6">
            <button
              type="button"
              className="app-button"
              onClick={() => updateScale(instances - 1)}
              disabled={instances <= minInstances}
            >
              Decrease Scale
            </button>
            <button
              type="button"
              className="app-button primary"
              onClick={() => updateScale(instances + 1)}
              disabled={instances >= maxInstances}
            >
              Increase Scale (+1)
            </button>
            <button
              type="button"
              className="app-button primary"
              onClick={() => updateScale(instances + 10)}
              disabled={instances >= maxInstances}
            >
              Increase Scale (+10)
            </button>
            <button
              type="button"
              className="app-button primary"
              onClick={() => updateScale(1000)}
              disabled={instances >= maxInstances}
            >
              Max Scale (1000)
            </button>
          </div>

          <div className="mb-6">
            <label className="block text-sm font-medium text-gray-700 mb-2">Task Payload</label>
            <input
              type="text"
              value={taskPayload}
              onChange={(e) => setTaskPayload(e.target.value)}
              className="w-full p-3 border border-gray-300 rounded-md"
              placeholder="e.g. Analyze dataset"
            />
          </div>

          <button
            id="deploy-agents-btn"
            type="button"
            className="app-button primary w-full py-3 text-lg justify-center mb-6"
            onClick={runDeployment}
            disabled={loading}
          >
            {loading ? 'Deploying...' : 'Deploy Scalable Multi-Agent Fleet'}
          </button>

          {results.length > 0 && (
            <div className="glassmorphism bg-white backdrop-blur-[30px] saturate-[210%] border border-white/40 p-6">
              <h3 className="font-bold text-lg mb-4">Results ({results.length} outputs)</h3>
              <div className="max-h-60 overflow-y-auto bg-transparent p-4 rounded-md border border-gray-200">
                {results.slice(0, 20).map((r, i) => (
                  <div key={i} className="text-sm mb-2 pb-2 border-b border-gray-200 last:border-0 last:mb-0 last:pb-0">
                    <span className="font-semibold text-[#0071E3]">Agent {i + 1}: </span>
                    {r}
                  </div>
                ))}
                {results.length > 20 && (
                  <div className="text-sm text-gray-500 italic">... and {results.length - 20} more results not shown.</div>
                )}
              </div>
            </div>
          )}
        </div>
      </section>
    </AppShell>
  );
}
