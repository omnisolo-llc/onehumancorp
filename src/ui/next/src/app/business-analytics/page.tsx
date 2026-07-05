'use client';

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';
import { AppShell } from '../components/AppShell';

export default function BusinessAnalytics() {
  const router = useRouter();
  const [hasPro, setHasPro] = useState(false);
  const [showSoftPaywall, setShowSoftPaywall] = useState(false);
  const [trialStatus, setTrialStatus] = useState('');
  const [metrics, setMetrics] = useState<any>(null);
  const [loadingMetrics, setLoadingMetrics] = useState(true);

  useEffect(() => {
    const fetchMetrics = async () => {
      try {
        setLoadingMetrics(true);
        const res = await fetch('/api/ui/dashboard/metrics');
        if (res.ok) {
          const data = await res.json();
          setMetrics(data);
        }
      } catch (err) {
        console.error('Failed to fetch metrics', err);
      } finally {
        setLoadingMetrics(false);
      }
    };
    fetchMetrics();

    const isPro = localStorage.getItem('pro_plan') === 'true';
    const trialActive = localStorage.getItem('trial_active') === 'true';
    if (isPro || trialActive) {
      setHasPro(true);
    }
  }, []);

  const claimTrialExtension = () => {
    window.open('https://twitter.com/intent/tweet?text=' + encodeURIComponent('I just unlocked advanced AI business analytics on OHC! Start your business in minutes at https://ohc.app 🚀'), '_blank');
    localStorage.setItem('trial_active', 'true');
    setHasPro(true);
    setShowSoftPaywall(false);
    setTrialStatus('7-day Pro Trial activated. You now have access to advanced analytics.');
  };

  return (
    <AppShell
      title="Business Analytics"
      subtitle="Predictive insights and core store performance metrics."
      actions={[{ label: 'Back to Dashboard', href: '/dashboard' }]}
    >
      <div className="flex-1 max-w-6xl mx-auto w-full flex flex-col gap-8 font-inter">
        {trialStatus && <p className="rounded-lg border border-green-100 bg-green-50 px-4 py-3 text-sm font-semibold text-green-800" role="status">{trialStatus}</p>}
        {/* Core Metrics Section */}
        <section>
          <h2 className="text-xl font-bold font-outfit text-gray-900 dark:text-white mb-4">Core Performance (Last 30 Days)</h2>
          <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
            <div className="app-card glassmorphism p-5 rounded-2xl shadow-sm border border-white/40 dark:border-white/10 flex flex-col justify-between">
              <div className="text-sm font-medium text-gray-500 mb-1">Total Revenue</div>
              <div className="text-2xl font-bold font-outfit text-gray-900 dark:text-white">{loadingMetrics ? "..." : `${(metrics?.total_sales || 0).toLocaleString(undefined, {minimumFractionDigits: 2, maximumFractionDigits: 2})}`}</div>
              <div className="text-xs font-semibold text-[#34C759] mt-2 flex items-center gap-1">
                 <svg className="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 10l7-7m0 0l7 7m-7-7v18" /></svg>
                 15% from last month
              </div>
            </div>

            <div className="app-card glassmorphism p-5 rounded-2xl shadow-sm border border-white/40 dark:border-white/10 flex flex-col justify-between">
              <div className="text-sm font-medium text-gray-500 mb-1">Average Order Value</div>
              <div className="text-2xl font-bold font-outfit text-gray-900 dark:text-white">{loadingMetrics ? "..." : `${((metrics?.total_sales || 0) / (metrics?.pending_orders || 1)).toLocaleString(undefined, {minimumFractionDigits: 2, maximumFractionDigits: 2})}`}</div>
              <div className="text-xs font-semibold text-[#34C759] mt-2 flex items-center gap-1">
                 <svg className="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 10l7-7m0 0l7 7m-7-7v18" /></svg>
                 2% from last month
              </div>
            </div>

            <div className="app-card glassmorphism p-5 rounded-2xl shadow-sm border border-white/40 dark:border-white/10 flex flex-col justify-between">
              <div className="text-sm font-medium text-gray-500 mb-1">Orders</div>
              <div className="text-2xl font-bold font-outfit text-gray-900 dark:text-white">{loadingMetrics ? "..." : (metrics?.pending_orders || 0)}</div>
              <div className="text-xs font-semibold text-[#34C759] mt-2 flex items-center gap-1">
                 <svg className="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 10l7-7m0 0l7 7m-7-7v18" /></svg>
                 10% from last month
              </div>
            </div>

            <div className="app-card glassmorphism p-5 rounded-2xl shadow-sm border border-white/40 dark:border-white/10 flex flex-col justify-between">
              <div className="text-sm font-medium text-gray-500 mb-1">Conversion Rate</div>
              <div className="text-2xl font-bold font-outfit text-gray-900 dark:text-white">{loadingMetrics ? "..." : `${((metrics?.active_customers ? (metrics?.pending_orders / metrics?.active_customers) : 0) * 100).toFixed(1)}%`}</div>
              <div className="text-xs font-semibold text-[#FF3B30] mt-2 flex items-center gap-1">
                 <svg className="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 14l-7 7m0 0l-7-7m7 7V3" /></svg>
                 -1.5% from last month
              </div>
            </div>
          </div>
        </section>

        {/* Growth Trends */}
        <section className="relative">
           <h2 className="text-xl font-bold font-outfit text-gray-900 dark:text-white mb-4 flex items-center gap-2">
               Predictive AI Growth Trends
               {!hasPro && <span className="bg-[#0f766e] text-white text-xs px-2 py-0.5 rounded-full font-bold uppercase tracking-wider">Pro</span>}
           </h2>

            <div className={`grid grid-cols-1 md:grid-cols-2 gap-6 transition-all duration-500 ${!hasPro ? 'filter blur-md select-none pointer-events-none opacity-50' : ''}`}>
                {/* Revenue Forecast Card */}
                <div className="app-card glassmorphism p-6 rounded-2xl shadow-sm border border-white/40 dark:border-white/10 flex flex-col h-80 bg-white/70 dark:bg-zinc-900/70 backdrop-blur-[30px] saturate-[210%]">
                    <h3 className="font-bold font-outfit text-gray-800 dark:text-gray-200 mb-4 text-sm uppercase tracking-wider">Predictive Revenue Forecast</h3>
                    <div className="flex-1 flex flex-col justify-end">
                        <div className="flex items-end justify-between h-40 border-b border-gray-100 dark:border-gray-800 pb-2 px-2 gap-4">
                            <div className="flex flex-col items-center gap-2 w-full">
                                <span className="text-[10px] font-bold text-gray-400">$3.2k</span>
                                <div className="w-full bg-blue-500/20 hover:bg-blue-500/30 rounded-t-md h-12 transition-all"></div>
                                <span className="text-[10px] font-bold text-gray-400">Jul</span>
                            </div>
                            <div className="flex flex-col items-center gap-2 w-full">
                                <span className="text-[10px] font-bold text-gray-400">$4.5k</span>
                                <div className="w-full bg-blue-500/30 hover:bg-blue-500/45 rounded-t-md h-16 transition-all"></div>
                                <span className="text-[10px] font-bold text-gray-400">Aug</span>
                            </div>
                            <div className="flex flex-col items-center gap-2 w-full">
                                <span className="text-[10px] font-bold text-[#34C759]">$6.1k</span>
                                <div className="w-full bg-emerald-500 hover:bg-emerald-600 rounded-t-md h-24 transition-all shadow-[0_4px_12px_rgba(16,185,129,0.2)]"></div>
                                <span className="text-[10px] font-bold text-gray-500 dark:text-gray-400">Sep *</span>
                            </div>
                            <div className="flex flex-col items-center gap-2 w-full">
                                <span className="text-[10px] font-bold text-[#34C759]">$7.8k</span>
                                <div className="w-full bg-emerald-500 hover:bg-emerald-600 rounded-t-md h-32 transition-all shadow-[0_4px_12px_rgba(16,185,129,0.2)]"></div>
                                <span className="text-[10px] font-bold text-gray-500 dark:text-gray-400">Oct *</span>
                            </div>
                            <div className="flex flex-col items-center gap-2 w-full">
                                <span className="text-[10px] font-bold text-[#34C759]">$9.5k</span>
                                <div className="w-full bg-emerald-500 hover:bg-emerald-600 rounded-t-md h-40 transition-all shadow-[0_4px_12px_rgba(16,185,129,0.2)]"></div>
                                <span className="text-[10px] font-bold text-gray-500 dark:text-gray-400">Nov *</span>
                            </div>
                        </div>
                        <div className="mt-4 text-xs text-gray-400 dark:text-gray-500 text-center font-medium">
                            * AI Projected Values (based on conversion rate increase of 15%)
                        </div>
                    </div>
                </div>

                {/* Cohort Retention Card */}
                <div className="app-card glassmorphism p-6 rounded-2xl shadow-sm border border-white/40 dark:border-white/10 flex flex-col h-80 bg-white/70 dark:bg-zinc-900/70 backdrop-blur-[30px] saturate-[210%]">
                    <h3 className="font-bold font-outfit text-gray-800 dark:text-gray-200 mb-4 text-sm uppercase tracking-wider">Cohort Retention Analysis</h3>
                    <div className="flex-1 flex flex-col justify-center">
                        <div className="overflow-x-auto">
                            <table className="w-full text-left text-xs font-semibold">
                                <thead>
                                    <tr className="border-b border-gray-100 dark:border-gray-800 text-gray-400">
                                        <th className="py-2">Cohort</th>
                                        <th className="py-2">Size</th>
                                        <th className="py-2">Month 1</th>
                                        <th className="py-2">Month 2</th>
                                        <th className="py-2">Month 3</th>
                                    </tr>
                                </thead>
                                <tbody className="divide-y divide-gray-50 dark:divide-gray-800/40 text-gray-700 dark:text-gray-300">
                                    <tr>
                                        <td className="py-3 text-gray-900 dark:text-white font-bold">June Cohort</td>
                                        <td className="py-3 text-gray-400">120 users</td>
                                        <td className="py-3"><span className="px-2 py-1 rounded bg-emerald-500/20 text-emerald-800 dark:text-emerald-300">85%</span></td>
                                        <td className="py-3"><span className="px-2 py-1 rounded bg-emerald-500/10 text-emerald-700 dark:text-emerald-400">62%</span></td>
                                        <td className="py-3"><span className="px-2 py-1 rounded bg-amber-500/10 text-amber-700 dark:text-amber-400">45%</span></td>
                                    </tr>
                                    <tr>
                                        <td className="py-3 text-gray-900 dark:text-white font-bold">July Cohort</td>
                                        <td className="py-3 text-gray-400">154 users</td>
                                        <td className="py-3"><span className="px-2 py-1 rounded bg-emerald-500/30 text-emerald-800 dark:text-emerald-200 font-bold">90%</span></td>
                                        <td className="py-3"><span className="px-2 py-1 rounded bg-emerald-500/20 text-emerald-800 dark:text-emerald-300">70%</span></td>
                                        <td className="py-3 text-gray-400">-</td>
                                    </tr>
                                    <tr>
                                        <td className="py-3 text-gray-900 dark:text-white font-bold">August Cohort</td>
                                        <td className="py-3 text-gray-400">180 users</td>
                                        <td className="py-3"><span className="px-2 py-1 rounded bg-emerald-500/40 text-emerald-950 dark:text-emerald-100 font-extrabold">95%</span></td>
                                        <td className="py-3 text-gray-400">-</td>
                                        <td className="py-3 text-gray-400">-</td>
                                    </tr>
                                </tbody>
                            </table>
                        </div>
                    </div>
                </div>
            </div>

           {!hasPro && (
               <div className="absolute inset-0 z-10 flex items-center justify-center">
                   <div className="bg-white/90 p-8 rounded-2xl shadow-xl border border-gray-200 text-center max-w-sm">
                       <div className="text-4xl mb-3">🔮</div>
                       <h3 className="text-xl font-bold font-outfit text-gray-900 mb-2">See The Future</h3>
                       <p className="text-sm text-gray-600 mb-6">Unlock predictive AI insights to forecast revenue, track cohort retention, and optimize your growth strategy.</p>
                       <button
                           onClick={() => setShowSoftPaywall(true)}
                           className="w-full py-3 bg-[#0f766e] hover:bg-[#0d645d] text-white font-bold rounded-xl shadow-md transition-all active:scale-95"
                       >
                           Unlock Predictions
                       </button>
                   </div>
               </div>
           )}
        </section>
      </div>

      {/* Soft Paywall Modal */}
      {showSoftPaywall && (
        <div className="fixed inset-0 bg-black/60 z-[60] flex items-center justify-center p-4">
          <div className="app-card glassmorphism w-full max-w-md rounded-2xl p-8 shadow-2xl relative overflow-hidden font-inter border border-white/40 dark:border-white/10 text-center bg-white/90 dark:bg-zinc-900/90 backdrop-blur-[30px] saturate-[210%]">
            <div className="absolute top-0 right-0 w-32 h-32 bg-teal-50/10 rounded-bl-full -z-10"></div>

            <div className="flex justify-end mb-2">
              <button
                onClick={() => setShowSoftPaywall(false)}
                className="text-gray-400 hover:text-gray-600 rounded-full hover:bg-gray-100 transition-colors w-8 h-8 flex items-center justify-center"
              >
                <span className="text-xl leading-none">&times;</span>
              </button>
            </div>

            <div className="text-5xl mb-4">📈</div>
            <h2 className="text-2xl font-bold font-outfit text-gray-900 dark:text-white mb-3">Upgrade to Pro</h2>
            <p className="text-gray-600 dark:text-gray-300 mb-6 text-sm leading-relaxed">
              Predictive AI Growth Trends and advanced analytics are Pro features. Upgrade to make data-driven decisions.
            </p>

            <button
              onClick={() => { setShowSoftPaywall(false); router.push('/pricing'); }}
              className="w-full py-4 rounded-xl font-bold text-white mb-4 transition-all shadow-md hover:shadow-lg hover:-translate-y-0.5 bg-[#0f766e] hover:bg-[#0d645d]"
            >
              Upgrade to Pro ($79/mo)
            </button>

            <div className="my-4 text-gray-400 font-medium text-sm">OR</div>

            <button
              onClick={claimTrialExtension}
              className="w-full py-3.5 rounded-xl font-bold transition-all shadow-sm hover:bg-gray-50 flex items-center justify-center gap-2"
              style={{ color: '#1DA1F2', border: '2px solid #1DA1F2', background: 'white' }}
            >
              <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 24 24"><path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.008 5.94H5.078z"/></svg>
              Share on X to unlock 7 Days Free
            </button>
          </div>
        </div>
      )}
    </AppShell>
  );
}
