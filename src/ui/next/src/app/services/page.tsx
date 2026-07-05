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
      <section id="services-screen" className="app-panel glassmorphism">
        <div className="app-panel-header">
          <div>
            <div className="app-panel-title font-outfit text-[#1D1D1F] dark:text-[#F5F5F7]">Runtime Controls</div>
            <div className="app-list-subtitle font-inter text-[#86868B] dark:text-[#A1A1A6]">Resource usage: CPU 5%, memory 128MB.</div>
          </div>
          <span className={`app-badge ${status === 'running' ? 'good bg-[#34C759] dark:bg-[#00C24B] text-white' : 'warn bg-[#FF9500] dark:bg-[#FF9F1A] text-white'}`}>{status}</span>
        </div>

        <div className="app-panel-body">
          <p className="text-sm text-gray-600 dark:text-gray-400 font-inter" role="status">{message}</p>

          <div className="mt-6 flex flex-wrap items-center gap-4">
            <button
              type="button"
              className={`app-button min-h-[44px] px-6 py-3 rounded-xl font-bold font-inter text-white transition-all shadow-sm ${status === 'restarting' ? 'bg-gray-400 dark:bg-gray-600 cursor-not-allowed' : 'bg-[#0066FF] hover:bg-[#0052cc]'}`}
              onClick={restartService}
              disabled={status === 'restarting'}
            >
              {status === 'restarting' ? 'Restarting...' : 'Restart Service'}
            </button>

            <label className="inline-flex min-h-[44px] items-center gap-3 rounded-xl border border-white/50 dark:border-white/10 glassmorphism px-4 py-2 text-sm font-medium text-[#1D1D1F] dark:text-[#F5F5F7] cursor-pointer hover:bg-white dark:hover:bg-black/40 transition-colors">
              <input
                aria-label="Auto restart"
                type="checkbox"
                checked={autoRestart}
                className="w-5 h-5 accent-[#0066FF] rounded focus:ring-[#0066FF]"
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
