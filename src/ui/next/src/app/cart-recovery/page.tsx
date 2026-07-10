"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';
import { PoweredByOHC } from '../components/PoweredByOHC';


export default function CartRecoveryPage() {
  const router = useRouter();
  const [customerName, setCustomerName] = useState('');
  const [cartValue, setCartValue] = useState('');
  const [generatedDraft, setGeneratedDraft] = useState('');
  const [isGenerating, setIsGenerating] = useState(false);
  const [isSent, setIsSent] = useState(false);
  const [abandonedCartsCount, setAbandonedCartsCount] = useState<number>(0);
  const [hasPro, setHasPro] = useState(false);
  const [showSoftPaywall, setShowSoftPaywall] = useState(false);
  const [trialStatus, setTrialStatus] = useState('');
  const [isAutoEnabled, setIsAutoEnabled] = useState(false);

  useEffect(() => {
    if (typeof localStorage !== 'undefined') {
      setHasPro(localStorage.getItem('has_pro') === 'true');
    }
    const fetchAbandonedCartsCount = async () => {
      try {
        const res = await fetch('/api/v1/growth/campaign/abandoned-carts-count');
        const data = await res.json();
        setAbandonedCartsCount(res.ok ? (data.count ?? 0) : 0);
      } catch (e) {
        setAbandonedCartsCount(0);
      }
    };
    fetchAbandonedCartsCount();
  }, []);

  const generateDraft = async () => {
    setIsGenerating(true);
    try {
        const res = await fetch('/api/v1/growth/campaign/generate-cart', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({
                customer_name: customerName,
                cart_value: cartValue
            })
        });

        const data = await res.json();
        setGeneratedDraft(`${data.message}`);
        setIsGenerating(false);
        setIsSent(false);
    } catch (e) {
        console.error(e);
        setIsGenerating(false);
    }
  };

  const handleGenerate = () => {
    generateDraft();
  };

  const claimTrialExtension = () => {
    const tenant = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant_id') || 'DEFAULT' : 'DEFAULT';
    const referralUrl = `${window.location.origin}/onboarding?ref=${tenant}`;
    window.open(`https://twitter.com/intent/tweet?text=${encodeURIComponent('I just set up automated abandoned cart recovery for my business on One Human Corp! Start your own business today: ' + referralUrl)}`, '_blank');
    if (typeof localStorage !== 'undefined') {
        localStorage.setItem('has_pro', 'true');
    }
    setHasPro(true);
    setShowSoftPaywall(false);
    setTrialStatus('Your 7-day Pro trial has been activated.');
    generateDraft();
    setIsAutoEnabled(true);
  };

  const toggleAutoRecovery = () => {
    if (!hasPro) {
      setShowSoftPaywall(true);
      return;
    }
    setIsAutoEnabled(!isAutoEnabled);
  };

  const handleSend = async () => {
    if (!hasPro) {
      setShowSoftPaywall(true);
      return;
    }

    try {
      const res = await fetch('/api/v1/growth/campaign/send', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
           name: "Abandoned Cart Recovery",
           subject: "Recover your cart",
           body: generatedDraft,
           target_segment: "abandoned_carts"
        })
      });
      setIsSent(true);
    } catch (e) {
      console.error(e);

      setIsSent(true);
    }
  };

  return (
    <div className="flex flex-col min-h-screen font-inter" style={{ backgroundColor: '#F5F5F7' }}>
      {/* Header */}
      <header className="px-6 py-4 flex items-center justify-between border-b" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', borderBottom: '1px solid rgba(255, 255, 255, 0.4)', position: 'sticky', top: 0, zIndex: 50 }}>
         <h1 className="text-2xl font-bold font-outfit" style={{ color: '#1D1D1F', letterSpacing: '-0.02em' }}>Abandoned Cart Recovery 🛒</h1>
         <div className="flex items-center gap-3">
             <button onClick={() => router.push('/dashboard')} className="px-4 py-2 bg-gray-200 rounded-md text-sm font-medium hover:bg-gray-300 transition-colors">
               Back to Dashboard
             </button>
         </div>
      </header>

      <main className="p-6 md:p-8 flex-1 max-w-4xl mx-auto w-full flex flex-col gap-8">
        {trialStatus && <p className="rounded-lg border border-green-100 bg-green-50 px-4 py-3 text-sm font-semibold text-green-800" role="status">{trialStatus}</p>}
        <div className="bg-gradient-to-r from-orange-50 to-amber-50 border border-orange-100 rounded-2xl p-6 shadow-sm">
           <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2">Recover Abandoned Carts</h2>
           <p className="text-gray-600 text-sm">
             Generate highly-converting, personalized follow-up emails using AI for users who left items in their cart.
           </p>
        </div>

        {/* Auto Recovery Banner */}
        <section className="w-full p-6 shadow-md flex items-center justify-between" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)', borderRadius: '16px' }}>
          <div>
            <h3 className="text-lg font-semibold font-outfit m-0 mb-1" style={{ color: '#1D1D1F' }}>Automate with Agent Nova</h3>
            <p className="text-gray-600 text-sm m-0">Automatically generate and send recovery emails 4 hours after a cart is abandoned.</p>
          </div>
          <div className="flex items-center gap-3">
             {isAutoEnabled && <span className="text-sm font-bold text-green-600">Auto-Recovery Enabled</span>}
             <button
               id="auto-recovery-toggle"
               onClick={toggleAutoRecovery}
               className={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors focus:outline-none ${isAutoEnabled ? 'bg-orange-500' : 'bg-gray-200'}`}
             >
               <span className={`inline-block h-4 w-4 transform rounded-full bg-white transition-transform ${isAutoEnabled ? 'translate-x-6' : 'translate-x-1'}`} />
             </button>
          </div>
        </section>

        <div className="flex flex-col md:flex-row gap-8">
          {/* Campaign Settings */}
          <section className="w-full md:w-1/2 p-6 shadow-md" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)', borderRadius: '16px' }}>
            <div className="flex items-center gap-4 mb-4">
              <h3 className="text-xl font-semibold font-outfit m-0" style={{ color: '#1D1D1F' }}>Preview Context</h3>
              <div className="flex items-center gap-2 px-3 py-1 bg-yellow-50 rounded-full border border-yellow-100">
                  <span className="text-xs font-medium text-yellow-600">Pro Feature</span>
              </div>
            </div>
            <div className="flex flex-col gap-4">
              <div>
                <label htmlFor="customer-name" className="block text-sm font-medium text-gray-700 mb-1">Customer Name (Optional preview)</label>
                <input
                  id="customer-name"
                  type="text"
                  value={customerName}
                  onChange={(e) => setCustomerName(e.target.value)}
                  placeholder="e.g. Alice"
                  className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-[#FF9500]"
                />
              </div>
              <div>
                <label htmlFor="cart-value" className="block text-sm font-medium text-gray-700 mb-1">Cart Value (Optional preview)</label>
                <input
                  id="cart-value"
                  type="text"
                  value={cartValue}
                  onChange={(e) => setCartValue(e.target.value)}
                  placeholder="e.g. $45.00"
                  className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-[#FF9500]"
                />
              </div>
              <button
                onClick={handleGenerate}
                disabled={isGenerating}
                className={`w-full py-3 mt-4 text-white font-semibold rounded-xl shadow-lg transition-all flex items-center justify-center gap-2 ${isGenerating ? 'bg-orange-400 cursor-not-allowed' : 'bg-orange-600 hover:bg-orange-700 hover:shadow-xl hover:-translate-y-0.5 active:translate-y-0'}`}
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
                  <div className="mt-4 pt-4 border-t border-gray-200">
                    <PoweredByOHC tenantId="my-store" />
                  </div>
                </div>

                {isSent ? (
                  <div className="w-full py-3 bg-green-50 text-green-700 font-bold rounded-xl text-center border border-green-200">
                    ✅ Campaign sent to {abandonedCartsCount} abandoned carts!
                  </div>
                ) : (
                  <button
                    onClick={handleSend}
                    disabled={abandonedCartsCount === 0}
                    className={`w-full py-3 text-white font-bold rounded-xl shadow-md transition-all flex items-center justify-center gap-2 relative overflow-hidden group ${abandonedCartsCount === 0 ? 'bg-gray-400 cursor-not-allowed' : 'bg-gray-900 hover:bg-black'}`}
                  >
                    Send to {abandonedCartsCount} Abandoned Carts
                  </button>
                )}
              </div>
            ) : (
              <div className="flex-1 flex flex-col items-center justify-center text-gray-400 border-2 border-dashed border-gray-200 rounded-xl p-6 text-center">
                <svg className="w-12 h-12 mb-3 text-gray-300" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.5} d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" /></svg>
                <p className="text-sm font-medium">Configure your campaign to generate a high-converting recovery draft.</p>
              </div>
            )}
          </section>
        </div>
      </main>

      {/* Soft Paywall Modal */}
      {showSoftPaywall && (
        <div className="fixed inset-0 bg-black/60 z-[60] flex items-center justify-center p-4">
          <div className="app-card w-full max-w-md rounded-2xl p-8 shadow-2xl relative overflow-hidden font-inter border border-orange-100 text-center">
            <div className="absolute top-0 right-0 w-32 h-32 bg-orange-50 rounded-bl-full -z-10"></div>

            <div className="flex justify-end mb-2">
              <button
                onClick={() => setShowSoftPaywall(false)}
                className="text-gray-400 hover:text-gray-600 rounded-full hover:bg-gray-100 transition-colors w-8 h-8 flex items-center justify-center"
              >
                <span className="text-xl leading-none">&times;</span>
              </button>
            </div>

            <div className="w-16 h-16 bg-orange-100 rounded-2xl flex items-center justify-center text-3xl shadow-inner text-orange-600 mx-auto mb-6">
              🛒
            </div>
            <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-3">Upgrade to Pro</h2>
            <p className="text-gray-600 mb-6 text-sm leading-relaxed">
              Abandoned Cart Recovery is a Pro feature. Upgrade to our Pro plan to automatically recover lost sales.
            </p>

            <button
              onClick={() => { setShowSoftPaywall(false); window.location.href = '/pricing'; }}
              className="w-full py-4 rounded-xl font-bold text-white mb-4 transition-all shadow-md hover:shadow-lg hover:opacity-90 bg-gradient-to-r from-orange-500 to-amber-500"
            >
              Upgrade to Pro
            </button>

            <div className="my-4 text-gray-400 font-medium text-sm">OR</div>

            <button
              onClick={claimTrialExtension}
              className="w-full py-3.5 rounded-xl font-bold transition-all shadow-sm hover:bg-gray-50 flex items-center justify-center gap-2 border-2 border-[#1DA1F2] text-[#1DA1F2] bg-white"
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
