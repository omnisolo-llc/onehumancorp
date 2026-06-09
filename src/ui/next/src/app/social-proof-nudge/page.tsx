"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';

export default function SocialProofNudgePage() {
  const router = useRouter();
  const [productName, setProductName] = useState('');
  const [customerLocation, setCustomerLocation] = useState('');
  const [timeAgo, setTimeAgo] = useState('just now');
  const [theme, setTheme] = useState<'light' | 'dark'>('light');
  const [hasPro, setHasPro] = useState(false);
  const [showPaywall, setShowPaywall] = useState(false);
  const [copied, setCopied] = useState(false);

  useEffect(() => {
    if (typeof window !== 'undefined') {
      setHasPro(localStorage.getItem('has_pro') === 'true');
    }
  }, []);

  const handleRemoveBranding = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (!hasPro) {
      e.preventDefault();
      setShowPaywall(true);
    }
  };

  const getEmbedCode = () => {
    return `<!-- Social Proof Nudge Widget -->\n<div id="ohc-social-proof" data-product="${productName || 'A product'}" data-location="${customerLocation || 'Someone'}" data-time="${timeAgo}" data-theme="${theme}" data-branding="${!hasPro}"></div>\n<script src="https://ohc.app/widgets/social-proof.js" async></script>\n${!hasPro ? '<!-- ⚡ Powered by OHC -->' : ''}`;
  };

  const getThemeStyles = () => {
    if (theme === 'dark') {
      return { background: '#1D1D1F', color: '#ffffff', borderColor: '#333333' };
    }
    return { background: '#ffffff', color: '#111827', borderColor: '#e5e7eb' };
  };

  const claimTrialExtension = () => {
    const tenant = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant_id') || 'DEFAULT' : 'DEFAULT';
    const referralUrl = `${window.location.origin}/onboarding?ref=${tenant}`;
    window.open(`https://twitter.com/intent/tweet?text=${encodeURIComponent('I just unlocked powerful AI tools for my business on One Human Corp! Start your own business today: ' + referralUrl)}`, '_blank');
    if (typeof localStorage !== 'undefined') {
        localStorage.setItem('has_pro', 'true');
    }
    setHasPro(true);
    setShowPaywall(false);
  };

  return (
    <div className="flex flex-col min-h-screen font-inter" style={{ backgroundColor: '#F5F5F7' }}>
      {/* Header */}
      <header className="px-6 py-4 flex items-center justify-between border-b" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', borderBottom: '1px solid rgba(255, 255, 255, 0.4)', position: 'sticky', top: 0, zIndex: 50 }}>
         <h1 className="text-2xl font-bold font-outfit" style={{ color: '#1D1D1F', letterSpacing: '-0.02em' }}>Social Proof Nudge 🚀</h1>
         <div className="flex items-center gap-3">
             <button onClick={() => router.push('/dashboard')} className="px-4 py-2 bg-gray-200 rounded-md text-sm font-medium hover:bg-gray-300 transition-colors">
               Back to Dashboard
             </button>
             <div className="w-8 h-8 rounded-full bg-gray-200 flex items-center justify-center text-sm font-bold text-gray-600">
                 AC
             </div>
         </div>
      </header>

      <main className="p-6 md:p-8 flex-1 max-w-5xl mx-auto w-full flex flex-col md:flex-row gap-8">

        {/* Editor Settings */}
        <section className="w-full md:w-1/2 flex flex-col gap-6">
            <div className="bg-gradient-to-r from-blue-50 to-indigo-50 border border-blue-100 rounded-2xl p-6 shadow-sm">
                <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2">Boost Sales with FOMO</h2>
                <p className="text-gray-600 text-sm">
                    Show visitors that others are buying right now. Stores using social proof nudges see up to a <strong>15% increase</strong> in conversion rates.
                </p>
            </div>

            <div className="p-6 shadow-md" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)', borderRadius: '16px' }}>
                <h2 className="text-xl font-semibold font-outfit mb-4" style={{ color: '#1D1D1F' }}>Widget Details</h2>
                <div className="flex flex-col gap-4">
                    <div>
                        <label className="block text-sm font-medium text-gray-700 mb-1">Example Product</label>
                        <input
                            type="text"
                            placeholder="e.g. Signature Coffee Blend"
                            value={productName}
                            onChange={(e) => setProductName(e.target.value)}
                            className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500"
                        />
                    </div>
                    <div>
                        <label className="block text-sm font-medium text-gray-700 mb-1">Example Location</label>
                        <input
                            type="text"
                            placeholder="e.g. Someone in London"
                            value={customerLocation}
                            onChange={(e) => setCustomerLocation(e.target.value)}
                            className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500"
                        />
                    </div>
                    <div>
                        <label className="block text-sm font-medium text-gray-700 mb-1">Time Display</label>
                        <select
                            value={timeAgo}
                            onChange={(e) => setTimeAgo(e.target.value)}
                            className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500 glassmorphism"
                        >
                            <option value="just now">Just now</option>
                            <option value="2 minutes ago">2 minutes ago</option>
                            <option value="1 hour ago">1 hour ago</option>
                            <option value="today">Today</option>
                        </select>
                    </div>
                    <div>
                        <label className="block text-sm font-medium text-gray-700 mb-2">Theme</label>
                        <div className="flex gap-2">
                            <button aria-label="Light theme" aria-pressed={theme === 'light'} onClick={() => setTheme('light')} className={`w-8 h-8 rounded-full border-2 ${theme === 'light' ? 'border-indigo-600' : 'border-gray-300'}`} style={{ background: '#ffffff' }}></button>
                            <button aria-label="Dark theme" aria-pressed={theme === 'dark'} onClick={() => setTheme('dark')} className={`w-8 h-8 rounded-full border-2 ${theme === 'dark' ? 'border-indigo-600' : 'border-gray-300'}`} style={{ background: '#1D1D1F' }}></button>
                        </div>
                    </div>
                    <div className="flex items-center gap-2 mt-2 pt-4 border-t border-gray-200">
                        <input
                            type="checkbox"
                            id="removeBranding"
                            checked={hasPro}
                            onChange={handleRemoveBranding}
                            className="w-4 h-4 text-indigo-600 border-gray-300 rounded focus:ring-indigo-500"
                        />
                        <label htmlFor="removeBranding" className="text-sm font-medium text-gray-700 flex items-center gap-2">
                            Remove "Powered by OHC" Badge
                            {!hasPro && <span className="bg-yellow-100 text-yellow-800 text-[10px] font-bold px-2 py-0.5 rounded uppercase tracking-wider">PRO</span>}
                        </label>
                    </div>
                </div>
            </div>

            <div className="p-6 shadow-md" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)', borderRadius: '16px' }}>
                <h2 className="text-xl font-semibold font-outfit mb-4" style={{ color: '#1D1D1F' }}>Embed on Your Site</h2>
                <div className="bg-gray-900 text-gray-300 p-4 rounded-xl font-mono text-xs overflow-x-auto mb-4">
                    <pre id="embed-code">
                        {getEmbedCode()}
                    </pre>
                </div>
                <button
                    onClick={() => {
                        navigator.clipboard.writeText(getEmbedCode());
                        setCopied(true);
                        setTimeout(() => setCopied(false), 2000);
                    }}
                    className={`w-full py-3 rounded-lg text-sm font-semibold transition-all ${copied ? 'bg-green-100 text-green-700' : 'bg-indigo-600 text-white hover:bg-indigo-700'}`}
                >
                    {copied ? 'Copied to Clipboard!' : 'Copy Embed Code'}
                </button>
            </div>
        </section>

        {/* Live Preview */}
        <section className="w-full md:w-1/2 flex flex-col gap-4">
             <h2 className="text-xl font-semibold font-outfit" style={{ color: '#1D1D1F' }}>Live Preview</h2>

             <div className="w-full h-[600px] bg-gray-100 rounded-2xl shadow-inner border-2 border-dashed border-gray-300 relative overflow-hidden flex items-end justify-start p-6">
                 {/* Decorative background to look like a website */}
                 <div className="absolute inset-0 opacity-10 pointer-events-none" style={{ backgroundImage: 'linear-gradient(45deg, #ccc 25%, transparent 25%, transparent 75%, #ccc 75%, #ccc), linear-gradient(45deg, #ccc 25%, transparent 25%, transparent 75%, #ccc 75%, #ccc)', backgroundSize: '20px 20px', backgroundPosition: '0 0, 10px 10px' }}></div>

                 {/* The actual widget preview */}
                 <div
                    className="z-10 flex items-center gap-4 p-4 rounded-xl shadow-2xl transition-all duration-300 animate-fade-in-up border w-full max-w-sm"
                    style={getThemeStyles()}
                 >
                     <div className="w-12 h-12 bg-indigo-100 rounded-lg flex items-center justify-center text-xl shrink-0">
                         🛍️
                     </div>
                     <div className="flex-1 min-w-0">
                         <p className="text-sm font-semibold truncate">
                             {customerLocation || 'Someone'} <span className="font-normal opacity-80">purchased</span>
                         </p>
                         <p className="text-sm font-bold truncate text-indigo-600 dark:text-indigo-400">
                             {productName || 'A product'}
                         </p>
                         <div className="flex items-center justify-between mt-1">
                             <p className="text-xs opacity-60 font-medium">
                                 {timeAgo}
                             </p>
                             <div className="flex items-center gap-1 opacity-70">
                                 <span className="text-[10px] uppercase font-bold tracking-widest text-green-500">Verified</span>
                                 <svg className="w-3 h-3 text-green-500" fill="currentColor" viewBox="0 0 20 20"><path fillRule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clipRule="evenodd"></path></svg>
                             </div>
                         </div>
                     </div>
                 </div>

                 {!hasPro && (
                     <div className="absolute bottom-2 left-6 z-10">
                         <a href="/onboarding?ref=social-proof-nudge" className="text-[10px] font-bold uppercase tracking-wider opacity-60 hover:opacity-100 transition-opacity drop-shadow-sm" style={{ color: theme === 'dark' ? '#fff' : '#000' }}>
                             ⚡ Powered by OHC
                         </a>
                     </div>
                 )}
             </div>
        </section>
      </main>

      {/* Soft Paywall Modal */}
      {showPaywall && (
        <div className="fixed inset-0 bg-black/60 z-[60] flex items-center justify-center p-4">
          <div className="app-card w-full max-w-md rounded-2xl p-8 shadow-2xl relative overflow-hidden font-inter border border-blue-100 text-center">
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
              Make the Social Proof Nudge 100% yours. Upgrade to Pro to remove the "Powered by OHC" watermark and unlock premium widget themes.
            </p>

            <button
              onClick={() => { setShowPaywall(false); window.location.href = '/pricing'; }}
              className="w-full py-4 rounded-xl font-bold text-white mb-4 transition-all shadow-md hover:shadow-lg hover:opacity-90"
              style={{ background: 'linear-gradient(135deg, #0066ff 0%, #3b82f6 100%)' }}
            >
              Upgrade to Pro
            </button>

            <div className="my-4 text-gray-400 font-medium text-sm">OR</div>

            <button
              onClick={claimTrialExtension}
              className="w-full py-3.5 rounded-xl font-bold transition-all shadow-sm bg-black text-white border-2 border-black hover:bg-gray-800 flex items-center justify-center gap-2"
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
        @keyframes fadeInUp { from { opacity: 0; transform: translateY(20px); } to { opacity: 1; transform: translateY(0); } }
        .animate-fade-in-up { animation: fadeInUp 0.5s ease-out forwards; }
      `}} />
    </div>
  );
}
