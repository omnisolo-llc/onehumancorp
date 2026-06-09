"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';

export default function AnalyticsPage() {
  const router = useRouter();
  const [hasPro, setHasPro] = useState(false);
  const [showSoftPaywall, setShowSoftPaywall] = useState(false);
  const [tenant, setTenant] = useState('my-store');
  const [trialStatus, setTrialStatus] = useState('');

  useEffect(() => {
    if (typeof localStorage !== 'undefined') {
      setHasPro(localStorage.getItem('has_pro') === 'true');
      const savedTenant = localStorage.getItem('tenant_id') || localStorage.getItem('tenant') || 'my-store';
      setTenant(savedTenant);
    }
  }, []);

  const claimTrialExtension = () => {
    const referralUrl = `${window.location.origin}/onboarding?ref=${tenant}`;
    window.open(`https://twitter.com/intent/tweet?text=${encodeURIComponent('I just unlocked powerful AI tools for my business on One Human Corp! Start your own business today: ' + referralUrl)}`, '_blank');
    if (typeof localStorage !== 'undefined') {
        localStorage.setItem('has_pro', 'true');
    }
    setHasPro(true);
    setShowSoftPaywall(false);
    setTrialStatus('Your 7-day Pro trial has been activated.');
  };

  return (
    <div className="flex flex-col min-h-screen font-inter" style={{ backgroundColor: '#F5F5F7' }}>
      {/* Header */}
      <header className="px-6 py-4 flex items-center justify-between border-b sticky top-0 z-50 glassmorphism/65 backdrop-blur-md border-white/40 shadow-sm">
         <h1 className="text-2xl font-bold font-outfit" style={{ color: '#1D1D1F', letterSpacing: '-0.02em' }}>Business Analytics 📊</h1>
         <div className="flex items-center gap-3">
             <button onClick={() => router.push('/dashboard')} className="px-4 py-2 bg-gray-200 rounded-md text-sm font-medium hover:bg-gray-300 transition-colors">
               Back to Dashboard
             </button>
         </div>
      </header>

      <main className="p-6 md:p-8 flex-1 max-w-5xl mx-auto w-full flex flex-col gap-8">
        {trialStatus && <p className="rounded-lg border border-green-100 bg-green-50 px-4 py-3 text-sm font-semibold text-green-800" role="status">{trialStatus}</p>}

        {/* Basic Analytics Section */}
        <section>
          <h2 className="text-xl font-bold font-outfit text-gray-900 mb-4">Core Metrics (30 Days)</h2>
          <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
            <div className="app-card p-6 rounded-2xl shadow-sm border border-gray-100 flex flex-col justify-between">
              <div className="text-sm font-medium text-gray-500 mb-1">Total Revenue</div>
              <div className="text-3xl font-bold font-outfit text-gray-900">$4,250.00</div>
              <div className="text-xs font-semibold text-green-500 mt-2 flex items-center gap-1">
                 <svg className="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 10l7-7m0 0l7 7m-7-7v18" /></svg>
                 12% from last month
              </div>
            </div>

            <div className="app-card p-6 rounded-2xl shadow-sm border border-gray-100 flex flex-col justify-between">
              <div className="text-sm font-medium text-gray-500 mb-1">Active Customers</div>
              <div className="text-3xl font-bold font-outfit text-gray-900">142</div>
              <div className="text-xs font-semibold text-green-500 mt-2 flex items-center gap-1">
                 <svg className="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 10l7-7m0 0l7 7m-7-7v18" /></svg>
                 5% from last month
              </div>
            </div>

            <div className="app-card p-6 rounded-2xl shadow-sm border border-gray-100 flex flex-col justify-between">
              <div className="text-sm font-medium text-gray-500 mb-1">Conversion Rate</div>
              <div className="text-3xl font-bold font-outfit text-gray-900">3.8%</div>
              <div className="text-xs font-semibold text-gray-400 mt-2 flex items-center gap-1">
                 <svg className="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M20 12H4" /></svg>
                 No change
              </div>
            </div>
          </div>
        </section>

        {/* Advanced AI Insights Section */}
        <section className="relative">
           <h2 className="text-xl font-bold font-outfit text-gray-900 mb-4 flex items-center gap-2">
               Advanced AI Customer Insights
               {!hasPro && <span className="bg-yellow-100 text-yellow-800 text-xs px-2 py-0.5 rounded-full font-bold uppercase tracking-wider">Pro</span>}
           </h2>

           <div className={`grid grid-cols-1 md:grid-cols-2 gap-6 transition-all duration-500 ${!hasPro ? 'filter blur-sm select-none pointer-events-none opacity-60' : ''}`}>
               <div className="app-card p-6 rounded-2xl shadow-sm border border-gray-100 h-64 flex flex-col">
                   <h3 className="font-semibold text-gray-800 mb-4">Traffic Sources</h3>
                   <div className="flex-1 flex flex-col justify-center items-center gap-3">
                       {/* Mock Chart representation */}
                       <div className="w-full flex items-end justify-around h-32 border-b border-gray-200 pb-2">
                           <div className="w-8 bg-blue-500 rounded-t-sm h-full" title="Direct (45%)"></div>
                           <div className="w-8 bg-indigo-500 rounded-t-sm h-3/4" title="Social (30%)"></div>
                           <div className="w-8 bg-purple-500 rounded-t-sm h-1/2" title="Organic (15%)"></div>
                           <div className="w-8 bg-pink-500 rounded-t-sm h-1/4" title="Referral (10%)"></div>
                       </div>
                       <div className="flex gap-4 text-xs font-medium text-gray-500">
                           <span className="flex items-center gap-1"><span className="w-2 h-2 bg-blue-500 rounded-full"></span>Direct</span>
                           <span className="flex items-center gap-1"><span className="w-2 h-2 bg-indigo-500 rounded-full"></span>Social</span>
                           <span className="flex items-center gap-1"><span className="w-2 h-2 bg-purple-500 rounded-full"></span>Organic</span>
                       </div>
                   </div>
               </div>

               <div className="app-card p-6 rounded-2xl shadow-sm border border-gray-100 h-64 flex flex-col">
                   <h3 className="font-semibold text-gray-800 mb-4">AI Buying Intent</h3>
                   <div className="flex-1 flex flex-col justify-center gap-4">
                       <div>
                           <div className="flex justify-between text-xs mb-1 font-medium"><span className="text-gray-700">High Intent Visitors</span><span className="text-green-600">28%</span></div>
                           <div className="w-full bg-gray-200 rounded-full h-2"><div className="bg-green-500 h-2 rounded-full w-[28%]"></div></div>
                       </div>
                       <div>
                           <div className="flex justify-between text-xs mb-1 font-medium"><span className="text-gray-700">Considering</span><span className="text-yellow-600">45%</span></div>
                           <div className="w-full bg-gray-200 rounded-full h-2"><div className="bg-yellow-500 h-2 rounded-full w-[45%]"></div></div>
                       </div>
                       <div>
                           <div className="flex justify-between text-xs mb-1 font-medium"><span className="text-gray-700">Just Browsing</span><span className="text-gray-500">27%</span></div>
                           <div className="w-full bg-gray-200 rounded-full h-2"><div className="bg-gray-400 h-2 rounded-full w-[27%]"></div></div>
                       </div>
                   </div>
               </div>
           </div>

           {!hasPro && (
               <div className="absolute inset-0 z-10 flex items-center justify-center">
                   <div className="glassmorphism/90 p-8 rounded-2xl shadow-xl border border-gray-200 text-center max-w-sm">
                       <div className="text-4xl mb-3">🔒</div>
                       <h3 className="text-xl font-bold font-outfit text-gray-900 mb-2">Unlock Advanced Insights</h3>
                       <p className="text-sm text-gray-600 mb-6">See exactly where your traffic is coming from and predict buyer behavior with our Pro Plan.</p>
                       <button
                           onClick={() => setShowSoftPaywall(true)}
                           className="w-full py-3 bg-indigo-600 hover:bg-indigo-700 text-white font-bold rounded-xl shadow-md transition-all active:scale-95"
                       >
                           Unlock Now
                       </button>
                   </div>
               </div>
           )}
        </section>
      </main>

      {/* Soft Paywall Modal */}
      {showSoftPaywall && (
        <div className="fixed inset-0 bg-black/60 z-[60] flex items-center justify-center p-4">
          <div className="app-card w-full max-w-md rounded-2xl p-8 shadow-2xl relative overflow-hidden font-inter border border-indigo-100 text-center">
            <div className="absolute top-0 right-0 w-32 h-32 bg-indigo-50 rounded-bl-full -z-10"></div>

            <div className="flex justify-end mb-2">
              <button
                onClick={() => setShowSoftPaywall(false)}
                className="text-gray-400 hover:text-gray-600 rounded-full hover:bg-gray-100 transition-colors w-8 h-8 flex items-center justify-center"
              >
                <span className="text-xl leading-none">&times;</span>
              </button>
            </div>

            <div className="text-5xl mb-4">🚀</div>
            <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-3">Upgrade to Pro</h2>
            <p className="text-gray-600 mb-6 text-sm leading-relaxed">
              Advanced AI Customer Insights is a Pro feature. Upgrade to supercharge your business intelligence.
            </p>

            <button
              onClick={() => { setShowSoftPaywall(false); window.location.href = '/pricing'; }}
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
