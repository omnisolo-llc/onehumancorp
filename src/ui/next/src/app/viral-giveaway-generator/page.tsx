"use client";

import React, { useState, useEffect } from 'react';
import { useProPlan } from '../components/useProPlan';
import { useRouter } from 'next/navigation';
import { PoweredByOHC } from "../components/PoweredByOHC";

export default function ViralGiveawayGeneratorPage() {
  const router = useRouter();
  const [tenant, setTenant] = useState('my-store');
  const [theme, setTheme] = useState<'light' | 'dark'>('light');
  const [prize, setPrize] = useState('$500 Store Credit');
  const [duration, setDuration] = useState('7');
  const [copied, setCopied] = useState(false);
  const { hasPro } = useProPlan();
  const [showPaywall, setShowPaywall] = useState(false);
  const [isClient, setIsClient] = useState(false);
  const [showModal, setShowModal] = useState(false);
  const [removeBranding, setRemoveBranding] = useState(false);

  useEffect(() => {
    setIsClient(true);
    if (typeof localStorage !== 'undefined') {
      const storedTenant = localStorage.getItem('business_display_name') || 'my-store';
      setTenant(storedTenant);
    }
    document.title = "Viral Giveaway Generator | OHC";
  }, []);

  const handleRemoveBranding = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (!hasPro) {
      e.preventDefault();
      setShowPaywall(true);
    } else {
      setRemoveBranding(e.target.checked);
    }
  };

  const embedUrl = `https://ohc.app/api/v1/growth/viral-giveaway/embed?tenant=${tenant}&theme=${theme}&prize=${encodeURIComponent(prize)}&duration=${encodeURIComponent(duration)}&branding=${!removeBranding}`;

  const embedCode = `<iframe src="${embedUrl}" width="100%" height="450" frameborder="0" scrolling="no" style="border:none; overflow:hidden; border-radius:16px;"></iframe>` + (removeBranding ? '' : `\n<div style="font-family: sans-serif; text-align: center; font-size: 12px; margin-top: 8px;"><a href="https://ohc.app/api/v1/growth/referrals/click?target=/onboarding&ref=${tenant}" target="_blank" style="color: #6b7280; text-decoration: none; font-weight: 600;">⚡ Powered by OHC</a></div>`);

  const handleCopy = () => {
    navigator.clipboard.writeText(embedCode);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  const handleGenerate = () => {
    setShowModal(true);
  };

  if (!isClient) return null;

  return (
    <div className="min-h-screen bg-gray-50 dark:bg-black font-inter py-8 px-4 sm:px-6 lg:px-8">
      <div className="max-w-6xl mx-auto flex flex-col lg:flex-row gap-8">

        {/* Left Column - Configuration */}
        <div className="flex-1 space-y-6">
          <div className="bg-white dark:bg-[#16161a] rounded-[24px] shadow-sm border border-gray-200 dark:border-gray-800 p-8">
            <h1 className="text-3xl font-bold font-outfit text-gray-900 dark:text-white mb-2">Viral Giveaway Generator</h1>
            <p className="text-gray-600 dark:text-gray-400 mb-8">
              Launch a viral sweepstakes to capture emails and drive social shares.
              Users get extra entries for referring friends.
            </p>

            <div className="space-y-6">
              <div>
                <label className="block text-sm font-semibold text-gray-700 dark:text-gray-300 mb-2">Prize</label>
                <input
                  type="text"
                  value={prize}
                  onChange={(e) => setPrize(e.target.value)}
                  className="w-full px-4 py-3 bg-gray-50 dark:bg-black border border-gray-200 dark:border-gray-800 rounded-xl focus:ring-2 focus:ring-pink-500 focus:border-transparent transition-all dark:text-white"
                  placeholder="e.g., $500 Store Credit"
                />
              </div>

              <div>
                <label className="block text-sm font-semibold text-gray-700 dark:text-gray-300 mb-2">Duration (Days)</label>
                <input
                  type="number"
                  value={duration}
                  onChange={(e) => setDuration(e.target.value)}
                  className="w-full px-4 py-3 bg-gray-50 dark:bg-black border border-gray-200 dark:border-gray-800 rounded-xl focus:ring-2 focus:ring-pink-500 focus:border-transparent transition-all dark:text-white"
                  placeholder="e.g., 7"
                />
              </div>

              <div>
                <label className="block text-sm font-semibold text-gray-700 dark:text-gray-300 mb-2">Widget Theme</label>
                <div className="flex gap-4">
                  <label className="flex items-center gap-2 cursor-pointer">
                    <input
                      type="radio"
                      checked={theme === 'light'}
                      onChange={() => setTheme('light')}
                      className="text-pink-600 focus:ring-pink-500"
                    />
                    <span className="text-gray-700 dark:text-gray-300">Light</span>
                  </label>
                  <label className="flex items-center gap-2 cursor-pointer">
                    <input
                      type="radio"
                      checked={theme === 'dark'}
                      onChange={() => setTheme('dark')}
                      className="text-pink-600 focus:ring-pink-500"
                    />
                    <span className="text-gray-700 dark:text-gray-300">Dark</span>
                  </label>
                </div>
              </div>

              <div className="pt-6 border-t border-gray-100 dark:border-gray-800">
                <label className="flex items-center gap-3 cursor-pointer group">
                  <div className="relative flex items-center justify-center">
                    <input
                      type="checkbox"
                      checked={removeBranding}
                      onChange={handleRemoveBranding}
                      className="w-5 h-5 border-2 border-gray-300 dark:border-gray-600 rounded bg-transparent checked:bg-pink-500 checked:border-pink-500 transition-all appearance-none cursor-pointer"
                    />
                    <svg className={`absolute w-3 h-3 text-white pointer-events-none transition-opacity ${removeBranding ? 'opacity-100' : 'opacity-0'}`} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="3" strokeLinecap="round" strokeLinejoin="round"><polyline points="20 6 9 17 4 12"></polyline></svg>
                  </div>
                  <div className="flex items-center gap-2">
                    <span className="text-sm font-medium text-gray-900 dark:text-gray-200 group-hover:text-pink-600 transition-colors">Remove "Powered by OHC" Badge</span>
                    {!hasPro && <span className="px-2 py-0.5 rounded text-[10px] font-bold tracking-wide uppercase bg-gradient-to-r from-amber-200 to-yellow-400 text-yellow-900 shadow-sm border border-yellow-300/50">Pro</span>}
                  </div>
                </label>
                <p className="mt-2 text-xs text-gray-500 dark:text-gray-400 ml-8">
                  Removing the branding hides the referral link that earns you affiliate credit.
                </p>
              </div>

              <button
                onClick={handleGenerate}
                className="w-full py-4 bg-gradient-to-r from-pink-500 to-rose-500 hover:from-pink-600 hover:to-rose-600 text-white rounded-xl font-bold shadow-lg shadow-pink-500/25 transition-all active:scale-[0.98] mt-8"
              >
                Generate Embed Code
              </button>
            </div>
          </div>
        </div>

        {/* Right Column - Preview */}
        <div className="flex-1">
          <div className="sticky top-8">
            <h2 className="text-lg font-semibold font-outfit text-gray-900 dark:text-white mb-4 flex items-center gap-2">
              <span className="w-2 h-2 rounded-full bg-green-500 animate-pulse"></span>
              Live Preview
            </h2>
            <div className={`rounded-[24px] shadow-2xl border ${theme === 'dark' ? 'border-gray-800 bg-[#0f0f11]' : 'border-gray-200 bg-white'} overflow-hidden min-h-[450px] flex flex-col transition-colors duration-300`}>

              {/* Widget UI Mock */}
              <div className="p-8 flex flex-col items-center justify-center flex-1 text-center">
                <div className="w-16 h-16 rounded-2xl bg-gradient-to-br from-pink-400 to-rose-500 text-white flex items-center justify-center text-3xl shadow-lg shadow-pink-500/30 mb-6 transform -rotate-6">
                  🎁
                </div>
                <h3 className={`text-2xl font-bold font-outfit mb-2 ${theme === 'dark' ? 'text-white' : 'text-gray-900'}`}>
                  Win {prize || '[Prize]'}
                </h3>
                <p className={`text-sm mb-6 ${theme === 'dark' ? 'text-gray-400' : 'text-gray-600'}`}>
                  Ends in {duration || '7'} days. Enter your email to win!
                </p>

                <div className="w-full max-w-sm space-y-3">
                  <input
                    type="email"
                    placeholder="Enter your email"
                    className={`w-full px-4 py-3 rounded-xl border text-sm ${theme === 'dark' ? 'bg-[#1a1a20] border-gray-800 text-white placeholder-gray-500' : 'bg-gray-50 border-gray-200 text-gray-900'}`}
                    readOnly
                  />
                  <button className="w-full py-3 bg-pink-500 hover:bg-pink-600 text-white rounded-xl font-bold shadow-md transition-colors text-sm">
                    Enter Giveaway
                  </button>
                </div>

                <div className={`mt-6 p-4 rounded-xl text-xs w-full max-w-sm ${theme === 'dark' ? 'bg-[#1a1a20] text-gray-400' : 'bg-pink-50 text-pink-700'}`}>
                  <strong className="block mb-1">Boost Your Chances!</strong>
                  Get 3 extra entries for every friend you refer.
                </div>
              </div>

              {!removeBranding && (
                <div className={`py-3 text-center border-t ${theme === 'dark' ? 'border-gray-800 bg-black' : 'border-gray-100 bg-gray-50'}`}>
                  <PoweredByOHC tenantId={tenant} />
                </div>
              )}
            </div>
          </div>
        </div>
      </div>

      {/* Embed Code Modal */}
      {showModal && (
        <div className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/60 backdrop-blur-sm">
          <div className="bg-white dark:bg-[#16161a] rounded-2xl p-6 w-full max-w-2xl shadow-2xl border border-gray-200 dark:border-gray-800 animate-in fade-in zoom-in duration-200">
            <div className="flex justify-between items-center mb-4">
              <h3 className="text-xl font-bold text-gray-900 dark:text-white font-outfit">Your Embed Code</h3>
              <button onClick={() => setShowModal(false)} className="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300">
                <svg className="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M6 18L18 6M6 6l12 12" /></svg>
              </button>
            </div>
            <p className="text-sm text-gray-600 dark:text-gray-400 mb-4">
              Copy and paste this code into your website's HTML where you want the widget to appear.
            </p>
            <div className="relative">
              <pre className="w-full p-4 bg-gray-50 dark:bg-[#0a0a0c] border border-gray-200 dark:border-gray-800 rounded-xl text-sm font-mono text-gray-800 dark:text-gray-300 overflow-x-auto whitespace-pre-wrap">
                {embedCode}
              </pre>
              <button
                onClick={handleCopy}
                className="absolute top-2 right-2 px-4 py-2 bg-white dark:bg-[#1a1a20] border border-gray-200 dark:border-gray-700 text-gray-700 dark:text-gray-200 rounded-lg text-sm font-medium hover:bg-gray-50 dark:hover:bg-gray-800 transition-colors shadow-sm"
              >
                {copied ? 'Copied!' : 'Copy Code'}
              </button>
            </div>
          </div>
        </div>
      )}

      {/* Paywall */}
      {showPaywall && (
        <div className="fixed inset-0 z-[100] flex items-center justify-center p-4 bg-black/80 backdrop-blur-md">
          <div className="bg-white dark:bg-[#16161a] rounded-3xl w-full max-w-md p-8 shadow-2xl relative overflow-hidden border border-gray-200 dark:border-gray-800">
            <div className="absolute top-0 right-0 w-32 h-32 bg-gradient-to-br from-pink-500/20 to-rose-500/20 rounded-bl-full -z-10 blur-2xl"></div>

            <div className="w-16 h-16 bg-gradient-to-br from-pink-500 to-rose-500 rounded-2xl flex items-center justify-center text-2xl shadow-lg shadow-pink-500/30 mb-6 text-white font-bold mx-auto">
              PRO
            </div>

            <h2 className="text-2xl font-bold font-outfit text-gray-900 dark:text-white mb-3 text-center">
              White-label Your Widgets
            </h2>
            <p className="text-gray-600 dark:text-gray-400 mb-8 text-sm leading-relaxed text-center">
              Make the Viral Giveaway 100% yours. Upgrade to Pro to remove the "Powered by OHC" watermark and unlock premium widget themes.
            </p>

            <div className="space-y-3">
              <button
                onClick={() => { setShowPaywall(false); router.push('/pricing'); }}
                className="w-full py-3.5 bg-gradient-to-r from-pink-500 to-rose-500 hover:from-pink-600 hover:to-rose-600 text-white rounded-xl font-bold shadow-lg shadow-pink-500/25 transition-all"
              >
                Upgrade to Pro
              </button>
              <button
                onClick={() => setShowPaywall(false)}
                className="w-full py-3.5 bg-gray-100 hover:bg-gray-200 dark:bg-[#202026] dark:hover:bg-[#2a2a32] text-gray-900 dark:text-white rounded-xl font-semibold transition-all"
              >
                Keep Branding
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
