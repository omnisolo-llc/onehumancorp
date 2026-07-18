"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';
import { AppShell } from '../components/AppShell';

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
    <AppShell title="Business Analytics">
      <div className="mx-auto max-w-5xl space-y-8 font-inter">
        <header className="mb-8 p-6 bg-gradient-to-r from-indigo-50/50 to-purple-50/50 rounded-3xl border border-indigo-100/40 shadow-sm backdrop-blur-[30px] saturate-[210%]">
          <h1 className="text-3xl font-extrabold font-outfit text-gray-900 tracking-tight">Business Analytics 📊</h1>
          <p className="mt-2 text-sm text-gray-500">Track your store performance, active visitor numbers, conversion rates, and AI insights.</p>
        </header>

        {trialStatus && (
          <p className="rounded-xl border border-green-100 bg-green-50 px-4 py-3 text-sm font-bold text-green-800 animate-fade-in" role="status">
            ✓ {trialStatus}
          </p>
        )}

        {/* Basic Analytics Section */}
        <section className="space-y-4">
          <h2 className="text-xl font-bold font-outfit text-gray-900">Core Metrics (30 Days)</h2>
          <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
            <div className="app-card p-6 rounded-2xl shadow-md border border-indigo-100/40 flex flex-col justify-between hover:shadow-lg hover:-translate-y-0.5 transition-all duration-300 bg-white/70 backdrop-blur-[30px] saturate-[210%]">
              <div className="text-xs font-bold uppercase tracking-wider text-gray-400">Total Revenue</div>
              <div className="text-3xl font-extrabold font-outfit text-gray-900 mt-2">$4,250.00</div>
              <div className="text-xs font-bold text-green-600 mt-4 flex items-center gap-1">
                 <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2.5} d="M5 10l7-7m0 0l7 7m-7-7v18" /></svg>
                 12% from last month
              </div>
            </div>

            <div className="app-card p-6 rounded-2xl shadow-md border border-indigo-100/40 flex flex-col justify-between hover:shadow-lg hover:-translate-y-0.5 transition-all duration-300 bg-white/70 backdrop-blur-[30px] saturate-[210%]">
              <div className="text-xs font-bold uppercase tracking-wider text-gray-400">Active Customers</div>
              <div className="text-3xl font-extrabold font-outfit text-gray-900 mt-2">142</div>
              <div className="text-xs font-bold text-green-600 mt-4 flex items-center gap-1">
                 <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2.5} d="M5 10l7-7m0 0l7 7m-7-7v18" /></svg>
                 5% from last month
              </div>
            </div>

            <div className="app-card p-6 rounded-2xl shadow-md border border-indigo-100/40 flex flex-col justify-between hover:shadow-lg hover:-translate-y-0.5 transition-all duration-300 bg-white/70 backdrop-blur-[30px] saturate-[210%]">
              <div className="text-xs font-bold uppercase tracking-wider text-gray-400">Conversion Rate</div>
              <div className="text-3xl font-extrabold font-outfit text-gray-900 mt-2">3.8%</div>
              <div className="text-xs font-bold text-gray-400 mt-4 flex items-center gap-1">
                 <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2.5} d="M20 12H4" /></svg>
                 No change
              </div>
            </div>
          </div>
        </section>

        {/* Advanced AI Insights Section */}
        <section className="relative">
           <h2 className="text-xl font-bold font-outfit text-gray-900 mb-4 flex items-center gap-2">
               Advanced AI Customer Insights
               {!hasPro && <span className="bg-amber-100 text-amber-800 text-xs px-2 py-0.5 rounded-full font-bold uppercase tracking-wider border border-amber-200">Pro</span>}
           </h2>

           <div className={`grid grid-cols-1 md:grid-cols-2 gap-6 transition-all duration-500 ${!hasPro ? 'filter blur-sm select-none pointer-events-none opacity-60' : ''}`}>
               <div className="app-card p-6 rounded-2xl shadow-md border border-indigo-100/40 h-64 flex flex-col bg-white/70 backdrop-blur-[30px] saturate-[210%]">
                   <h3 className="font-bold font-outfit text-gray-800 mb-4 text-sm uppercase tracking-wider">Traffic Sources</h3>
                   <div className="flex-1 flex flex-col justify-center items-center gap-3">
                       <div className="w-full flex items-end justify-around h-32 border-b border-gray-100 pb-2">
                           <div className="w-8 bg-indigo-500 rounded-t-lg h-full" title="Direct (45%)"></div>
                           <div className="w-8 bg-purple-500 rounded-t-lg h-3/4" title="Social (30%)"></div>
                           <div className="w-8 bg-pink-500 rounded-t-lg h-1/2" title="Organic (15%)"></div>
                           <div className="w-8 bg-rose-400 rounded-t-lg h-1/4" title="Referral (10%)"></div>
                       </div>
                       <div className="flex gap-4 text-xs font-bold text-gray-400">
                           <span className="flex items-center gap-1.5"><span className="w-2.5 h-2.5 bg-indigo-500 rounded-full"></span>Direct</span>
                           <span className="flex items-center gap-1.5"><span className="w-2.5 h-2.5 bg-purple-500 rounded-full"></span>Social</span>
                           <span className="flex items-center gap-1.5"><span className="w-2.5 h-2.5 bg-pink-500 rounded-full"></span>Organic</span>
                       </div>
                   </div>
               </div>

               <div className="app-card p-6 rounded-2xl shadow-md border border-indigo-100/40 h-64 flex flex-col bg-white/70 backdrop-blur-[30px] saturate-[210%]">
                   <h3 className="font-bold font-outfit text-gray-800 mb-4 text-sm uppercase tracking-wider">AI Buying Intent</h3>
                   <div className="flex-1 flex flex-col justify-center gap-4">
                       <div>
                           <div className="flex justify-between text-xs mb-1.5 font-bold"><span className="text-gray-500">High Intent Visitors</span><span className="text-green-600">28%</span></div>
                           <div className="w-full bg-gray-100 rounded-full h-2.5"><div className="bg-[#34C759] h-2.5 rounded-full w-[28%]"></div></div>
                       </div>
                       <div>
                           <div className="flex justify-between text-xs mb-1.5 font-bold"><span className="text-gray-500">Considering</span><span className="text-amber-600">45%</span></div>
                           <div className="w-full bg-gray-100 rounded-full h-2.5"><div className="bg-amber-500 h-2.5 rounded-full w-[45%]"></div></div>
                       </div>
                       <div>
                           <div className="flex justify-between text-xs mb-1.5 font-bold"><span className="text-gray-500">Just Browsing</span><span className="text-gray-400">27%</span></div>
                           <div className="w-full bg-gray-100 rounded-full h-2.5"><div className="bg-gray-300 h-2.5 rounded-full w-[27%]"></div></div>
                       </div>
                   </div>
               </div>
           </div>

           {!hasPro && (
               <div className="absolute inset-0 z-10 flex items-center justify-center">
                   <div className="bg-white/95 p-8 rounded-3xl shadow-2xl border border-indigo-100/60 text-center max-w-sm backdrop-blur-[30px] saturate-[210%]">
                       <div className="text-5xl mb-4">🔒</div>
                       <h3 className="text-2xl font-bold font-outfit text-gray-900 mb-2">Unlock Advanced Insights</h3>
                       <p className="text-sm text-gray-500 mb-6 leading-relaxed">See exactly where your traffic is coming from and predict buyer behavior with our Pro Plan.</p>
                       <button
                           onClick={() => setShowSoftPaywall(true)}
                           className="w-full py-3 bg-indigo-600 hover:bg-indigo-700 text-white font-bold rounded-xl shadow-md transition-all active:scale-95 text-sm"
                       >
                           Unlock Now
                       </button>
                   </div>
               </div>
           )}
        </section>
      </div>

      {/* Soft Paywall Modal */}
      {showSoftPaywall && (
        <div className="fixed inset-0 bg-black/60 z-[60] flex items-center justify-center p-4 backdrop-blur-[30px] saturate-[210%]">
          <div className="app-card w-full max-w-md rounded-[24px] p-8 shadow-2xl relative overflow-hidden font-inter border border-indigo-100 bg-white/95 backdrop-blur-[30px] saturate-[210%] text-center">
            <div className="absolute top-0 right-0 w-32 h-32 bg-indigo-50/50 rounded-bl-full -z-10"></div>

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
            <p className="text-gray-500 mb-6 text-sm leading-relaxed px-4">
              Advanced AI Customer Insights is a Pro feature. Upgrade to supercharge your business intelligence.
            </p>

            <button
              onClick={() => { setShowSoftPaywall(false); window.location.href = '/pricing'; }}
              className="w-full py-4 rounded-xl font-bold text-white mb-4 transition-all shadow-md hover:shadow-lg hover:-translate-y-0.5 text-sm"
              style={{ background: 'linear-gradient(135deg, #4f46e5 0%, #7c3aed 100%)' }}
            >
              Upgrade to Pro ($79/mo)
            </button>

            <div className="my-4 text-gray-400 font-bold text-xs uppercase tracking-wider">OR</div>

            <button
              onClick={claimTrialExtension}
              className="w-full py-3.5 rounded-xl font-bold transition-all shadow-sm hover:bg-gray-50 flex items-center justify-center gap-2 text-sm"
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
