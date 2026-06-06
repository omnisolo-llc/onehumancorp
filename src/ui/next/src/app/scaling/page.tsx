"use client";

import React, { useState } from 'react';
import { AppShell } from '../components/AppShell';

export default function ScalingPage() {
  const [instances, setInstances] = useState(3);
  const [message, setMessage] = useState('No optimization needed.');
  const minInstances = 1;
  const maxInstances = 10;

  const updateScale = (nextInstances: number) => {
    const boundedInstances = Math.min(maxInstances, Math.max(minInstances, nextInstances));
    setInstances(boundedInstances);
    setMessage(
      boundedInstances === instances
        ? `Scale is already at the ${boundedInstances === minInstances ? 'minimum' : 'maximum'} bound.`
        : `Scaling configuration updated to ${boundedInstances} instances.`,
    );
  };

  return (
    <AppShell
      title="Scaling Configuration"
      subtitle="Tune the local service instance range for predictable demand."
      statusItems={[
        { label: 'Current Scale', value: String(instances), tone: instances > 6 ? 'warn' : 'good' },
        { label: 'Range', value: `${minInstances}-${maxInstances}`, tone: 'neutral' },
      ]}
    >
      <section id="scaling-screen" className="app-panel">
        <div className="app-panel-header">
          <div>
            <div className="app-panel-title">Instance Range</div>
            <div className="app-list-subtitle">Min {minInstances} / Max {maxInstances} instance bounds.</div>
          </div>
          <span className="app-badge good">Autoscale Ready</span>
        </div>

        <div className="app-panel-body">
          <div className="app-card">
            <div className="app-metric-label">Current Scale</div>
            <div className="mt-2 text-4xl font-bold text-gray-900">{instances} instances</div>
            <p className="mt-2 text-sm text-gray-600" role="status">{message}</p>
          </div>

          <div className="mt-6 flex flex-wrap gap-3">
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
              Increase Scale
            </button>
          </div>
        </div>
      </section>
    </AppShell>
  );
}
