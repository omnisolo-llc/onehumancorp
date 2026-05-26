"use client";

import React, { useState } from 'react';
import { useRouter } from 'next/navigation';

export default function BusinessAnalyticsPage() {
  const router = useRouter();
  const [showSoftPaywall, setShowSoftPaywall] = useState(false);
  const [hasPro, setHasPro] = useState(typeof localStorage !== 'undefined' ? localStorage.getItem('has_pro') === 'true' : false);

  const handleActionClick = () => {
    if (!hasPro) {
      setShowSoftPaywall(true);
    } else {
      router.push('/review-campaigns');
    }
  };

  const claimTrialExtension = () => {
    const tenant = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant_id') || 'DEFAULT' : 'DEFAULT';
    window.open(`https://twitter.com/intent/tweet?text=${encodeURIComponent('I just unlocked powerful AI tools for my business on One Human Corp! Start your own business today: ohc://join?ref=' + tenant)}`, '_blank');
    if (typeof localStorage !== 'undefined') {
        localStorage.setItem('has_pro', 'true');
    }
    setHasPro(true);
    setShowSoftPaywall(false);
    setTimeout(() => {
      alert('Thank you for sharing! Your 7-day Pro trial has been activated.');
      router.push('/review-campaigns');
    }, 500);
  };

  return (
    <div className="flex flex-col min-h-screen font-inter" style={{ backgroundColor: '#F5F5F7' }}>
      {/* Header */}
      <header className="px-6 py-4 flex items-center justify-between border-b" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', borderBottom: '1px solid rgba(255, 255, 255, 0.4)', position: 'sticky', top: 0, zIndex: 50 }}>
         <h1 className="text-2xl font-bold font-outfit" style={{ color: '#1D1D1F', letterSpacing: '-0.02em' }}>Business Analytics 📊</h1>
         <div className="flex items-center gap-3">
             <button onClick={() => router.push('/dashboard')} className="px-4 py-2 bg-gray-200 rounded-md text-sm font-medium hover:bg-gray-300 transition-colors">
               Back to Dashboard
             </button>
             <div className="w-8 h-8 rounded-full bg-gray-200 flex items-center justify-center text-sm font-bold text-gray-600">
                 AC
             </div>
         </div>
      </header>
      <main className="p-6 md:p-8 flex-1 max-w-6xl mx-auto w-full flex flex-col gap-8">
        <section className="mb-4">
          <h2 className="text-xl font-semibold font-outfit mb-4" style={{ color: '#1D1D1F' }}>Store Performance</h2>
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
            <div className="p-6 shadow-sm flex flex-col justify-center" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)', borderRadius: '16px' }}>
              <p className="text-sm text-gray-500 font-medium mb-1">Total Revenue</p>
              <p className="text-2xl font-bold font-outfit text-gray-900">$12,450</p>
              <p className="text-xs text-green-600 font-medium mt-2">↑ 14% vs last month</p>
            </div>
            <div className="p-6 shadow-sm flex flex-col justify-center" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)', borderRadius: '16px' }}>
              <p className="text-sm text-gray-500 font-medium mb-1">Store Visitors</p>
              <p className="text-2xl font-bold font-outfit text-gray-900">3,240</p>
              <p className="text-xs text-green-600 font-medium mt-2">↑ 8% vs last month</p>
            </div>
            <div className="p-6 shadow-sm flex flex-col justify-center" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)', borderRadius: '16px' }}>
              <p className="text-sm text-gray-500 font-medium mb-1">Conversion Rate</p>
              <p className="text-2xl font-bold font-outfit text-gray-900">3.8%</p>
              <p className="text-xs text-red-500 font-medium mt-2">↓ 1.2% vs last month</p>
            </div>
            <div className="p-6 shadow-sm flex flex-col justify-center" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)', borderRadius: '16px' }}>
              <p className="text-sm text-gray-500 font-medium mb-1">Referral Signups</p>
              <p className="text-2xl font-bold font-outfit text-gray-900">42</p>
              <p className="text-xs text-green-600 font-medium mt-2">↑ 24% vs last month</p>
            </div>
          </div>
        </section>

        <section className="mb-4">
          <h2 className="text-xl font-semibold font-outfit mb-4" style={{ color: '#1D1D1F' }}>AI Growth Insights</h2>
          <div className="p-6 shadow-sm bg-white" style={{ border: '1px solid rgba(0,0,0,0.05)', borderRadius: '16px' }}>
             <h3 className="text-lg font-bold font-outfit mb-2 flex items-center gap-2"><span className="text-yellow-500">✨</span> Action Recommended: Re-engage Recent Buyers</h3>
             <p className="text-gray-600 text-sm mb-4">
               Our AI detected that 45 customers bought from you in the last 14 days but haven't left a review. Sending an automated review request campaign can boost your conversion rate by up to 12%.
             </p>
             <button
                onClick={handleActionClick}
                className="px-6 py-3 bg-indigo-600 hover:bg-indigo-700 text-white font-bold rounded-xl shadow-md transition-all flex items-center gap-2"
             >
                <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 10V3L4 14h7v7l9-11h-7z" /></svg>
                Draft AI Campaign
             </button>
          </div>
        </section>

      </main>

      {/* Soft Paywall Modal */}
      {showSoftPaywall && (
        <div className="fixed inset-0 bg-black/60 z-[60] flex items-center justify-center p-4">
          <div className="bg-white w-full max-w-md rounded-3xl p-8 shadow-2xl relative overflow-hidden font-inter text-center">
            <div className="absolute top-0 inset-x-0 h-32 bg-gradient-to-b from-blue-50 to-white -z-10"></div>
            <button onClick={() => setShowSoftPaywall(false)} className="absolute top-4 right-4 p-2 text-gray-400 hover:text-gray-600 rounded-full hover:bg-gray-100 transition-colors">
              <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" /></svg>
            </button>
            <div className="text-5xl mb-4">✨</div>
            <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-3">Unlock AI Power</h2>
            <p className="text-gray-600 mb-6 text-sm leading-relaxed">
              AI Growth Insights and Automated Campaigns are Pro features. Upgrade to our Pro plan to boost your sales on autopilot.
            </p>

            <button
              onClick={() => { setShowSoftPaywall(false); window.location.href = '/pricing'; }}
              className="w-full py-4 rounded-xl font-bold text-white mb-4 transition-all shadow-md hover:shadow-lg hover:opacity-90"
              style={{ background: 'linear-gradient(135deg, #0066ff 0%, #3b82f6 100%)' }}
            >
              Upgrade to Pro
            </button>

            <div className="my-4 text-gray-400 font-medium text-sm">OR</div>

            <button
              onClick={claimTrialExtension}
              className="w-full py-3.5 rounded-xl font-bold transition-all shadow-sm hover:bg-gray-50 flex items-center justify-center gap-2"
              style={{ color: '#1DA1F2', border: '2px solid #1DA1F2', background: 'white' }}
            >
              <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 24 24"><path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.008 5.94H5.078z"/></svg>
              Share on X to get 7 Days Free
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
