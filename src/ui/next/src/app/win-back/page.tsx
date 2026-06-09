"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';

export default function WinBackCampaignPage() {
  const router = useRouter();
  const [productName, setProductName] = useState('');
  const [discountOffer, setDiscountOffer] = useState('');
  const [generatedDraft, setGeneratedDraft] = useState('');
  const [isGenerating, setIsGenerating] = useState(false);
  const [hasPro, setHasPro] = useState(false);
  const [showSoftPaywall, setShowSoftPaywall] = useState(false);
  const [isSent, setIsSent] = useState(false);
  const [trialStatus, setTrialStatus] = useState('');

  useEffect(() => {
    if (typeof localStorage !== 'undefined') {
      setHasPro(localStorage.getItem('has_pro') === 'true');
    }
  }, []);

  const generateDraft = () => {
    setIsGenerating(true);
    setTimeout(() => {
      const storeName = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant') || 'Our Store' : 'Our Store';
      const storeSlug = storeName.toLowerCase().replace(/[^a-z0-9]/g, '') || 'my-store';
      const draft = `Subject: We miss you! Here's ${discountOffer}% off your next order 🎁\n\nHi there,\n\nIt's been a while since we last saw you at ${storeName}. We noticed you loved our products, and we wanted to welcome you back with something special.\n\nUse code WINBACK${discountOffer} to get ${discountOffer}% off your next purchase${productName ? ` of our ${productName}` : ''}!\n\nShop now: /bio/${storeSlug}\n\nBest,\nThe ${storeName} Team\n\n⚡ Powered by OHC`;
      setGeneratedDraft(draft);
      setIsGenerating(false);
      setIsSent(false);
    }, 1500);
  };

  const handleGenerate = () => {
    if (!hasPro) {
      setShowSoftPaywall(true);
      return;
    }

    generateDraft();
  };

  const claimTrialExtension = () => {
    const tenant = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant_id') || 'DEFAULT' : 'DEFAULT';
    const referralUrl = `${window.location.origin}/onboarding?ref=${tenant}`;
    window.open(`https://twitter.com/intent/tweet?text=${encodeURIComponent('I just unlocked powerful AI win-back campaigns for my business on One Human Corp! Start your own business today: ' + referralUrl)}`, '_blank');
    if (typeof localStorage !== 'undefined') {
        localStorage.setItem('has_pro', 'true');
    }
    setHasPro(true);
    setShowSoftPaywall(false);
    setTrialStatus('Your 7-day Pro trial has been activated.');
    generateDraft();
  };

  const handleSend = () => {
    setIsSent(true);
  };

  return (
    <div className="flex flex-col min-h-screen font-inter" style={{ backgroundColor: '#F5F5F7' }}>
      {/* Header */}
      <header className="px-6 py-4 flex items-center justify-between border-b" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', borderBottom: '1px solid rgba(255, 255, 255, 0.4)', position: 'sticky', top: 0, zIndex: 50 }}>
         <h1 className="text-2xl font-bold font-outfit" style={{ color: '#1D1D1F', letterSpacing: '-0.02em' }}>Customer Win-back Campaign 💌</h1>
         <div className="flex items-center gap-3">
             <button onClick={() => router.push('/dashboard')} className="px-4 py-2 bg-gray-200 rounded-md text-sm font-medium hover:bg-gray-300 transition-colors">
               Back to Dashboard
             </button>
         </div>
      </header>

      <main className="p-6 md:p-8 flex-1 max-w-4xl mx-auto w-full flex flex-col gap-8">
        {trialStatus && <p className="rounded-lg border border-green-100 bg-green-50 px-4 py-3 text-sm font-semibold text-green-800" role="status">{trialStatus}</p>}
        <div className="bg-gradient-to-r from-purple-50 to-pink-50 border border-purple-100 rounded-2xl p-6 shadow-sm">
           <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2">Re-engage Inactive Customers</h2>
           <p className="text-gray-600 text-sm">
             Generate highly-converting, personalized win-back emails using AI. It costs 5x less to retain a customer than to acquire a new one.
           </p>
        </div>

        <div className="flex flex-col md:flex-row gap-8">
          {/* Campaign Settings */}
          <section className="w-full md:w-1/2 p-6 shadow-md" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)', borderRadius: '16px' }}>
            <div className="flex items-center gap-4 mb-4">
              <h3 className="text-xl font-semibold font-outfit m-0" style={{ color: '#1D1D1F' }}>Campaign Details</h3>
              <div className="flex items-center gap-2 px-3 py-1 bg-yellow-50 rounded-full border border-yellow-100">
                  <span className="text-xs font-medium text-yellow-600">Pro Feature</span>
              </div>
            </div>
            <div className="flex flex-col gap-4">
              <div>
                <label htmlFor="product-name" className="block text-sm font-medium text-gray-700 mb-1">Product to Feature (Optional)</label>
                <input
                  id="product-name"
                  type="text"
                  value={productName}
                  onChange={(e) => setProductName(e.target.value)}
                  placeholder="e.g. Signature Coffee Blend"
                  className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-purple-500"
                />
              </div>
              <div>
                <label htmlFor="discount-offer" className="block text-sm font-medium text-gray-700 mb-1">Discount Offer (%)</label>
                <input
                  id="discount-offer"
                  type="number"
                  value={discountOffer}
                  onChange={(e) => setDiscountOffer(e.target.value)}
                  placeholder="e.g. 15"
                  className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-purple-500"
                />
              </div>
              <button
                onClick={handleGenerate}
                disabled={!discountOffer || isGenerating}
                className={`w-full py-3 mt-4 text-white font-semibold rounded-xl shadow-lg transition-all flex items-center justify-center gap-2 ${(!discountOffer || isGenerating) ? 'bg-purple-400 cursor-not-allowed' : 'bg-purple-600 hover:bg-purple-700 hover:shadow-xl hover:-translate-y-0.5 active:translate-y-0'}`}
              >
                {isGenerating ? 'Drafting with AI...' : 'Generate AI Campaign'}
              </button>
            </div>
          </section>

          {/* AI Draft Preview */}
          <section className="w-full md:w-1/2 p-6 shadow-md flex flex-col" style={{ background: '#ffffff', border: '1px solid rgba(0, 0, 0, 0.05)', borderRadius: '16px' }}>
            <h3 className="text-xl font-semibold font-outfit mb-4 flex items-center gap-2" style={{ color: '#1D1D1F' }}>
              <span className="text-yellow-500">✨</span> AI Generated Draft
            </h3>

            {generatedDraft ? (
              <div className="flex-1 flex flex-col">
                <div className="flex-1 bg-gray-50 border border-gray-100 rounded-xl p-4 mb-4">
                  <pre className="whitespace-pre-wrap text-sm text-gray-700 font-inter font-medium" style={{ fontFamily: 'inherit' }}>
                    {generatedDraft}
                  </pre>
                </div>

                {isSent ? (
                  <div className="w-full py-3 bg-green-50 text-green-700 font-bold rounded-xl text-center border border-green-200">
                    ✅ Campaign sent to 34 inactive customers!
                  </div>
                ) : (
                  <button
                    onClick={handleSend}
                    className="w-full py-3 bg-gray-900 hover:bg-black text-white font-bold rounded-xl shadow-md transition-all flex items-center justify-center gap-2 relative overflow-hidden group"
                  >
                    Send to 34 Inactive Customers
                  </button>
                )}
              </div>
            ) : (
              <div className="flex-1 flex flex-col items-center justify-center text-gray-400 border-2 border-dashed border-gray-200 rounded-xl p-6 text-center">
                <svg className="w-12 h-12 mb-3 text-gray-300" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.5} d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" /></svg>
                <p className="text-sm font-medium">Configure your campaign to generate a high-converting win-back draft.</p>
              </div>
            )}
          </section>
        </div>
      </main>

      {/* Soft Paywall Modal */}
      {showSoftPaywall && (
        <div className="fixed inset-0 bg-black/60 z-[60] flex items-center justify-center p-4">
          <div className="app-card w-full max-w-md rounded-2xl p-8 shadow-2xl relative overflow-hidden font-inter border border-purple-100 text-center">
            <div className="absolute top-0 right-0 w-32 h-32 bg-purple-50 rounded-bl-full -z-10"></div>

            <div className="flex justify-end mb-2">
              <button
                onClick={() => setShowSoftPaywall(false)}
                className="text-gray-400 hover:text-gray-600 rounded-full hover:bg-gray-100 transition-colors w-8 h-8 flex items-center justify-center"
              >
                <span className="text-xl leading-none">&times;</span>
              </button>
            </div>

            <div className="w-16 h-16 bg-purple-100 rounded-2xl flex items-center justify-center text-3xl shadow-inner text-purple-600 mx-auto mb-6">
              ✨
            </div>
            <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-3">Upgrade to Pro</h2>
            <p className="text-gray-600 mb-6 text-sm leading-relaxed">
              Customer Win-back Campaigns are a Pro feature. Upgrade to our Pro plan to re-engage past customers and boost sales on autopilot.
            </p>

            <button
              onClick={() => { setShowSoftPaywall(false); window.location.href = '/pricing'; }}
              className="w-full py-4 rounded-xl font-bold text-white mb-4 transition-all shadow-md hover:shadow-lg hover:opacity-90"
              style={{ background: 'linear-gradient(135deg, #a855f7 0%, #d946ef 100%)' }}
            >
              Upgrade to Pro
            </button>

            <div className="my-4 text-gray-400 font-medium text-sm">OR</div>

            <button
              onClick={claimTrialExtension}
              className="w-full py-3.5 rounded-xl font-bold transition-all shadow-sm hover:bg-gray-50 flex items-center justify-center gap-2 border-2 border-[#1DA1F2] text-[#1DA1F2] glassmorphism"
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
