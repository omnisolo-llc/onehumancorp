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

        {/* Growth Trends */}
        <section className="relative">
           <h2 className="text-xl font-bold font-outfit text-gray-900 dark:text-white mb-4 flex items-center gap-2">
               Predictive AI Growth Trends
               {!hasPro && <span className="bg-[#0f766e] text-white text-xs px-2 py-0.5 rounded-full font-bold uppercase tracking-wider">Pro</span>}
           </h2>

           <div className={`grid grid-cols-1 md:grid-cols-2 gap-6 transition-all duration-500 ${!hasPro ? 'filter blur-md select-none pointer-events-none opacity-50' : ''}`}>
               <div className="app-card glassmorphism p-6 rounded-2xl shadow-sm border border-white/40 dark:border-white/10 h-72 flex flex-col">
                   <h3 className="font-semibold text-gray-800 dark:text-gray-200 mb-4">Revenue Forecast</h3>
                   <div className="flex-1 flex flex-col justify-end gap-2 pb-4 border-b border-gray-100 dark:border-gray-850 relative">
                        {/* Mock area chart */}
                       <div className="w-full h-full absolute inset-0 flex items-end">
                           <svg viewBox="0 0 100 50" className="w-full h-full preserve-3d" preserveAspectRatio="none">
                               <path d="M0,50 L0,30 Q10,20 20,25 T40,15 T60,20 T80,5 Q90,10 100,0 L100,50 Z" fill="rgba(15, 118, 110, 0.2)" stroke="#0f766e" strokeWidth="1"></path>
                               <path d="M80,5 Q90,10 100,0" fill="none" stroke="#0f766e" strokeWidth="2" strokeDasharray="2,2"></path>
                           </svg>
                       </div>
                       <div className="flex justify-between w-full text-xs text-gray-400 mt-2 absolute bottom-0">
                           <span>Oct</span><span>Nov</span><span>Dec</span><span className="text-[#0f766e] font-semibold">Jan (Est)</span>
                       </div>
                   </div>
               </div>

               <div className="app-card glassmorphism p-6 rounded-2xl shadow-sm border border-white/40 dark:border-white/10 h-72 flex flex-col">
                   <h3 className="font-semibold text-gray-800 dark:text-gray-200 mb-4">Customer Cohort Retention</h3>
                   <div className="flex-1 flex flex-col gap-2">
                       <div className="flex justify-between items-center text-xs">
                           <span className="w-16 font-medium text-gray-600 dark:text-gray-300">Month 1</span>
                           <div className="flex-1 flex gap-1 h-6">
                               <div className="bg-[#0f766e] rounded-sm" style={{width: '100%'}}></div>
                           </div>
                           <span className="w-8 text-right text-gray-500">100%</span>
                       </div>
                       <div className="flex justify-between items-center text-xs">
                           <span className="w-16 font-medium text-gray-600 dark:text-gray-300">Month 2</span>
                           <div className="flex-1 flex gap-1 h-6">
                               <div className="bg-[#115e59] rounded-sm" style={{width: '65%'}}></div>
                           </div>
                           <span className="w-8 text-right text-gray-500">65%</span>
                       </div>
                       <div className="flex justify-between items-center text-xs">
                           <span className="w-16 font-medium text-gray-600 dark:text-gray-300">Month 3</span>
                           <div className="flex-1 flex gap-1 h-6">
                               <div className="bg-[#14b8a6] rounded-sm" style={{width: '45%'}}></div>
                           </div>
                           <span className="w-8 text-right text-gray-500">45%</span>
                       </div>
                       <div className="flex justify-between items-center text-xs">
                           <span className="w-16 font-medium text-gray-600 dark:text-gray-300">Month 4</span>
                           <div className="flex-1 flex gap-1 h-6">
                               <div className="bg-[#2dd4bf] rounded-sm" style={{width: '35%'}}></div>
                           </div>
                           <span className="w-8 text-right text-gray-500">35%</span>
                       </div>
                       <div className="flex justify-between items-center text-xs">
                           <span className="w-16 font-medium text-gray-600 dark:text-gray-300">Month 5</span>
                           <div className="flex-1 flex gap-1 h-6">
                               <div className="bg-[#56e39f] rounded-sm" style={{width: '28%'}}></div>
                           </div>
                           <span className="w-8 text-right text-gray-500">28%</span>
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
