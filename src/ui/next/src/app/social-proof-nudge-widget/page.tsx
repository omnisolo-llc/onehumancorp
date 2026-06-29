"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';

export default function SocialProofNudgeWidgetPage() {
  const router = useRouter();
  const [tenant, setTenant] = useState('my-business');
  const [theme, setTheme] = useState<'light' | 'dark'>('light');
  const [position, setPosition] = useState<'bottom-left' | 'bottom-right'>('bottom-left');
  const [message, setMessage] = useState('Maya just booked a lesson');
  const [timeAgo, setTimeAgo] = useState('2 mins ago');
  const [copied, setCopied] = useState(false);
  const [hasPro, setHasPro] = useState(false);
  const [showPaywall, setShowPaywall] = useState(false);
  const [isClient, setIsClient] = useState(false);
  const [removeBranding, setRemoveBranding] = useState(false);
  const [hasSharedToUnlock, setHasSharedToUnlock] = useState(false);

  useEffect(() => {
    setIsClient(true);
    if (typeof localStorage !== 'undefined') {
      const storedTenant = localStorage.getItem('tenant') || 'my-business';
      setTenant(storedTenant);
      const isPro = localStorage.getItem('has_pro') === 'true';
      setHasPro(isPro);
      const hasShared = localStorage.getItem('ohc_social_proof_shared') === 'true';
      setHasSharedToUnlock(hasShared);

      if (hasShared || isPro) {
        setRemoveBranding(true);
      }
    }
    document.title = "Social Proof Widget | OHC";
  }, []);

  const handleRemoveBranding = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (!hasPro && !hasSharedToUnlock) {
      e.preventDefault();
      setShowPaywall(true);
    } else {
      setRemoveBranding(e.target.checked);
    }
  };

  const claimTrialExtension = () => {
    const referralUrl = `${window.location.origin}/onboarding?ref=${tenant}`;
    window.open(`https://twitter.com/intent/tweet?text=${encodeURIComponent('I just unlocked powerful AI tools for my business on One Human Corp! Start your own business today: ' + referralUrl)}`, '_blank');
    if (typeof localStorage !== 'undefined') {
        localStorage.setItem('ohc_social_proof_shared', 'true');
    }
    setHasSharedToUnlock(true);
    setRemoveBranding(true);
    setShowPaywall(false);
  };

  const embedUrl = `https://ohc.app/api/v1/growth/social-proof/embed?tenant=${tenant}&theme=${theme}&message=${encodeURIComponent(message)}&timeAgo=${encodeURIComponent(timeAgo)}&branding=${!removeBranding}`;
  const scriptCode = `<script src="${embedUrl}&format=js" async defer></script>`;

  const handleCopy = () => {
    navigator.clipboard.writeText(scriptCode);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  if (!isClient) return null;

  return (
    <div className="flex flex-col min-h-screen font-inter bg-gradient-to-br from-indigo-50 via-purple-50 to-pink-50 items-center justify-center py-10 px-4">
      <div className="w-full max-w-4xl bg-white/80 backdrop-blur-xl rounded-[24px] shadow-sm border border-gray-100 flex flex-col md:flex-row gap-8">
        <div className="flex-1 p-8">
          <div className="flex items-center gap-2 mb-6">
             <button onClick={() => router.push('/dashboard')} className="text-gray-500 hover:text-indigo-600 font-medium text-sm transition-colors flex items-center gap-1">
               <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10 19l-7-7m0 0l7-7m-7 7h18" /></svg>
               Back
             </button>
          </div>
          <h1 className="text-3xl font-bold font-outfit text-gray-900 mb-6 flex items-center gap-2">
            Social Proof Nudge 📣
          </h1>
          <p className="text-gray-600 text-sm mb-6">
            Boost conversions by showing live activity on your store. Build trust instantly.
          </p>

          <div className="space-y-4">
             <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">Nudge Message</label>
                <input type="text" value={message} onChange={(e) => setMessage(e.target.value)} className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500" placeholder="e.g. Someone just bought a cake!" />
             </div>
             <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">Time Ago text</label>
                <input type="text" value={timeAgo} onChange={(e) => setTimeAgo(e.target.value)} className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500" placeholder="e.g. 5 mins ago" />
             </div>
             <div className="grid grid-cols-2 gap-4">
                 <div>
                    <label className="block text-sm font-medium text-gray-700 mb-1">Theme</label>
                    <select value={theme} onChange={(e) => setTheme(e.target.value as any)} className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500 bg-white">
                      <option value="light">Light</option>
                      <option value="dark">Dark</option>
                    </select>
                 </div>
                 <div>
                    <label className="block text-sm font-medium text-gray-700 mb-1">Position</label>
                    <select value={position} onChange={(e) => setPosition(e.target.value as any)} className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500 bg-white">
                      <option value="bottom-left">Bottom Left</option>
                      <option value="bottom-right">Bottom Right</option>
                    </select>
                 </div>
             </div>
             <div className="flex items-center gap-2 mt-4 pt-4 border-t border-gray-200">
                <input
                    type="checkbox"
                    id="removeBranding"
                    checked={removeBranding}
                    onChange={handleRemoveBranding}
                    className="w-4 h-4 text-indigo-600 border-gray-300 rounded focus:ring-indigo-500"
                />
                <label htmlFor="removeBranding" className="text-sm font-medium text-gray-700 flex items-center gap-2">
                    Remove "Powered by OHC" Badge
                    {!hasPro && !hasSharedToUnlock && <span className="bg-yellow-100 text-yellow-800 text-[10px] font-bold px-2 py-0.5 rounded uppercase tracking-wider">PRO</span>}
                </label>
             </div>
          </div>

          <div className="mt-8 bg-gray-900 text-gray-300 p-4 rounded-xl font-mono text-xs overflow-x-auto mb-4">
             <pre>{scriptCode}</pre>
          </div>
          <button
             onClick={handleCopy}
             className={`w-full py-3 rounded-lg text-sm font-semibold transition-all ${copied ? 'bg-green-100 text-green-700' : 'bg-indigo-600 text-white hover:bg-indigo-700'}`}
          >
             {copied ? 'Copied to Clipboard!' : 'Copy Integration Script'}
          </button>
        </div>

        <div className="flex-1 flex flex-col p-8 bg-gray-50/50 rounded-r-[24px]">
           <h2 className="text-xl font-semibold font-outfit text-gray-900 mb-4">Live Preview</h2>
           <div className="flex-1 bg-gray-200 rounded-2xl shadow-inner border border-gray-300 relative overflow-hidden flex items-center justify-center min-h-[400px]">

              {/* Simulated Website Content */}
              <div className="absolute inset-0 p-8 opacity-20 pointer-events-none">
                 <div className="w-3/4 h-8 bg-gray-400 rounded mb-6"></div>
                 <div className="w-full h-4 bg-gray-400 rounded mb-3"></div>
                 <div className="w-5/6 h-4 bg-gray-400 rounded mb-3"></div>
                 <div className="w-full h-4 bg-gray-400 rounded mb-3"></div>
                 <div className="w-2/3 h-4 bg-gray-400 rounded mb-8"></div>

                 <div className="grid grid-cols-2 gap-4">
                     <div className="w-full h-32 bg-gray-400 rounded"></div>
                     <div className="w-full h-32 bg-gray-400 rounded"></div>
                 </div>
              </div>

              {/* The actual preview widget */}
              <div className={`absolute ${position === 'bottom-left' ? 'bottom-6 left-6' : 'bottom-6 right-6'} z-10 animate-bounce-in`} style={{ animation: 'bounceIn 0.6s cubic-bezier(0.175, 0.885, 0.32, 1.275)' }}>
                  <div className={`flex flex-col p-3.5 sm:p-4 rounded-2xl shadow-2xl border min-w-[280px] max-w-[320px] transition-all duration-300 ${theme === 'dark' ? 'bg-[#1c1c1e] border-white/10 text-white' : 'bg-white border-gray-100 text-gray-900'}`}>
                      <div className="flex items-start gap-3">
                          <div className={`w-10 h-10 rounded-full flex-shrink-0 flex items-center justify-center text-lg ${theme === 'dark' ? 'bg-indigo-900/50 text-indigo-400' : 'bg-indigo-100 text-indigo-600'}`}>
                              🎉
                          </div>
                          <div className="flex-1 pt-0.5">
                              <p className="font-semibold text-[13px] sm:text-sm leading-tight mb-1" style={{ color: theme === 'dark' ? '#fff' : '#111827' }}>
                                  {message || 'Someone took action!'}
                              </p>
                              <p className="text-[11px] sm:text-xs" style={{ color: theme === 'dark' ? '#9ca3af' : '#6b7280' }}>
                                  {timeAgo || 'Just now'}
                              </p>
                          </div>
                      </div>

                      {!removeBranding && (
                          <div className={`mt-3 pt-2 text-right border-t ${theme === 'dark' ? 'border-gray-800' : 'border-gray-100'}`}>
                              <span className={`text-[10px] font-medium ${theme === 'dark' ? 'text-gray-500' : 'text-gray-400'}`}>
                                  ⚡ Powered by OHC
                              </span>
                          </div>
                      )}
                  </div>
              </div>
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
               Make the Social Proof Widget 100% yours. Upgrade to Pro to remove the "Powered by OHC" watermark.
             </p>

             <button
                onClick={claimTrialExtension}
                className="w-full py-4 px-6 bg-[#000] text-white font-semibold rounded-xl mb-3 shadow-md hover:shadow-lg transition-all flex justify-center items-center gap-2 group"
             >
                <svg className="w-5 h-5 text-white group-hover:scale-110 transition-transform" fill="currentColor" viewBox="0 0 24 24"><path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.008 5.94H5.078z"/></svg>
                Share on X to Unlock Free
             </button>

             <button
               onClick={() => { setShowPaywall(false); window.location.href = '/pricing'; }}
               className="w-full py-3 rounded-xl font-bold text-gray-700 bg-gray-100 mb-4 transition-all hover:bg-gray-200"
             >
               View Pro Plans
             </button>
          </div>
        </div>
      )}

      <style dangerouslySetInnerHTML={{__html: `
        @keyframes bounceIn {
            0% { opacity: 0; transform: scale(0.3) translateY(20px); }
            50% { opacity: 1; transform: scale(1.05) translateY(-5px); }
            70% { transform: scale(0.9) translateY(5px); }
            100% { opacity: 1; transform: scale(1) translateY(0); }
        }
        .animate-bounce-in {
            animation: bounceIn 0.6s cubic-bezier(0.175, 0.885, 0.32, 1.275);
        }
      `}} />
    </div>
  );
}
