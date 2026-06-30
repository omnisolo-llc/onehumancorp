"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';

export default function GiveawayGeneratorPage() {
  const router = useRouter();
  const [title, setTitle] = useState('');
  const [description, setDescription] = useState('');
  const [tenant, setTenant] = useState('DEFAULT');
  const [giveawayLink, setGiveawayLink] = useState('');
  const [isGenerating, setIsGenerating] = useState(false);
  const [hasPro, setHasPro] = useState(false);
  const [showSoftPaywall, setShowSoftPaywall] = useState(false);
  const [copied, setCopied] = useState(false);

  useEffect(() => {
    if (typeof window !== 'undefined') {
      const storedTenant = localStorage.getItem('tenant_id') || localStorage.getItem('tenant') || 'my-store';
      setTenant(storedTenant);
      setHasPro(localStorage.getItem('has_pro') === 'true');

      const checkStorage = () => {
        setHasPro(localStorage.getItem('has_pro') === 'true');
      };
      window.addEventListener('storage', checkStorage);
      return () => window.removeEventListener('storage', checkStorage);
    }
  }, []);

  const generateLink = () => {
    setIsGenerating(true);
    const origin = typeof window !== 'undefined' ? window.location.origin : 'https://ohc.app';
    const link = `${origin}/giveaway/enter?tenant=${encodeURIComponent(tenant)}&title=${encodeURIComponent(title || 'Enter our Giveaway!')}&description=${encodeURIComponent(description)}`;
    setGiveawayLink(link);
    setIsGenerating(false);
  };

  const handleGenerate = () => {
    if (!hasPro) {
      setShowSoftPaywall(true);
      return;
    }
    generateLink();
  };

  const claimTrialExtension = () => {
    const referralUrl = `${window.location.origin}/onboarding?ref=${tenant}`;
    window.open(`https://twitter.com/intent/tweet?text=${encodeURIComponent('I just launched a viral giveaway for my business on One Human Corp! Start your own business today: ' + referralUrl)}`, '_blank');
    if (typeof localStorage !== 'undefined') {
        localStorage.setItem('has_pro', 'true');
    }
    setHasPro(true);
    setShowSoftPaywall(false);
    generateLink();
  };

  return (
    <div className="flex flex-col min-h-screen font-inter" style={{ backgroundColor: '#F5F5F7' }}>
      {/* Header */}
      <header className="px-6 py-4 flex items-center justify-between border-b" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', borderBottom: '1px solid rgba(255, 255, 255, 0.4)', position: 'sticky', top: 0, zIndex: 50 }}>
         <h1 className="text-2xl font-bold font-outfit" style={{ color: '#1D1D1F', letterSpacing: '-0.02em' }}>Viral Giveaway Generator 🎁</h1>
         <div className="flex items-center gap-3">
             <button onClick={() => router.push('/dashboard')} className="px-4 py-2 bg-gray-200 rounded-md text-sm font-medium hover:bg-gray-300 transition-colors">
               Back to Dashboard
             </button>
         </div>
      </header>

      <main className="p-6 md:p-8 flex-1 max-w-5xl mx-auto w-full flex flex-col md:flex-row gap-8">

        {/* Campaign Settings */}
        <section className="w-full md:w-1/2 flex flex-col gap-6">
          <div className="p-6 shadow-md" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)', borderRadius: '16px' }}>
            <div className="flex items-center gap-4 mb-4">
              <h2 className="text-xl font-semibold font-outfit m-0" style={{ color: '#1D1D1F' }}>Giveaway Details</h2>
              <div className="flex items-center gap-2 px-3 py-1 bg-yellow-50 rounded-full border border-yellow-100">
                  <span className="text-xs font-medium text-yellow-600">Pro Feature</span>
              </div>
            </div>

            <div className="flex flex-col gap-4">
              <div>
                <label htmlFor="giveaway-title" className="block text-sm font-medium text-gray-700 mb-1">Prize / Title</label>
                <input
                  id="giveaway-title"
                  type="text"
                  value={title}
                  onChange={(e) => setTitle(e.target.value)}
                  placeholder="e.g. Win a $100 Gift Card!"
                  className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-purple-500"
                />
              </div>

              <div>
                <label htmlFor="giveaway-desc" className="block text-sm font-medium text-gray-700 mb-1">Description</label>
                <textarea
                  id="giveaway-desc"
                  rows={3}
                  value={description}
                  onChange={(e) => setDescription(e.target.value)}
                  placeholder="e.g. Enter your email below to win. Get 3 extra entries if you share on social media!"
                  className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-purple-500 resize-none"
                />
              </div>

              <button
                onClick={handleGenerate}
                disabled={!title || isGenerating}
                className={`w-full py-3 mt-4 text-white font-semibold rounded-xl shadow-lg transition-all ${(!title || isGenerating) ? 'bg-purple-400 cursor-not-allowed' : 'bg-purple-600 hover:bg-purple-700 hover:shadow-xl hover:-translate-y-0.5 active:translate-y-0'}`}
              >
                {isGenerating ? 'Generating...' : 'Generate Giveaway Link'}
              </button>
            </div>
          </div>

          {giveawayLink && (
            <div className="p-6 shadow-md bg-white border border-green-200">
              <h3 className="text-lg font-bold font-outfit text-gray-900 mb-2 flex items-center gap-2">
                <span className="text-[#34C759]">✅</span> Link Ready!
              </h3>
              <p className="text-sm text-gray-600 mb-4">Share this link with your audience to start capturing emails.</p>

              <div className="flex gap-2">
                <input
                  type="text"
                  readOnly
                  value={giveawayLink}
                  className="flex-1 px-3 py-2 bg-gray-50 border border-gray-200 rounded-lg text-sm text-gray-700 focus:outline-none"
                />
                <button
                  onClick={() => {
                    navigator.clipboard.writeText(giveawayLink);
                    setCopied(true);
                    setTimeout(() => setCopied(false), 2000);
                  }}
                  className={`px-4 py-2 rounded-lg text-sm font-semibold transition-all ${copied ? 'bg-green-100 text-green-700' : 'bg-gray-900 text-white hover:bg-black'}`}
                >
                  {copied ? 'Copied!' : 'Copy'}
                </button>
              </div>
            </div>
          )}
        </section>

        {/* Live Preview */}
        <section className="w-full md:w-1/2 flex justify-center items-start">
             <div className="w-full max-w-sm bg-white rounded-3xl shadow-2xl overflow-hidden relative border border-gray-200 flex flex-col items-center">
                 <div className="w-full h-32 bg-gradient-to-r from-purple-500 to-pink-500 relative flex items-center justify-center">
                     <span className="text-5xl drop-shadow-md">🎁</span>
                 </div>

                 <div className="w-full p-8 flex flex-col items-center text-center">
                     <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2">
                         {title || 'Win a $50 Gift Card!'}
                     </h2>
                     <p className="text-sm text-gray-600 mb-6 leading-relaxed">
                         {description || 'Enter your email below for a chance to win. Plus, get bonus entries when you share with friends!'}
                     </p>

                     <div className="w-full space-y-3">
                         <input
                            type="email"
                            placeholder="Enter your email"
                            className="w-full px-4 py-3 bg-gray-50 border border-gray-200 rounded-xl text-sm"
                            disabled
                         />
                         <button disabled className="w-full py-3 bg-purple-600 text-white font-bold rounded-xl shadow-md">
                            Enter Giveaway
                         </button>
                     </div>

                     <div className="mt-8">
                        <a href={`/onboarding?ref=${tenant}`} target="_blank" rel="noopener noreferrer" className="text-xs font-semibold text-gray-400 uppercase tracking-widest hover:text-gray-600 transition-colors">⚡ Powered by OHC</a>
                     </div>
                 </div>
             </div>
        </section>
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

            <div className="text-5xl mb-4">✨</div>
            <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-3">Upgrade to Pro</h2>
            <p className="text-gray-600 mb-6 text-sm leading-relaxed">
              Viral Giveaways are a Pro feature. Upgrade to our Pro plan to generate viral loops and capture emails on autopilot.
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
      `}} />
    </div>
  );
}
