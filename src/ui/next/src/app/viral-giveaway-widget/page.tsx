"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';
import { useProPlan } from '../components/useProPlan';
import { PoweredByOHC } from '../components/PoweredByOHC';

export default function ViralGiveawayWidgetPage() {
  const router = useRouter();
  const { hasPro } = useProPlan();
  const [isClient, setIsClient] = useState(false);
  const [tenant, setTenant] = useState('my-business');

  const [giveawayTitle, setGiveawayTitle] = useState('Win a Free Consultation');
  const [giveawayPrize, setGiveawayPrize] = useState('$500 Value Service Package');
  const [winnersCount, setWinnersCount] = useState(1);

  const [generating, setGenerating] = useState(false);
  const [showResult, setShowResult] = useState(false);
  const [referralLink, setReferralLink] = useState('');
  const [copied, setCopied] = useState(false);
  const [removeBranding, setRemoveBranding] = useState(false);
  const [showPaywall, setShowPaywall] = useState(false);

  useEffect(() => {
    setIsClient(true);
    if (typeof localStorage !== 'undefined') {
      const storedTenant = localStorage.getItem('business_display_name') || 'my-business';
      setTenant(storedTenant);
    }
  }, []);

  const handleGenerate = async () => {
    setGenerating(true);
    try {
      // Simulate API call for link generation
      await new Promise(resolve => setTimeout(resolve, 800));

      const baseUrl = typeof window !== 'undefined' ? window.location.origin : 'https://ohc.app';
      const refId = tenant.replace(/[^a-zA-Z0-9]/g, '').toLowerCase() || 'giveaway';
      setReferralLink(`${baseUrl}/giveaway/enter?ref=${refId}&c=${Date.now().toString().slice(-6)}`);

      setShowResult(true);
    } catch (err) {
      console.error("Error generating link:", err);
    } finally {
      setGenerating(false);
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
    <div className="min-h-screen bg-gradient-to-br from-indigo-50 via-purple-50 to-pink-50 flex flex-col font-inter">
      <header className="bg-white/80 backdrop-blur-md border-b border-gray-200 px-6 py-4 flex items-center justify-between sticky top-0 z-10 shadow-sm">
        <h1 className="text-2xl font-bold font-outfit text-[#1D1D1F] tracking-tight">Viral Giveaway Generator 🏆</h1>
        <button
          onClick={() => router.push('/dashboard')}
          className="px-4 py-2 bg-gray-200 min-h-[44px] min-w-[44px] text-sm font-medium hover:bg-gray-300 rounded-lg transition-colors"
        >
          Back to Dashboard
        </button>
      </header>

      <main className="p-6 md:p-8 flex-1 max-w-6xl mx-auto w-full flex flex-col md:flex-row gap-8">

        {/* Left column: Editor */}
        <div className="w-full md:w-1/3 flex flex-col gap-6">
          <div className="p-6 shadow-[0_8px_30px_rgb(0,0,0,0.12)] bg-white/60 backdrop-blur-[40px] saturate-[200%] border border-white/40 rounded-3xl">
            <h2 className="text-xl font-bold font-outfit text-gray-900 mb-6">Giveaway Settings</h2>
            <p className="text-gray-600 text-sm mb-6">Configure a viral giveaway. Entrants get bonus entries when their friends join.</p>

            <div className="space-y-5">
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-2">Giveaway Title</label>
                <input
                  type="text"
                  value={giveawayTitle}
                  onChange={(e) => setGiveawayTitle(e.target.value)}
                  className="w-full px-4 py-3 border border-gray-300/50 rounded-xl bg-white/50 backdrop-blur-sm min-h-[44px] focus:outline-none focus:ring-2 focus:ring-[#0066FF] transition-all text-gray-900"
                />
              </div>

              <div>
                <label className="block text-sm font-medium text-gray-700 mb-2">The Prize</label>
                <input
                  type="text"
                  value={giveawayPrize}
                  onChange={(e) => setGiveawayPrize(e.target.value)}
                  className="w-full px-4 py-3 border border-gray-300/50 rounded-xl bg-white/50 backdrop-blur-sm min-h-[44px] focus:outline-none focus:ring-2 focus:ring-[#0066FF] transition-all text-gray-900"
                />
              </div>

              <div>
                <label className="block text-sm font-medium text-gray-700 mb-2">Number of Winners</label>
                <input
                  type="number"
                  min="1"
                  max="100"
                  value={winnersCount}
                  onChange={(e) => setWinnersCount(parseInt(e.target.value) || 1)}
                  className="w-full px-4 py-3 border border-gray-300/50 rounded-xl bg-white/50 backdrop-blur-sm min-h-[44px] focus:outline-none focus:ring-2 focus:ring-[#0066FF] transition-all text-gray-900"
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

              <button
                id="generate-btn"
                onClick={handleGenerate}
                disabled={generating}
                className="w-full py-4 mt-2 bg-[#0066FF] hover:bg-blue-700 disabled:bg-blue-400 text-white font-bold rounded-xl shadow-[0_4px_14px_0_rgba(0,102,255,0.39)] transition-all min-h-[44px] flex items-center justify-center text-lg"
              >
                {generating ? 'Generating...' : 'Generate Widget'}
              </button>
            </div>
          </div>

          {showResult && (
            <div id="result-area" className="p-6 shadow-[0_8px_30px_rgb(0,0,0,0.12)] bg-indigo-50/70 backdrop-blur-[40px] border border-indigo-100/50 rounded-3xl animate-in fade-in slide-in-from-bottom-4 duration-500">
              <h3 className="font-bold text-indigo-900 mb-2 flex items-center gap-2">
                <span className="text-xl">🚀</span> Share Your Link
              </h3>
              <p className="text-sm text-indigo-800 mb-4">
                Your giveaway is ready! Share this link to start collecting entries.
              </p>

              <div className="flex items-center gap-2 bg-white min-h-[44px] border border-indigo-200 p-1 rounded-lg mb-4 overflow-hidden">
                <div className="px-2 py-1 text-xs text-gray-500 truncate flex-1 font-mono">
                  {referralLink}
                </div>
              </div>

              <button
                id="copy-btn"
                onClick={handleCopy}
                className="w-full py-3 bg-indigo-600 hover:bg-indigo-700 text-white font-medium min-h-[44px] rounded-xl transition-colors shadow-md"
              >
                {copied ? 'Copied!' : 'Copy Link'}
              </button>
            </div>
          )}
        </div>

        {/* Right column: Preview */}
        <div className="w-full md:w-2/3 flex flex-col">
          <div className="flex-1 shadow-[0_20px_40px_rgb(0,0,0,0.15)] overflow-hidden flex flex-col bg-white/40 backdrop-blur-[40px] saturate-[200%] border border-white/50 rounded-3xl relative">

            <div className="bg-gray-200 py-3 px-4 flex items-center gap-2 border-b border-gray-300">
              <div className="flex gap-1.5">
                <div className="w-3 h-3 rounded-full bg-red-400"></div>
                <div className="w-3 h-3 rounded-full bg-amber-400"></div>
                <div className="w-3 h-3 rounded-full bg-green-400"></div>
              </div>
              <div className="mx-auto bg-white/60 text-xs text-gray-500 px-4 py-1 rounded-full w-1/2 text-center truncate font-mono">
                Preview: Your Landing Page
              </div>
            </div>

            <div className="flex-1 flex items-center justify-center p-6 md:p-12 bg-gradient-to-br from-indigo-100 to-purple-100 overflow-y-auto">

              <div className="w-full max-w-lg bg-white shadow-2xl rounded-3xl overflow-hidden">
                <div className="bg-gradient-to-r from-purple-600 to-indigo-600 p-8 text-center text-white relative">
                  <div className="absolute top-4 right-4 bg-white/20 backdrop-blur-sm px-3 py-1 rounded-full text-xs font-bold uppercase tracking-wider">
                    {winnersCount} {winnersCount === 1 ? 'Winner' : 'Winners'}
                  </div>
                  <div className="text-6xl mb-4 mt-2 filter drop-shadow-md">🏆</div>
                  <h2 className="text-3xl font-bold font-outfit mb-2">{giveawayTitle || 'Enter to Win!'}</h2>
                  <p className="text-indigo-100 text-lg font-medium">{giveawayPrize}</p>
                </div>

                <div className="p-8">
                  <div className="mb-6 bg-purple-50 border border-purple-100 p-4 rounded-2xl flex items-start gap-4">
                     <div className="text-2xl mt-1">🚀</div>
                     <div>
                         <h4 className="font-bold text-gray-900 text-sm mb-1">Viral Multiplier Active!</h4>
                         <p className="text-xs text-gray-600">Enter your email below, then share your unique link. You get <strong className="text-purple-600">+3 bonus entries</strong> for every friend who joins!</p>
                     </div>
                  </div>

                  <form className="space-y-4" onSubmit={(e) => e.preventDefault()}>
                    <div>
                      <input type="text" placeholder="Your Name" className="w-full px-4 py-3 bg-gray-50 border border-gray-200 rounded-xl focus:outline-none focus:ring-2 focus:ring-purple-500" readOnly />
                    </div>
                    <div>
                      <input type="email" placeholder="Your Email Address" className="w-full px-4 py-3 bg-gray-50 border border-gray-200 rounded-xl focus:outline-none focus:ring-2 focus:ring-purple-500" readOnly />
                    </div>
                    <button className="w-full py-4 bg-gray-900 text-white font-bold rounded-xl shadow-lg hover:bg-gray-800 transition-colors">
                      Enter Giveaway Now
                    </button>
                  </form>

                  {!removeBranding && (
                    <div className="mt-8 text-center">
                      <PoweredByOHC tenantId={tenant} />
                    </div>
                  )}
                </div>
              </div>

            </div>
          </div>
        </div>

      </main>

      {/* Paywall */}
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
               Make the Giveaway Widget 100% yours. Upgrade to Pro to remove the "Powered by OHC" watermark.
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
