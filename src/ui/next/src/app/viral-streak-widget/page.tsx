"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';
import { PoweredByOHC } from "../components/PoweredByOHC";

export default function ViralStreakWidgetPage() {
  const router = useRouter();
  const [tenant, setTenant] = useState('my-store');
  const [theme, setTheme] = useState<'light' | 'dark'>('light');
  const [title, setTitle] = useState('Daily Login Streak');
  const [goal, setGoal] = useState('7');
  const [reward, setReward] = useState('Free Coffee');
  const [copied, setCopied] = useState(false);
  const [hasPro, setHasPro] = useState(false);
  const [showPaywall, setShowPaywall] = useState(false);
  const [isClient, setIsClient] = useState(false);
  const [showModal, setShowModal] = useState(false);

  useEffect(() => {
    setIsClient(true);
    if (typeof localStorage !== 'undefined') {
      const storedTenant = localStorage.getItem('tenant') || 'my-store';
      setTenant(storedTenant);
      setHasPro(localStorage.getItem('has_pro') === 'true');
    }
    document.title = "Viral Streak Widget | OHC";
  }, []);

  const handleRemoveBranding = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (!hasPro) {
      e.preventDefault();
      setShowPaywall(true);
    }
  };

  const embedUrl = `https://ohc.app/api/v1/growth/viral-streak/embed?tenant=${tenant}&theme=${theme}&title=${encodeURIComponent(title)}&goal=${encodeURIComponent(goal)}&reward=${encodeURIComponent(reward)}&branding=${!hasPro}`;
  const embedCode = `<iframe src="${embedUrl}" width="100%" height="450" frameborder="0" scrolling="no" style="border:none; overflow:hidden; border-radius:16px;"></iframe>`;

  const handleCopy = () => {
    navigator.clipboard.writeText(embedCode);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  if (!isClient) return null;

  return (
    <div className="flex flex-col min-h-screen font-inter bg-gradient-to-br from-indigo-50 via-purple-50 to-pink-50 items-center justify-center py-10 px-4">
      <div className="w-full max-w-5xl bg-white/80 backdrop-blur-xl rounded-[24px] shadow-sm border border-gray-100 flex flex-col md:flex-row gap-8">

        {/* Configuration Panel */}
        <div className="flex-1 p-8">
          <button
            onClick={() => router.push('/dashboard')}
            className="mb-6 px-4 py-2 bg-gray-100 text-gray-700 text-sm font-medium hover:bg-gray-200 rounded-lg transition-colors inline-flex items-center gap-2"
          >
            &larr; Back to Dashboard
          </button>

          <h1 className="text-3xl font-bold font-outfit text-gray-900 mb-6">Viral Streak Widget 📅</h1>
          <p className="text-sm text-gray-600 mb-6">Build customer habits by rewarding consecutive daily visits or actions. Watch retention soar.</p>

          <div className="space-y-4">
             <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">Streak Title</label>
                <input type="text" value={title} onChange={(e) => setTitle(e.target.value)} className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500 bg-white/80" />
             </div>
             <div className="grid grid-cols-2 gap-4">
                 <div>
                    <label className="block text-sm font-medium text-gray-700 mb-1">Streak Goal (Days)</label>
                    <input type="number" value={goal} onChange={(e) => setGoal(e.target.value)} min="2" max="30" className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500 bg-white/80" />
                 </div>
                 <div>
                    <label className="block text-sm font-medium text-gray-700 mb-1">Reward</label>
                    <input type="text" value={reward} onChange={(e) => setReward(e.target.value)} className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500 bg-white/80" />
                 </div>
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
                    onChange={handleRemoveBranding}
                    className="w-4 h-4 text-indigo-600 border-gray-300 rounded focus:ring-indigo-500"
                />
                <label htmlFor="removeBranding" className="text-sm font-medium text-gray-700 flex items-center gap-2 cursor-pointer">
                    Remove "Powered by OHC" Badge
                    {!hasPro && <span className="bg-yellow-100 text-yellow-800 text-[10px] font-bold px-2 py-0.5 rounded uppercase tracking-wider">PRO</span>}
                </label>
             </div>
          </div>

          <div className="mt-8">
             <button
                onClick={() => setShowModal(true)}
                className="w-full py-3 bg-indigo-600 hover:bg-indigo-700 text-white font-medium rounded-xl transition-colors shadow-md"
             >
                Get Embed Code
             </button>
          </div>
        </div>

        {/* Live Preview */}
        <div className="flex-1 flex flex-col p-8 bg-gray-50/50 rounded-r-[24px]">
           <h2 className="text-xl font-semibold font-outfit text-gray-900 mb-4">Live Preview</h2>
           <div className="flex-1 rounded-2xl shadow-inner border-2 border-dashed border-gray-300 relative overflow-hidden flex items-center justify-center p-2 min-h-[450px]">

              <div className={`w-full max-w-sm p-6 rounded-2xl shadow-lg border relative z-10 transition-colors ${theme === 'dark' ? 'bg-gray-900 border-gray-700 text-white' : 'bg-white border-gray-200 text-gray-900'}`}>
                  <div className="text-center mb-6">
                      <div className="inline-block p-4 rounded-full bg-gradient-to-tr from-amber-400 to-orange-500 text-white shadow-inner mb-4">
                          <svg className="w-8 h-8" fill="currentColor" viewBox="0 0 20 20"><path fillRule="evenodd" d="M12.395 2.553a1 1 0 00-1.45-.385c-.345.23-.614.558-.822.88-.214.33-.403.713-.57 1.116-.334.804-.614 1.768-.84 2.734a31.365 31.365 0 00-.613 3.58 2.64 2.64 0 01-.945-1.067c-.328-.68-.398-1.534-.398-2.654A1 1 0 005.05 6.05 6.981 6.981 0 003 11a7 7 0 1011.95-4.95c-.592-.591-.98-.985-1.348-1.467-.363-.476-.724-1.063-1.207-2.03zM12.12 15.12A3 3 0 017 13s.879.5 2.5.5c0-1 .5-4 1.25-4.5.5 1 .786 1.293 1.371 1.879A2.99 2.99 0 0113 13a2.99 2.99 0 01-.879 2.121z" clipRule="evenodd"></path></svg>
                      </div>
                      <h3 className="text-2xl font-bold font-outfit">{title}</h3>
                      <p className={`text-sm mt-1 ${theme === 'dark' ? 'text-gray-400' : 'text-gray-500'}`}>Hit {goal} days to unlock {reward}</p>
                  </div>

                  <div className="flex justify-between items-center mb-6">
                     {Array.from({ length: Math.min(parseInt(goal) || 7, 7) }).map((_, i) => (
                         <div key={i} className="flex flex-col items-center gap-1">
                             <div className={`w-8 h-8 rounded-full flex items-center justify-center text-xs font-bold transition-all ${i < 3 ? 'bg-orange-500 text-white shadow-md shadow-orange-500/30' : i === 3 ? 'border-2 border-orange-500 text-orange-500' : theme === 'dark' ? 'bg-gray-800 text-gray-500' : 'bg-gray-100 text-gray-400'}`}>
                                 {i < 3 ? '✓' : i + 1}
                             </div>
                             {i + 1 == parseInt(goal) && <div className="text-[10px] uppercase font-bold text-orange-500 mt-1">Reward</div>}
                         </div>
                     ))}
                  </div>

                  <button className="w-full py-3 bg-orange-500 hover:bg-orange-600 text-white font-bold rounded-xl shadow-md transition-colors">
                      Claim Today's Streak
                  </button>

                  {!hasPro && (
                      <div className="mt-6 text-center">
                          <PoweredByOHC tenantId={tenant} />
                      </div>
                  )}
              </div>
           </div>
        </div>
      </div>

      {/* Embed Modal */}
      {showModal && (
        <div className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/40 backdrop-blur-[30px] saturate-[210%]">
            <div className="p-8 max-w-xl w-full shadow-2xl relative animate-in fade-in bg-white rounded-2xl">
                <button
                    aria-label="Close embed modal"
                    onClick={() => setShowModal(false)}
                    className="absolute top-6 right-6 text-gray-400 hover:text-gray-600 transition-colors"
                >
                    <svg className="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
                    </svg>
                </button>

                <h2 className="text-2xl font-bold font-outfit mb-2 text-gray-900">Embed Streak Widget</h2>
                <p className="text-gray-600 mb-6 text-sm">Copy and paste this HTML snippet into your website to embed the widget.</p>

                <div className="relative group">
                    <textarea
                        readOnly
                        value={embedCode}
                        className="w-full h-32 p-4 bg-gray-50 border border-gray-200 font-mono text-sm text-gray-800 resize-none focus:outline-none focus:ring-2 focus:ring-indigo-500 transition-all rounded-xl"
                    />
                </div>

                <div className="mt-6 flex flex-col sm:flex-row gap-3">
                    <button
                        onClick={handleCopy}
                        className="flex-1 py-3 bg-indigo-600 hover:bg-indigo-700 text-white font-medium rounded-xl transition-colors shadow-sm flex items-center justify-center gap-2"
                    >
                        {copied ? 'Copied!' : 'Copy Code'}
                    </button>
                    <button
                        onClick={() => setShowModal(false)}
                        className="flex-1 py-3 bg-gray-100 hover:bg-gray-200 text-gray-800 font-medium rounded-xl transition-colors"
                    >
                        Close
                    </button>
                </div>
            </div>
        </div>
      )}

      {/* Soft Paywall Modal */}
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
               Make the Streak Widget 100% yours. Upgrade to Pro to remove the "Powered by OHC" watermark.
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
