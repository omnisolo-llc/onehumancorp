"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';

export default function GiftCardsPage() {
  const router = useRouter();
  const [value, setValue] = useState('50');
  const [copied, setCopied] = useState(false);
  const [shareLink, setShareLink] = useState('');
  const [showShareModal, setShowShareModal] = useState(false);
  const [removeBranding, setRemoveBranding] = useState(false);
  const [tenantId, setTenantId] = useState('my-store');

  // Try to load tenant info on mount
  useEffect(() => {
    const tenant = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant') || 'my-store' : 'my-store';
    setTenantId(tenant);
    const origin = typeof window !== 'undefined' ? window.location.origin : 'https://ohc.app';
    setShareLink(`${origin}/gift-card?amount=${value}&ref=${tenant}`);
  }, [value]);

  const handleGenerate = () => {
    setShowShareModal(true);
  };

  const copyLink = () => {
    navigator.clipboard.writeText(shareLink);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div className="flex flex-col min-h-screen font-inter" style={{ backgroundColor: '#F5F5F7' }}>
      {/* Header */}
      <header className="px-6 py-4 flex items-center justify-between border-b" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', borderBottom: '1px solid rgba(255, 255, 255, 0.4)', position: 'sticky', top: 0, zIndex: 50 }}>
         <h1 className="text-2xl font-bold font-outfit" style={{ color: '#1D1D1F', letterSpacing: '-0.02em' }}>Gift Card Generator 🎁</h1>
         <div className="flex items-center gap-3">
             <button onClick={() => router.push('/dashboard')} className="px-4 py-2 bg-gray-200 rounded-lg text-sm font-medium hover:bg-gray-300 transition-colors">
               Back to Dashboard
             </button>
         </div>
      </header>

      <main className="flex-1 max-w-5xl mx-auto w-full p-6 flex flex-col md:flex-row gap-8">

        {/* Settings Panel */}
        <section className="w-full md:w-1/3 flex flex-col gap-6">
          <div className="p-6 rounded-2xl shadow-sm border border-gray-200" style={{ background: 'rgba(255, 255, 255, 0.6)', backdropFilter: 'blur(30px) saturate(210%)' }}>
            <h2 className="text-lg font-bold font-outfit mb-4">Card Details</h2>

            <div className="flex flex-col gap-4">
              <div>
                <label className="block text-sm font-semibold text-gray-700 mb-1">Gift Card Value</label>
                <div className="relative">
                  <span className="absolute left-3 top-1/2 -translate-y-1/2 text-gray-500 font-semibold">$</span>
                  <input
                    type="number"
                    value={value}
                    onChange={(e) => setValue(e.target.value)}
                    className="w-full pl-8 pr-4 py-2 rounded-xl border border-gray-300 bg-white/65 backdrop-blur-[30px] saturate-[210%] shadow-sm-sm focus:ring-2 focus:ring-indigo-500 focus:border-indigo-500 transition-all font-semibold"
                    min="5"
                    max="1000"
                  />
                </div>
              </div>

              <div className="pt-4 border-t border-gray-200">
                  <label className="flex items-center gap-3 cursor-pointer">
                      <input
                          type="checkbox"
                          checked={removeBranding}
                          onChange={() => setRemoveBranding(!removeBranding)}
                          className="w-5 h-5 rounded text-indigo-600 focus:ring-indigo-500 cursor-pointer"
                      />
                      <span className="text-sm font-medium text-gray-700">Remove "Powered by OHC" Badge (Pro)</span>
                  </label>
                  {removeBranding && (
                      <p className="text-xs text-amber-600 mt-2 p-2 bg-amber-50 rounded-lg border border-amber-200">
                          Note: Removing the branding badge requires an active Pro subscription.
                      </p>
                  )}
              </div>

              <button
                onClick={handleGenerate}
                className="mt-4 w-full py-3 bg-black text-white rounded-xl font-bold shadow-md hover:-translate-y-0.5 hover:shadow-lg transition-all"
              >
                Generate Gift Card
              </button>
            </div>
          </div>
        </section>

        {/* Live Preview Panel */}
        <section className="w-full md:w-2/3 flex flex-col gap-4">
           <h2 className="text-lg font-bold font-outfit px-2">Live Preview</h2>

           <div className="relative w-full aspect-[16/9] rounded-3xl overflow-hidden shadow-2xl flex flex-col items-center justify-center p-8 text-white transition-all duration-500 border border-white/20"
                style={{ background: 'linear-gradient(135deg, #FF9A9E 0%, #FECFEF 99%, #FECFEF 100%)', boxShadow: '0 20px 40px rgba(0,0,0,0.1)' }}>

               {/* Decorative elements */}
               <div className="absolute top-0 right-0 w-64 h-64 bg-white/20 rounded-full blur-3xl translate-x-1/3 -translate-y-1/3 pointer-events-none"></div>
               <div className="absolute bottom-0 left-0 w-64 h-64 bg-indigo-500/10 rounded-full blur-3xl -translate-x-1/3 translate-y-1/3 pointer-events-none"></div>

               <div className="z-10 flex flex-col items-center text-center w-full">
                  <div className="w-16 h-16 bg-white/30 rounded-2xl flex items-center justify-center text-3xl mb-4 backdrop-blur-[30px] saturate-[210%] shadow-inner border border-white/40">
                    🎁
                  </div>
                  <h3 className="text-2xl font-bold font-outfit tracking-wide uppercase drop-shadow-sm opacity-90">Digital Gift Card</h3>
                  <div className="text-7xl font-extrabold font-outfit my-2 drop-shadow-md">
                     ${value || '0'}
                  </div>
               </div>

               {/* Viral Loop Footer */}
               {!removeBranding && (
                  <div className="absolute bottom-4 left-0 w-full flex justify-center z-20">
                     <a href={`/api/v1/growth/referrals/click?target=/onboarding&ref=${tenantId}&source=gift_card`} target="_blank" className="text-xs font-bold tracking-widest uppercase opacity-80 mix-blend-overlay shadow-sm px-3 py-1 bg-white/10 rounded-full backdrop-blur-[30px] saturate-[210%] text-white hover:text-white" style={{ textDecoration: "none" }}>
                         ⚡ Powered by OHC
                     </a>
                  </div>
               )}
           </div>

           {/* Share Modal */}
           {showShareModal && (
             <div className="mt-4 p-6 rounded-2xl bg-white/65 backdrop-blur-[30px] saturate-[210%] shadow-sm-lg border border-gray-100 flex flex-col gap-4 animate-in slide-in-from-bottom-4">
                <h3 className="text-lg font-bold font-outfit text-gray-900">Share Your Gift Card</h3>
                <p className="text-sm text-gray-600">Send this link directly to your customer. They can claim it securely online.</p>

                <div className="flex gap-2">
                   <input
                      type="text"
                      readOnly
                      value={shareLink}
                      aria-label="Gift Card Link"
                      className="flex-1 px-4 py-3 rounded-xl border border-gray-200 bg-gray-50 text-gray-700 text-sm font-medium focus:outline-none"
                   />
                   <button
                      onClick={copyLink}
                      className={`px-6 py-3 rounded-xl font-bold text-sm transition-all shadow-sm ${copied ? 'bg-green-100 text-green-700 border border-green-200' : 'bg-indigo-600 text-white hover:bg-indigo-700'}`}
                   >
                      {copied ? 'Copied!' : 'Copy Link'}
                   </button>
                </div>

                <div className="flex gap-2 mt-2">
                   <a
                      href={`https://wa.me/?text=${encodeURIComponent(`Here is your $${value} gift card! Claim it here: ${shareLink} ${!removeBranding ? '⚡ Powered by OHC' : ''}`)}`}
                      target="_blank" rel="noreferrer"
                      className="flex-1 py-2 bg-[#25D366] text-white rounded-lg text-center font-semibold text-sm shadow-sm hover:opacity-90"
                   >
                     WhatsApp
                   </a>
                   <a
                      href={`mailto:?subject=Your $${value} Gift Card&body=${encodeURIComponent(`Here is your $${value} gift card! Claim it here: \n\n${shareLink}\n\n${!removeBranding ? '⚡ Powered by OHC' : ''}`)}`}
                      className="flex-1 py-2 bg-gray-800 text-white rounded-lg text-center font-semibold text-sm shadow-sm hover:opacity-90"
                   >
                     Email
                   </a>
                </div>
             </div>
           )}
        </section>
      </main>

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}
