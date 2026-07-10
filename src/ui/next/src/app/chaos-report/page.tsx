'use client';
import React, { useState, useEffect } from 'react';

export default function ChaosReportPage() {
  const [data, setData] = useState<any>(null);
  const [isDarkMode, setIsDarkMode] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    // Check system preference for dark mode
    if (typeof window !== 'undefined') {
      setIsDarkMode(window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches);
    }

    fetch('/api/v1/chaos/report')
      .then(res => {
        if (!res.ok) throw new Error('Failed to fetch chaos report');
        return res.json();
      })
      .then(d => setData(d))
      .catch(e => {
        setError(e.message);
      });
  }, []);

  return (
    <div className={`min-h-screen ${isDarkMode ? 'dark bg-gray-900 text-white' : 'bg-[#F5F5F7] text-[#1D1D1F]'} p-8 font-inter transition-colors duration-300`}>
      <header className="mb-10 max-w-6xl mx-auto flex justify-between items-center">
        <div>
          <h1 className="text-4xl font-bold font-outfit tracking-tight">System Reliability Report</h1>
          <p className={`mt-2 ${isDarkMode ? 'text-gray-400' : 'text-gray-500'}`}>Chaos Engineering & Sentry Dashboard</p>
        </div>
        <button
          onClick={() => setIsDarkMode(!isDarkMode)}
          className={`px-4 py-2 rounded-full text-sm font-medium ${isDarkMode ? 'bg-white/10 hover:bg-white/20' : 'bg-black/5 hover:bg-black/10'} transition-colors`}
        >
          Toggle {isDarkMode ? 'Light' : 'Dark'} Mode
        </button>
      </header>

      <main className="max-w-6xl mx-auto grid grid-cols-1 lg:grid-cols-2 gap-8">
        <section className="col-span-1 lg:col-span-2 mb-8 p-6 rounded-2xl shadow-lg transition-all duration-300 relative overflow-hidden premium-glass">
            <h2 className="text-xl font-bold mb-4 tracking-tight">Chaos Resilience Metrics</h2>
            <div className="space-y-3 font-mono text-sm opacity-90">
                <div className="flex items-center gap-2">
                    <span className="w-2 h-2 rounded-full bg-[#0066FF]"></span>
                    <span>API Latency (P99) under 100 Cloud Users: <span className="font-bold">{data?.latencyP99Cloud || 'Loading...'}</span></span>
                </div>
                <div className="flex items-center gap-2">
                    <span className="w-2 h-2 rounded-full bg-indigo-500"></span>
                    <span>API Latency (P99) under 10 Standalone Users: <span className="font-bold">{data?.latencyP99Standalone || 'Loading...'}</span></span>
                </div>
                <div className="flex items-center gap-2">
                    <span className="w-2 h-2 rounded-full bg-[#34C759]"></span>
                    <span>Error Rate during LLM Outage: <span className="font-bold">{data?.errorRateLlmOutage || 'Loading...'}</span></span>
                </div>
            </div>
        </section>

        <section
          className="p-8 rounded-3xl shadow-lg transition-all duration-300 relative overflow-hidden premium-glass"
        >
          <div className="absolute top-0 right-0 p-6 opacity-20">
            <svg width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
              <polyline points="22 12 18 12 15 21 9 3 6 12 2 12"></polyline>
            </svg>
          </div>

          <h2 className="text-2xl font-semibold mb-6">Latency Distribution</h2>
          <p className={`mb-8 text-sm ${isDarkMode ? 'text-gray-300' : 'text-gray-600'}`}>
            Response times across global regions under simulated network degradation.
          </p>

          <div className="h-64 flex items-end gap-3 p-4 rounded-xl relative" style={{ background: isDarkMode ? 'rgba(0,0,0,0.3)' : 'rgba(0,0,0,0.03)' }}>
            <div className="absolute left-0 bottom-0 w-full h-full border-b border-l border-current opacity-10 m-4"></div>

            {error && <div className="absolute inset-0 flex items-center justify-center text-[#FF3B30] font-bold">{error}</div>}
            {!data && !error && <div className="absolute inset-0 flex items-center justify-center">Loading...</div>}

            {data?.latencyHistograms?.map((val: number, i: number) => (
              <div key={i} className="flex-1 flex flex-col items-center justify-end group">
                <div
                  className={`w-full rounded-t-md transition-all duration-500 ${isDarkMode ? 'bg-[#0066FF]/80' : 'bg-[#0066FF]/60'} group-hover:bg-blue-400 relative`}
                  style={{ height: `${Math.max(5, Math.min(100, val / 10))}%` }}
                >
                  <div className={`absolute -top-8 left-1/2 -translate-x-1/2 opacity-0 group-hover:opacity-100 transition-opacity text-xs font-bold px-2 py-1 rounded ${isDarkMode ? 'bg-gray-800 text-white' : 'bg-white/65 backdrop-blur-[30px] backdrop-saturate-[2.1] shadow-sm text-gray-900'}`}>
                    {val}ms
                  </div>
                </div>
                <span className={`text-[10px] mt-2 font-mono ${isDarkMode ? 'text-gray-400' : 'text-gray-500'}`}>p{10 + i*15}</span>
              </div>
            ))}
          </div>
        </section>

        <section
          className="p-8 rounded-3xl shadow-lg transition-all duration-300 relative overflow-hidden premium-glass"
        >
          <div className="absolute top-0 right-0 p-6 opacity-20">
            <svg width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
              <path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"></path>
              <line x1="12" y1="9" x2="12" y2="13"></line>
              <line x1="12" y1="17" x2="12.01" y2="17"></line>
            </svg>
          </div>

          <h2 className="text-2xl font-semibold mb-6">Error Rate Over Time</h2>
          <p className={`mb-8 text-sm ${isDarkMode ? 'text-gray-300' : 'text-gray-600'}`}>
            Failure rates during database node partition and cache exhaustion events.
          </p>

          <div className="h-64 relative p-4 rounded-xl flex items-end" style={{ background: isDarkMode ? 'rgba(0,0,0,0.3)' : 'rgba(0,0,0,0.03)' }}>
            <div className="absolute left-0 bottom-0 w-full h-full border-b border-l border-current opacity-10 m-4"></div>

            {error && <div className="absolute inset-0 flex items-center justify-center text-[#FF3B30] font-bold z-20">{error}</div>}
            {!data && !error && <div className="absolute inset-0 flex items-center justify-center z-20">Loading...</div>}

            <svg className="w-full h-full absolute inset-0 p-4" preserveAspectRatio="none" viewBox="0 0 100 100">
              <path
                d={data ? `M 0 100 ${data.errorRate?.map((val: number, i: number) => `L ${i * 25} ${100 - val * 1000}`).join(' ')}` : ''}
                fill="none"
                stroke={isDarkMode ? '#ef4444' : '#dc2626'}
                strokeWidth="2"
                strokeLinecap="round"
                strokeLinejoin="round"
                className="drop-shadow-sm"
              />
              <path
                d={data ? `M 0 100 ${data.errorRate?.map((val: number, i: number) => `L ${i * 25} ${100 - val * 1000}`).join(' ')} L 100 100 Z` : ''}
                fill={isDarkMode ? 'rgba(239, 68, 68, 0.1)' : 'rgba(220, 38, 38, 0.05)'}
              />
            </svg>

            {/* Markers */}
            <div className="absolute inset-0 p-4 w-full h-full">
              {data?.errorRate?.map((val: number, i: number) => (
                <div
                  key={i}
                  className={`absolute w-3 h-3 rounded-full -ml-1.5 -mb-1.5 cursor-pointer z-10 transition-transform hover:scale-150 ${isDarkMode ? 'bg-red-400 ring-2 ring-gray-900' : 'bg-[#FF3B30] ring-2 ring-white'}`}
                  style={{ left: `${i * 25}%`, bottom: `${val * 1000}%` }}
                  title={`${(val * 100).toFixed(1)}% error rate`}
                ></div>
              ))}
            </div>

            <div className="absolute bottom-1 w-full flex justify-between px-4 text-[10px] font-mono text-current opacity-50">
              <span>08:00</span>
              <span>08:15</span>
              <span>08:30</span>
              <span>08:45</span>
              <span>09:00</span>
            </div>
          </div>
        </section>
      </main>
    </div>
  );
}
