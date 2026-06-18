'use client';

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';
import { AppShell } from '../components/AppShell';

export default function BusinessAnalytics() {
  const router = useRouter();
  const [hasPro, setHasPro] = useState(false);
  const [showSoftPaywall, setShowSoftPaywall] = useState(false);
  const [trialStatus, setTrialStatus] = useState('');

  useEffect(() => {
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
              <div className="text-2xl font-bold font-outfit text-gray-900 dark:text-white">$8,450.00</div>
              <div className="text-xs font-semibold text-green-500 mt-2 flex items-center gap-1">
                 <svg className="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 10l7-7m0 0l7 7m-7-7v18" /></svg>
                 15% from last month
              </div>
            </div>

            <div className="app-card glassmorphism p-5 rounded-2xl shadow-sm border border-white/40 dark:border-white/10 flex flex-col justify-between">
              <div className="text-sm font-medium text-gray-500 mb-1">Average Order Value</div>
              <div className="text-2xl font-bold font-outfit text-gray-900 dark:text-white">$45.50</div>
              <div className="text-xs font-semibold text-green-500 mt-2 flex items-center gap-1">
                 <svg className="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 10l7-7m0 0l7 7m-7-7v18" /></svg>
                 2% from last month
              </div>
            </div>

            <div className="app-card glassmorphism p-5 rounded-2xl shadow-sm border border-white/40 dark:border-white/10 flex flex-col justify-between">
              <div className="text-sm font-medium text-gray-500 mb-1">Orders</div>
              <div className="text-2xl font-bold font-outfit text-gray-900 dark:text-white">185</div>
              <div className="text-xs font-semibold text-green-500 mt-2 flex items-center gap-1">
                 <svg className="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 10l7-7m0 0l7 7m-7-7v18" /></svg>
                 10% from last month
              </div>
            </div>

            <div className="app-card glassmorphism p-5 rounded-2xl shadow-sm border border-white/40 dark:border-white/10 flex flex-col justify-between">
              <div className="text-sm font-medium text-gray-500 mb-1">Conversion Rate</div>
              <div className="text-2xl font-bold font-outfit text-gray-900 dark:text-white">4.2%</div>
              <div className="text-xs font-semibold text-red-500 mt-2 flex items-center gap-1">
                 <svg className="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 14l-7 7m0 0l-7-7m7 7V3" /></svg>
                 -1.5% from last month
              </div>
            </div>
          </div>
        </section>


      </div>

      {/* Soft Paywall Modal */}
      {showSoftPaywall && (
        <div className="fixed inset-0 bg-black/60 z-[60] flex items-center justify-center p-4">
          <div className="app-card glassmorphism w-full max-w-md rounded-2xl p-8 shadow-2xl relative overflow-hidden font-inter border border-white/40 dark:border-white/10 text-center bg-white/90 dark:bg-zinc-900/90 backdrop-blur-md">
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
