"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';
import { useProPlan } from '../components/useProPlan';
import '../globals.css';

export default function ViralGiveGetWidgetPage() {
  const router = useRouter();
  const [giveReward, setGiveReward] = useState('20% Off');
  const [getReward, setGetReward] = useState('$10 Credit');
  const [tenantId, setTenantId] = useState('default-team');
  const [referralLink, setReferralLink] = useState('');
  const [generating, setGenerating] = useState(false);
  const [copied, setCopied] = useState(false);
  const [showResult, setShowResult] = useState(false);
  const [boxesActive, setBoxesActive] = useState(false);

  const { hasPro } = useProPlan();
  const [showPaywall, setShowPaywall] = useState(false);
  const [removeBranding, setRemoveBranding] = useState(false);
  const [isClient, setIsClient] = useState(false);

  useEffect(() => {
    setIsClient(true);
    if (typeof window !== 'undefined') {
      const storedTenant = localStorage.getItem('business_display_name') || 'My Business';
      setTenantId(storedTenant);
    }
  }, []);

  const handleGenerate = async () => {
    setGenerating(true);
    setBoxesActive(false);
    setShowResult(false);

    try {
      const res = await fetch('/api/v1/growth/referrals/generate', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({
          customMessage: 'Give Get Generator'
        })
      });

      if (!res.ok) {
        throw new Error('Failed to generate referral link');
      }
      const data = await res.json();
      let refId = tenantId;
      if (data.referral_link) {
        refId = data.referral_link.split('/').pop() || tenantId;
      }

      const baseUrl = typeof window !== 'undefined' ? window.location.origin : 'https://ohc.app';
      setReferralLink(`${baseUrl}/give-get/join?ref=${refId}`);

      setTimeout(() => setBoxesActive(true), 200);
      setTimeout(() => setShowResult(true), 1000);

    } catch (err) {
      console.error("Error generating link:", err);
      // Fallback removed to enforce strict flow
      throw err;
    } finally {
      setTimeout(() => setGenerating(false), 1000);
    }
  };

  const handleCopy = () => {
    if (typeof navigator !== 'undefined' && navigator.clipboard) {
      navigator.clipboard.writeText(referralLink);
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    }
  };

  const handleBrandingToggle = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (!hasPro) {
        e.preventDefault();
        setShowPaywall(true);
        return;
    }
    setRemoveBranding(e.target.checked);
  };

  if (!isClient) return null;

  return (
    <div className="min-h-screen bg-gray-50 flex flex-col font-inter">
      <header className="bg-white border-b border-gray-200 px-6 py-4 flex items-center justify-between sticky top-0 z-10 shadow-sm">
        <h1 className="text-2xl font-bold font-outfit text-[#1D1D1F] tracking-tight">Viral Give-Get Generator 🎁</h1>
        <button
          onClick={() => router.push('/dashboard')}
          className="px-4 py-2 bg-gray-200 min-h-[44px] min-w-[44px] text-sm font-medium hover:bg-gray-300 rounded-lg transition-colors"
        >
          Back to Dashboard
        </button>
      </header>

      <main className="p-6 md:p-8 flex-1 max-w-3xl mx-auto w-full flex flex-col gap-8">

        <div className="text-center">
            <div className="text-5xl mb-4">🎁</div>
            <h2 className="text-3xl font-bold font-outfit text-gray-900 mb-2">Viral Give-Get Generator</h2>
            <p className="text-gray-600 max-w-lg mx-auto">Create a classic "Give 20%, Get $10" referral program. Friends get a discount, and you get rewarded when they buy.</p>
        </div>

        <div className="p-8 shadow-[0_8px_30px_rgb(0,0,0,0.12)] bg-white/60 backdrop-blur-[40px] saturate-[200%] border border-white/40 rounded-3xl">

          <div className="space-y-6 mb-8">
            <div>
              <label htmlFor="give-reward" className="block text-sm font-medium text-gray-700 mb-2">Give (Friend's Reward)</label>
              <input
                type="text"
                id="give-reward"
                value={giveReward}
                onChange={(e) => setGiveReward(e.target.value)}
                className="w-full px-4 py-3 border border-gray-300/50 rounded-xl bg-white/50 backdrop-blur-sm min-h-[44px] min-w-[44px] focus:outline-none focus:ring-2 focus:ring-[#0066FF] transition-all text-gray-900"
              />
            </div>

            <div>
              <label htmlFor="get-reward" className="block text-sm font-medium text-gray-700 mb-2">Get (Your Reward)</label>
              <input
                type="text"
                id="get-reward"
                value={getReward}
                onChange={(e) => setGetReward(e.target.value)}
                className="w-full px-4 py-3 border border-gray-300/50 rounded-xl bg-white/50 backdrop-blur-sm min-h-[44px] min-w-[44px] focus:outline-none focus:ring-2 focus:ring-[#0066FF] transition-all text-gray-900"
              />
            </div>

            <div className="pt-4 border-t border-gray-200">
                <label className="flex items-center gap-2 text-sm font-medium text-gray-700 cursor-pointer">
                    <input
                        type="checkbox"
                        checked={removeBranding}
                        onChange={handleBrandingToggle}
                        className="w-4 h-4 text-indigo-600 border-gray-300 rounded focus:ring-indigo-500"
                    />
                    Remove "Powered by OHC" Badge
                    {!hasPro && <span className="bg-yellow-100 text-yellow-800 text-[10px] font-bold px-2 py-0.5 rounded uppercase tracking-wider ml-1">PRO</span>}
                </label>
            </div>
          </div>

          <div className="flex items-center justify-around p-6 bg-blue-50/50 rounded-2xl border border-blue-100/50 mb-8 relative">
            <div id="give-box" className={`w-32 p-4 text-center rounded-2xl transition-all duration-500 transform ${boxesActive ? 'bg-[#0066FF] text-white scale-105 shadow-lg' : 'bg-white border-2 border-dashed border-[#0066FF] text-gray-900 opacity-70 scale-100'}`}>
              <h3 className="text-xs font-bold uppercase tracking-wider mb-2 opacity-80">Give</h3>
              <p id="give-display" className="text-xl font-bold font-outfit">{giveReward}</p>
            </div>

            <div className="text-3xl text-gray-400 z-10">➡️</div>

            <div id="get-box" className={`w-32 p-4 text-center rounded-2xl transition-all duration-500 transform delay-200 ${boxesActive ? 'bg-[#0066FF] text-white scale-105 shadow-lg' : 'bg-white border-2 border-dashed border-[#0066FF] text-gray-900 opacity-70 scale-100'}`}>
              <h3 className="text-xs font-bold uppercase tracking-wider mb-2 opacity-80">Get</h3>
              <p id="get-display" className="text-xl font-bold font-outfit">{getReward}</p>
            </div>

            {!removeBranding && (
                <div className="absolute -bottom-6 left-0 right-0 text-center">
                   <a href={`/onboarding?ref=${tenantId}`} target="_blank" rel="noopener noreferrer" className="text-[10px] font-semibold text-gray-400 hover:text-gray-600 transition-colors uppercase tracking-wider">
                       ⚡ Powered by OHC
                   </a>
                </div>
            )}
          </div>

          <button
            id="generate-btn"
            onClick={handleGenerate}
            disabled={generating}
            className="w-full py-4 mt-6 bg-[#0066FF] hover:bg-blue-700 disabled:bg-blue-400 text-white font-bold rounded-xl shadow-[0_4px_14px_0_rgba(0,102,255,0.39)] transition-all min-h-[44px] flex items-center justify-center text-lg"
          >
            {generating ? 'Generating...' : 'Generate Referral Link'}
          </button>

          {showResult && (
            <div id="result-area" className="mt-8 pt-8 border-t border-gray-200 animate-in fade-in slide-in-from-bottom-4 duration-500">
              <h2 className="text-xl font-bold font-outfit text-gray-900 mb-2">Link Generated Successfully!</h2>
              <p className="text-gray-600 mb-4 font-medium text-sm">Share this link to start your Give-Get loop:</p>

              <div className="flex gap-2">
                <input
                  type="text"
                  id="share-link"
                  readOnly
                  value={referralLink}
                  className="flex-1 px-4 py-3 bg-white/80 border border-gray-300 rounded-xl focus:outline-none focus:ring-2 focus:ring-[#0066FF] text-gray-700 min-h-[44px]"
                />
                <button
                  id="copy-btn"
                  onClick={handleCopy}
                  className="px-6 py-3 bg-gray-900 hover:bg-gray-800 text-white font-semibold rounded-xl transition-colors min-h-[44px] min-w-[100px]"
                >
                  {copied ? 'Copied!' : 'Copy'}
                </button>
              </div>
            </div>
          )}

        </div>
      </main>

      {showPaywall && (
        <div className="fixed inset-0 bg-black/60 z-[60] flex items-center justify-center p-4">
          <div className="bg-white rounded-2xl max-w-md p-8 shadow-2xl relative overflow-hidden font-inter border border-blue-100 text-center">
             <div className="absolute top-0 right-0 w-32 h-32 bg-blue-50 rounded-bl-full -z-10"></div>
             <div className="flex justify-end mb-2">
               <button
                 aria-label="Close paywall"
                 onClick={() => setShowPaywall(false)}
                 className="text-gray-400 hover:text-gray-600 rounded-full hover:bg-gray-100 transition-colors w-8 h-8 flex items-center justify-center"
               >
                 <span className="text-xl leading-none">&times;</span>
               </button>
             </div>
             <div className="w-16 h-16 bg-gradient-to-br from-indigo-500 to-purple-600 rounded-2xl flex items-center justify-center text-3xl shadow-lg mx-auto mb-6 text-white font-bold">
               PRO
             </div>
             <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-3">Upgrade to Remove Branding</h2>
             <p className="text-gray-600 mb-6 text-sm leading-relaxed">
               Make the Give-Get Generator 100% yours. Upgrade to Pro to remove the "Powered by OHC" watermark.
             </p>
             <button
               onClick={() => { setShowPaywall(false); window.location.href = '/pricing'; }}
               className="w-full py-4 rounded-xl font-bold text-white mb-4 transition-all shadow-md hover:shadow-lg hover:opacity-90"
               style={{ background: 'linear-gradient(135deg, #0066ff 0%, #3b82f6 100%)' }}
             >
               Upgrade to Pro
             </button>
             <button
               onClick={() => setShowPaywall(false)}
               className="mt-2 text-gray-500 hover:text-gray-700 font-medium text-sm w-full"
             >
               Cancel
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
