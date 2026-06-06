'use client';

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';

export default function BusinessAnalytics() {
  const router = useRouter();
  const [hasPro, setHasPro] = useState(false);
  const [showSoftPaywall, setShowSoftPaywall] = useState(false);

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
    alert('🎉 7-day Pro Trial activated! You now have access to advanced analytics.');
  };

  return (
    <div className="min-h-screen bg-gray-50 flex flex-col font-inter">
      <title>Business Analytics | OHC</title>

      <header className="bg-white border-b border-gray-200 px-6 py-4 flex items-center justify-between sticky top-0 z-50">
         <h1 className="text-2xl font-bold font-outfit" style={{ color: '#1D1D1F', letterSpacing: '-0.02em' }}>Business Analytics 📊</h1>
         <div className="flex items-center gap-3">
             <button onClick={() => router.push('/dashboard')} className="px-4 py-2 bg-gray-200 rounded-md text-sm font-medium hover:bg-gray-300 transition-colors">
               Back to Dashboard
             </button>
         </div>
      </header>

      <main className="p-6 md:p-8 flex-1 max-w-6xl mx-auto w-full flex flex-col gap-8">
        {/* Core Metrics Section */}
        <section>
          <h2 className="text-xl font-bold font-outfit text-gray-900 mb-4">Core Performance (Last 30 Days)</h2>
          <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
            <div className="bg-white p-5 rounded-2xl shadow-sm border border-gray-100 flex flex-col justify-between">
              <div className="text-sm font-medium text-gray-500 mb-1">Total Revenue</div>
              <div className="text-2xl font-bold font-outfit text-gray-900">$8,450.00</div>
              <div className="text-xs font-semibold text-green-500 mt-2 flex items-center gap-1">
                 <svg className="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 10l7-7m0 0l7 7m-7-7v18" /></svg>
                 15% from last month
              </div>
            </div>

            <div className="bg-white p-5 rounded-2xl shadow-sm border border-gray-100 flex flex-col justify-between">
              <div className="text-sm font-medium text-gray-500 mb-1">Average Order Value</div>
              <div className="text-2xl font-bold font-outfit text-gray-900">$45.50</div>
              <div className="text-xs font-semibold text-green-500 mt-2 flex items-center gap-1">
                 <svg className="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 10l7-7m0 0l7 7m-7-7v18" /></svg>
                 2% from last month
              </div>
            </div>

            <div className="bg-white p-5 rounded-2xl shadow-sm border border-gray-100 flex flex-col justify-between">
              <div className="text-sm font-medium text-gray-500 mb-1">Orders</div>
              <div className="text-2xl font-bold font-outfit text-gray-900">185</div>
              <div className="text-xs font-semibold text-green-500 mt-2 flex items-center gap-1">
                 <svg className="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 10l7-7m0 0l7 7m-7-7v18" /></svg>
                 10% from last month
              </div>
            </div>

            <div className="bg-white p-5 rounded-2xl shadow-sm border border-gray-100 flex flex-col justify-between">
              <div className="text-sm font-medium text-gray-500 mb-1">Conversion Rate</div>
              <div className="text-2xl font-bold font-outfit text-gray-900">4.2%</div>
              <div className="text-xs font-semibold text-red-500 mt-2 flex items-center gap-1">
                 <svg className="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 14l-7 7m0 0l-7-7m7 7V3" /></svg>
                 -1.5% from last month
              </div>
            </div>
          </div>
        </section>

        {/* Growth Trends */}
        <section className="relative">
           <h2 className="text-xl font-bold font-outfit text-gray-900 mb-4 flex items-center gap-2">
               Predictive AI Growth Trends
               {!hasPro && <span className="bg-yellow-100 text-yellow-800 text-xs px-2 py-0.5 rounded-full font-bold uppercase tracking-wider">Pro</span>}
           </h2>

           <div className={`grid grid-cols-1 md:grid-cols-2 gap-6 transition-all duration-500 ${!hasPro ? 'filter blur-md select-none pointer-events-none opacity-50' : ''}`}>
               <div className="bg-white p-6 rounded-2xl shadow-sm border border-gray-100 h-72 flex flex-col">
                   <h3 className="font-semibold text-gray-800 mb-4">Revenue Forecast</h3>
                   <div className="flex-1 flex flex-col items-center justify-center text-center p-6 bg-gray-50 rounded-xl border border-dashed border-gray-200">
                     <span className="text-3xl mb-2">📉</span>
                     <p className="text-gray-500 text-sm">Not enough historical data to generate an AI forecast.</p>
                   </div>
               </div>

               <div className="bg-white p-6 rounded-2xl shadow-sm border border-gray-100 h-72 flex flex-col">
                   <h3 className="font-semibold text-gray-800 mb-4">Customer Cohort Retention</h3>
                   <div className="flex-1 flex flex-col items-center justify-center text-center p-6 bg-gray-50 rounded-xl border border-dashed border-gray-200">
                     <span className="text-3xl mb-2">👥</span>
                     <p className="text-gray-500 text-sm">Customer retention data requires at least 3 months of history.</p>
                   </div>
               </div>
           </div>

           {!hasPro && (
               <div className="absolute inset-0 z-10 flex items-center justify-center">
                   <div className="bg-white/90 backdrop-blur-md p-8 rounded-2xl shadow-xl border border-gray-200 text-center max-w-sm">
                       <div className="text-4xl mb-3">🔮</div>
                       <h3 className="text-xl font-bold font-outfit text-gray-900 mb-2">See The Future</h3>
                       <p className="text-sm text-gray-600 mb-6">Unlock predictive AI insights to forecast revenue, track cohort retention, and optimize your growth strategy.</p>
                       <button
                           onClick={() => setShowSoftPaywall(true)}
                           className="w-full py-3 bg-indigo-600 hover:bg-indigo-700 text-white font-bold rounded-xl shadow-md transition-all active:scale-95"
                       >
                           Unlock Predictions
                       </button>
                   </div>
               </div>
           )}
        </section>
      </main>

      {/* Soft Paywall Modal */}
      {showSoftPaywall && (
        <div className="fixed inset-0 bg-black/60 z-[60] flex items-center justify-center p-4">
          <div className="bg-white w-full max-w-md rounded-2xl p-8 shadow-2xl relative overflow-hidden font-inter border border-indigo-100 text-center">
            <div className="absolute top-0 right-0 w-32 h-32 bg-indigo-50 rounded-bl-full -z-10"></div>

            <div className="flex justify-end mb-2">
              <button
                onClick={() => setShowSoftPaywall(false)}
                className="text-gray-400 hover:text-gray-600 rounded-full hover:bg-gray-100 transition-colors w-8 h-8 flex items-center justify-center"
              >
                <span className="text-xl leading-none">&times;</span>
              </button>
            </div>

            <div className="text-5xl mb-4">📈</div>
            <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-3">Upgrade to Pro</h2>
            <p className="text-gray-600 mb-6 text-sm leading-relaxed">
              Predictive AI Growth Trends and advanced analytics are Pro features. Upgrade to make data-driven decisions.
            </p>

            <button
              onClick={() => { setShowSoftPaywall(false); router.push('/pricing'); }}
              className="w-full py-4 rounded-xl font-bold text-white mb-4 transition-all shadow-md hover:shadow-lg hover:-translate-y-0.5"
              style={{ background: 'linear-gradient(135deg, #4f46e5 0%, #7c3aed 100%)' }}
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

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}
