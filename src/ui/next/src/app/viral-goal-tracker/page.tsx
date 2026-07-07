"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';

export default function ViralGoalTrackerPage() {
  const router = useRouter();
  const [tenant, setTenant] = useState('my-business');
  const [target, setTarget] = useState('10');
  const [reward, setReward] = useState('Free T-Shirt & 20% Off');
  const [theme, setTheme] = useState<'light' | 'dark'>('light');
  const [copied, setCopied] = useState(false);
  const [hasPro, setHasPro] = useState(false);
  const [showPaywall, setShowPaywall] = useState(false);
  const [isClient, setIsClient] = useState(false);

  useEffect(() => {
    setIsClient(true);
    if (typeof localStorage !== 'undefined') {
      const storedTenant = localStorage.getItem('tenant') || 'my-business';
      setTenant(storedTenant);
      setHasPro(localStorage.getItem('has_pro') === 'true');
    }
    document.title = "Viral Goal Tracker | OHC";
  }, []);

  const handleRemoveBranding = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (!hasPro) {
      e.preventDefault();
      setShowPaywall(true);
    }
  };

  const origin = typeof window !== 'undefined' && window.location.origin.includes('localhost') ? 'https://app.onehumancorp.com' : (typeof window !== 'undefined' ? window.location.origin : '');
  const embedUrl = `${origin}/api/v1/growth/viral-goal-tracker?tenant=${tenant}&theme=${theme}&target=${target}&reward=${encodeURIComponent(reward)}&hideBranding=${hasPro}`;
  const embedCode = `<iframe src="${embedUrl}" width="100%" height="220" style="border:none;border-radius:16px;overflow:hidden;" title="OHC Viral Goal Tracker"></iframe>`;

  const handleCopy = () => {
    navigator.clipboard.writeText(embedCode);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  if (!isClient) return null;

  return (
    <div className="flex flex-col min-h-screen font-inter bg-gradient-to-br from-indigo-50 via-purple-50 to-pink-50 items-center justify-center py-10 px-4">
      <div className="w-full max-w-4xl bg-white/80 backdrop-blur-xl rounded-[24px] shadow-sm border border-gray-100 flex flex-col lg:flex-row gap-8">
        <div className="flex-1 p-8">
          <h1 className="text-3xl font-bold font-outfit text-gray-900 mb-6">Goal Tracker Builder</h1>
          <p className="text-sm text-gray-600 mb-6 leading-relaxed">
             Create a gamified progress bar to encourage referrals and engagement.
          </p>

          <div className="space-y-4">
             <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">Goal Target</label>
                <input type="number" min="1" value={target} onChange={(e) => setTarget(e.target.value)} className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500" />
             </div>
             <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">Reward Name</label>
                <input type="text" value={reward} onChange={(e) => setReward(e.target.value)} className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500" />
             </div>
             <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">Theme</label>
                <select value={theme} onChange={(e) => setTheme(e.target.value as any)} className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500 bg-white">
                  <option value="light">Light</option>
                  <option value="dark">Dark</option>
                </select>
             </div>
             <div className="flex items-center gap-2 mt-4 pt-4 border-t border-gray-200">
                <input
                    type="checkbox"
                    id="removeBranding"
                    checked={hasPro}
                    disabled={hasPro}
                    onChange={handleRemoveBranding}
                    className="w-4 h-4 text-indigo-600 border-gray-300 rounded focus:ring-indigo-500"
                />
                <label htmlFor="removeBranding" className="text-sm font-medium text-gray-700 flex items-center gap-2">
                    Remove "Powered by OHC" Badge
                    {!hasPro && <span className="bg-yellow-100 text-yellow-800 text-[10px] font-bold px-2 py-0.5 rounded uppercase tracking-wider">PRO</span>}
                </label>
             </div>
          </div>

          <div className="mt-8 bg-gray-900 text-gray-300 p-4 rounded-xl font-mono text-xs overflow-x-auto mb-4">
             <pre>{embedCode}</pre>
          </div>
          <button
             onClick={handleCopy}
             className={`w-full py-3 rounded-lg text-sm font-semibold transition-all ${copied ? 'bg-green-100 text-green-700' : 'bg-indigo-600 text-white hover:bg-indigo-700'}`}
          >
             {copied ? 'Copied to Clipboard!' : 'Copy Embed Code'}
          </button>
        </div>

        <div className="flex-1 flex flex-col p-8 bg-gray-50 border-l border-gray-100 rounded-r-[24px]">
           <h2 className="text-xl font-semibold font-outfit text-gray-900 mb-4">Live Preview</h2>

           <div className={`p-6 rounded-2xl shadow-sm border ${theme === 'dark' ? 'bg-[#1c1c1e] text-white border-[#333]' : 'bg-white text-gray-900 border-gray-200'}`}>
              <div className="text-center mb-4">
                 <h3 className="font-bold text-lg">Unlock: {reward}</h3>
                 <p className={`text-sm ${theme === 'dark' ? 'text-gray-400' : 'text-gray-500'}`}>Invite friends to unlock your reward!</p>
              </div>

              <div className="w-full bg-gray-200 rounded-full h-3 mb-2 overflow-hidden dark:bg-gray-700">
                  <div className="bg-indigo-600 h-3 rounded-full" style={{ width: '40%' }}></div>
              </div>
              <div className={`flex justify-between text-xs mb-6 ${theme === 'dark' ? 'text-gray-400' : 'text-gray-500'}`}>
                  <span>4 referrals completed</span>
                  <span>{target} target</span>
              </div>

              <button className="w-full py-2 bg-indigo-600 hover:bg-indigo-700 text-white rounded-lg text-sm font-medium transition-colors mb-4">
                 Share to reach goal
              </button>

              {!hasPro && (
                 <div className="text-center">
                    <a href="#" className={`text-xs font-medium hover:underline ${theme === 'dark' ? 'text-gray-400' : 'text-gray-400 hover:text-gray-600'}`}>
                       ⚡ Powered by OHC
                    </a>
                 </div>
              )}
           </div>
        </div>
      </div>

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
               Make the Goal Tracker Widget 100% yours. Upgrade to Pro to remove the "Powered by OHC" watermark.
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
    </div>
  );
}
