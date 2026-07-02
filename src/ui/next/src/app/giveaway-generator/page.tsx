"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';
import Link from 'next/link';

export default function GiveawayGeneratorPage() {
  const router = useRouter();
  const [tenant, setTenant] = useState('my-store');
  const [theme, setTheme] = useState<'light' | 'dark'>('light');
  const [title, setTitle] = useState('Win a Free Custom Cake!');
  const [prize, setPrize] = useState('1 Tier Custom Cake (Value $150)');
  const [copied, setCopied] = useState(false);
  const [hasPro, setHasPro] = useState(false);
  const [showPaywall, setShowPaywall] = useState(false);
  const [isClient, setIsClient] = useState(false);

  useEffect(() => {
    setIsClient(true);
    if (typeof localStorage !== 'undefined') {
      const storedTenant = localStorage.getItem('tenant') || 'my-store';
      setTenant(storedTenant);
      setHasPro(localStorage.getItem('has_pro') === 'true');
    }
    document.title = "Viral Giveaway Generator | OHC";
  }, []);

  const handleRemoveBranding = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (!hasPro) {
      e.preventDefault();
      setShowPaywall(true);
    }
  };

  const embedUrl = `/api/v1/growth/giveaway/embed?tenant=${tenant}&theme=${theme}&title=${encodeURIComponent(title)}&prize=${encodeURIComponent(prize)}&branding=${!hasPro}`;
  const embedCode = `<iframe src="https://ohc.app${embedUrl}" width="100%" height="600" frameborder="0" scrolling="no" style="border:none; overflow:hidden; border-radius:16px;"></iframe>`;

  const handleCopy = () => {
    navigator.clipboard.writeText(embedCode);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  if (!isClient) return null;

  return (
    <div className="flex flex-col min-h-screen font-inter bg-gradient-to-br from-indigo-50 via-purple-50 to-pink-50 items-center justify-center py-10 px-4">
      <div className="w-full max-w-6xl bg-white/80 backdrop-blur-[30px] saturate-[210%] rounded-[24px] shadow-sm border border-gray-100 flex flex-col lg:flex-row gap-8">
        <div className="flex-1 p-8">
          <div className="flex items-center gap-2 mb-6">
            <Link href="/dashboard" className="text-gray-500 hover:text-indigo-600 transition-colors">
              <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10 19l-7-7m0 0l7-7m-7 7h18" /></svg>
            </Link>
            <h1 className="text-3xl font-bold font-outfit text-gray-900">Viral Giveaway Generator 🎁</h1>
          </div>
          <p className="text-sm text-gray-600 mb-8 leading-relaxed">
            Create a highly viral giveaway landing page or embed widget. The "Refer a Friend" multiplier is proven to decrease customer acquisition cost by up to 40%.
          </p>

          <div className="space-y-5">
             <div>
                <label className="block text-sm font-semibold text-gray-700 mb-1">Giveaway Headline</label>
                <input
                  type="text"
                  value={title}
                  onChange={(e) => setTitle(e.target.value)}
                  className="w-full px-4 py-3 bg-white border border-gray-200 rounded-xl focus:outline-none focus:ring-2 focus:ring-indigo-500 text-gray-900 font-medium"
                />
             </div>
             <div>
                <label className="block text-sm font-semibold text-gray-700 mb-1">Prize Description</label>
                <input
                  type="text"
                  value={prize}
                  onChange={(e) => setPrize(e.target.value)}
                  className="w-full px-4 py-3 bg-white border border-gray-200 rounded-xl focus:outline-none focus:ring-2 focus:ring-indigo-500 text-gray-900 font-medium"
                />
             </div>
             <div>
                <label className="block text-sm font-semibold text-gray-700 mb-1">Theme</label>
                <select
                  value={theme}
                  onChange={(e) => setTheme(e.target.value as any)}
                  className="w-full px-4 py-3 bg-white border border-gray-200 rounded-xl focus:outline-none focus:ring-2 focus:ring-indigo-500 text-gray-900 font-medium"
                >
                  <option value="light">Light</option>
                  <option value="dark">Dark</option>
                </select>
             </div>

             <div className="flex items-center gap-3 mt-6 pt-6 border-t border-gray-100">
                <input
                    type="checkbox"
                    id="removeBranding"
                    checked={hasPro}
                    onChange={handleRemoveBranding}
                    className="w-5 h-5 text-indigo-600 border-gray-300 rounded focus:ring-indigo-500"
                />
                <label htmlFor="removeBranding" className="text-sm font-medium text-gray-700 flex items-center gap-2 cursor-pointer">
                    Remove "Powered by OHC" Badge
                    {!hasPro && <span className="bg-gradient-to-r from-amber-400 to-orange-500 text-white text-[10px] font-bold px-2 py-0.5 rounded uppercase tracking-wider shadow-sm">PRO</span>}
                </label>
             </div>
          </div>

          <div className="mt-8 bg-gray-900 text-gray-300 p-4 rounded-xl font-mono text-xs overflow-x-auto mb-4 border border-gray-800">
             <pre>{embedCode}</pre>
          </div>
          <button
             onClick={handleCopy}
             className={`w-full py-4 rounded-xl text-sm font-bold transition-all shadow-md ${copied ? 'bg-green-500 text-white shadow-green-500/30' : 'bg-indigo-600 text-white hover:bg-indigo-700 shadow-indigo-600/30 active:scale-[0.98]'}`}
          >
             {copied ? 'Copied to Clipboard! 🎉' : 'Copy Embed Code'}
          </button>
        </div>

        <div className="flex-1 flex flex-col p-8 bg-gray-50/50 rounded-r-[24px] border-l border-white/50">
           <h2 className="text-xl font-semibold font-outfit text-gray-900 mb-4">Live Preview</h2>
           <div className={`flex-1 rounded-2xl shadow-xl relative overflow-hidden flex flex-col transition-colors duration-300 ${theme === 'dark' ? 'bg-[#111827]' : 'bg-white'}`} style={{ minHeight: '600px' }}>

              <div className="p-8 flex flex-col items-center text-center flex-1">
                 <div className="w-20 h-20 bg-gradient-to-br from-pink-500 to-purple-600 rounded-full flex items-center justify-center text-4xl shadow-lg shadow-pink-500/30 mb-6">
                   🎉
                 </div>
                 <h3 className={`text-3xl font-bold font-outfit mb-4 leading-tight ${theme === 'dark' ? 'text-white' : 'text-gray-900'}`}>
                   {title}
                 </h3>
                 <p className={`text-lg font-medium mb-8 ${theme === 'dark' ? 'text-pink-400' : 'text-pink-600'}`}>
                   Prize: {prize}
                 </p>

                 <div className="w-full space-y-4 max-w-sm mx-auto">
                    <input
                      type="email"
                      placeholder="Enter your email to join"
                      className={`w-full px-4 py-3 rounded-xl border focus:outline-none focus:ring-2 focus:ring-pink-500 ${theme === 'dark' ? 'bg-gray-800 border-gray-700 text-white' : 'bg-gray-50 border-gray-200 text-gray-900'}`}
                      disabled
                    />
                    <button className="w-full py-4 bg-gradient-to-r from-pink-500 to-purple-600 hover:from-pink-600 hover:to-purple-700 text-white font-bold rounded-xl shadow-lg shadow-pink-500/30 transition-all text-lg">
                      Enter to Win
                    </button>
                 </div>

                 <div className={`mt-10 p-4 rounded-xl border border-dashed ${theme === 'dark' ? 'bg-gray-800/50 border-gray-600' : 'bg-gray-50 border-gray-300'} w-full max-w-sm mx-auto`}>
                    <p className={`text-sm font-bold flex items-center justify-center gap-2 ${theme === 'dark' ? 'text-gray-300' : 'text-gray-700'}`}>
                       <span className="text-xl">🚀</span> Viral Multiplier
                    </p>
                    <p className={`text-xs mt-2 ${theme === 'dark' ? 'text-gray-400' : 'text-gray-500'}`}>
                       Get <strong>+3 bonus entries</strong> for every friend who enters using your unique link!
                    </p>
                 </div>
              </div>

              {!hasPro && (
                <div className={`py-3 text-center border-t ${theme === 'dark' ? 'border-gray-800 bg-gray-900' : 'border-gray-100 bg-gray-50'}`}>
                   <span className={`text-xs font-semibold tracking-wide ${theme === 'dark' ? 'text-gray-500' : 'text-gray-400'}`}>
                     ⚡ Powered by OHC
                   </span>
                </div>
              )}
           </div>
        </div>
      </div>

      {showPaywall && (
        <div className="fixed inset-0 bg-black/60 z-[60] flex items-center justify-center p-4 backdrop-blur-sm animate-in fade-in">
          <div className="bg-white rounded-3xl max-w-md p-8 shadow-2xl relative overflow-hidden font-inter border border-blue-100 text-center animate-in zoom-in-95">
             <div className="absolute top-0 right-0 w-32 h-32 bg-gradient-to-bl from-blue-50 to-transparent rounded-bl-full -z-10"></div>
             <div className="flex justify-end mb-2">
               <button
                 aria-label="Close paywall"
                 onClick={() => setShowPaywall(false)}
                 className="text-gray-400 hover:text-gray-600 rounded-full hover:bg-gray-100 transition-colors w-8 h-8 flex items-center justify-center"
               >
                 <span className="text-xl leading-none">&times;</span>
               </button>
             </div>
             <div className="w-16 h-16 bg-gradient-to-br from-amber-400 to-orange-500 rounded-2xl flex items-center justify-center text-3xl shadow-lg mx-auto mb-6 text-white font-bold shadow-orange-500/30">
               PRO
             </div>
             <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-3">Make it 100% Yours</h2>
             <p className="text-gray-600 mb-6 text-sm leading-relaxed">
               Upgrade to Pro to remove the "Powered by OHC" watermark and unlock full white-label customization for all growth widgets.
             </p>
             <button
               onClick={() => { setShowPaywall(false); window.location.href = '/pricing'; }}
               className="w-full py-4 rounded-xl font-bold text-white mb-4 transition-all shadow-lg hover:scale-[1.02] active:scale-[0.98]"
               style={{ background: 'linear-gradient(135deg, #0f172a 0%, #334155 100%)' }}
             >
               Upgrade to Pro
             </button>
             <button
               onClick={() => setShowPaywall(false)}
               className="mt-2 text-gray-500 hover:text-gray-700 font-medium text-sm w-full py-2"
             >
               Keep Branding
             </button>
          </div>
        </div>
      )}
    </div>
  );
}