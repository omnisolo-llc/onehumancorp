"use client";

import React, { useState } from 'react';
import { AppShell } from '../components/AppShell';

export default function ServicesPage() {
  const [status, setStatus] = useState('running');
  const [autoRestart, setAutoRestart] = useState(true);
  const [message, setMessage] = useState('All services are healthy.');

  const restartService = () => {
    setStatus('restarting');
    setMessage('Restart requested for the service manager.');
    window.setTimeout(() => {
      setStatus('running');
      setMessage('Service manager restarted successfully.');
    }, 250);
  };

  return (
    <AppShell
      title="Service Manager"
      subtitle="Monitor runtime health and restart policy for local services."
      statusItems={[
        { label: 'Status', value: status, tone: status === 'running' ? 'good' : 'warn' },
        { label: 'CPU', value: '5%', tone: 'good' },
        { label: 'Memory', value: '128MB', tone: 'good' },
      ]}
    >
      <section id="services-screen" className="app-panel">
        <div className="app-panel-header">
          <div>
            <div className="app-panel-title">Runtime Controls</div>
            <div className="app-list-subtitle">Resource usage: CPU 5%, memory 128MB.</div>
          </div>
          <span className={`app-badge ${status === 'running' ? 'good' : 'warn'}`}>{status}</span>
        </div>

        <div className="app-panel-body">
          <p className="text-sm text-gray-600" role="status">{message}</p>

          <div className="mt-6 flex flex-wrap items-center gap-4">
            <button
              type="button"
              className="app-button primary"
              onClick={restartService}
              disabled={status === 'restarting'}
            >
              {status === 'restarting' ? 'Restarting...' : 'Restart Service'}
            </button>

            <label className="inline-flex min-h-[44px] items-center gap-3 rounded-xl border border-gray-200 bg-white px-4 py-2 text-sm font-medium text-gray-800">
              <input
                aria-label="Auto restart"
                type="checkbox"
                checked={autoRestart}
                onChange={(event) => {
                  setAutoRestart(event.target.checked);
                  setMessage(event.target.checked ? 'Auto restart enabled.' : 'Auto restart disabled.');
                }}
              />
              Auto restart
            </label>
          </div>
        </div>
      </section>
    </AppShell>
  );
}
