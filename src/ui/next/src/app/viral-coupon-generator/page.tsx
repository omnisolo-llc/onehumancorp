"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';

export default function ViralCouponGeneratorPage() {
  const router = useRouter();
  const [tenant, setTenant] = useState('DEFAULT');
  const [title, setTitle] = useState('');
  const [discountCode, setDiscountCode] = useState('');
  const [discountPercent, setDiscountPercent] = useState('');
  const [isGenerating, setIsGenerating] = useState(false);
  const [embedCode, setEmbedCode] = useState('');
  const [copied, setCopied] = useState(false);
  const [showSoftPaywall, setShowSoftPaywall] = useState(false);
  const [hasPro, setHasPro] = useState(false);

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

  const handleGenerate = () => {
    if (!hasPro) {
      setShowSoftPaywall(true);
      return;
    }
    setIsGenerating(true);
    const origin = typeof window !== 'undefined' ? window.location.origin : 'https://ohc.app';
    const widgetUrl = `${origin}/embed/coupon?tenant_id=${encodeURIComponent(tenant)}&title=${encodeURIComponent(title || 'Special Discount')}&code=${encodeURIComponent(discountCode || 'SAVE20')}&percent=${encodeURIComponent(discountPercent || '20')}&theme=light`;

    const code = `<iframe src="${widgetUrl}" width="100%" height="400" frameborder="0" style="border: 1px solid #e5e7eb; border-radius: 12px; overflow: hidden; background: transparent;"></iframe>\n<div style="text-align: center; margin-top: 8px;"><a href="${origin}/onboarding?ref=${tenant}" target="_blank" style="font-size: 11px; color: #9ca3af; text-decoration: none; font-family: sans-serif;">⚡ Powered by OHC</a></div>`;

    setEmbedCode(code);
    setIsGenerating(false);
  };

  const claimTrialExtension = () => {
    const referralUrl = `${window.location.origin}/onboarding?ref=${tenant}`;
    window.open(`https://twitter.com/intent/tweet?text=${encodeURIComponent('I just created a viral coupon widget for my business on One Human Corp! Start your own business today: ' + referralUrl)}`, '_blank');
    if (typeof localStorage !== 'undefined') {
        localStorage.setItem('has_pro', 'true');
        window.dispatchEvent(new Event('storage'));
    }
    setHasPro(true);
    setShowSoftPaywall(false);
  };

  return (
    <div className="flex flex-col min-h-screen font-inter" style={{ backgroundColor: '#F5F5F7' }}>
      <header className="px-6 py-4 flex items-center justify-between border-b" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', borderBottom: '1px solid rgba(255, 255, 255, 0.4)', position: 'sticky', top: 0, zIndex: 50 }}>
         <h1 className="text-2xl font-bold font-outfit" style={{ color: '#1D1D1F', letterSpacing: '-0.02em' }}>Viral Coupon Generator 🎟️</h1>
         <div className="flex items-center gap-3">
             <button onClick={() => router.push('/dashboard')} className="px-4 py-2 bg-gray-200 rounded-md text-sm font-medium hover:bg-gray-300 transition-colors">
               Back to Dashboard
             </button>
         </div>
      </header>

      <main className="p-6 md:p-8 flex-1 max-w-5xl mx-auto w-full flex flex-col md:flex-row gap-8">
        <section className="w-full md:w-1/2 flex flex-col gap-6">
          <div className="p-6 shadow-md" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)', borderRadius: '16px' }}>
            <div className="flex items-center gap-4 mb-4">
              <h2 className="text-xl font-semibold font-outfit m-0" style={{ color: '#1D1D1F' }}>Widget Settings</h2>
              <div className="flex items-center gap-2 px-3 py-1 bg-purple-50 rounded-full border border-purple-100">
                  <span className="text-xs font-medium text-purple-600">Pro Feature</span>
              </div>
            </div>

            <div className="flex flex-col gap-4">
              <div>
                <label htmlFor="coupon-title" className="block text-sm font-medium text-gray-700 mb-1">Offer Title</label>
                <input
                  id="coupon-title"
                  type="text"
                  value={title}
                  onChange={(e) => setTitle(e.target.value)}
                  placeholder="e.g. VIP Member Discount"
                  className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-purple-500"
                />
              </div>

              <div className="flex gap-4">
                  <div className="flex-1">
                    <label htmlFor="discount-percent" className="block text-sm font-medium text-gray-700 mb-1">Discount %</label>
                    <input
                      id="discount-percent"
                      type="number"
                      value={discountPercent}
                      onChange={(e) => setDiscountPercent(e.target.value)}
                      placeholder="e.g. 15"
                      className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-purple-500"
                    />
                  </div>
                  <div className="flex-1">
                    <label htmlFor="discount-code" className="block text-sm font-medium text-gray-700 mb-1">Discount Code</label>
                    <input
                      id="discount-code"
                      type="text"
                      value={discountCode}
                      onChange={(e) => setDiscountCode(e.target.value)}
                      placeholder="e.g. VIP15"
                      className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-purple-500"
                    />
                  </div>
              </div>

              <button
                onClick={handleGenerate}
                disabled={isGenerating}
                className={`w-full py-3 mt-4 text-white font-semibold rounded-xl shadow-lg transition-all ${isGenerating ? 'bg-purple-400 cursor-not-allowed' : 'bg-purple-600 hover:bg-purple-700 hover:shadow-xl hover:-translate-y-0.5 active:translate-y-0'}`}
              >
                {isGenerating ? 'Generating...' : 'Generate Embed Code'}
              </button>
            </div>
          </div>

          {embedCode && (
            <div className="p-6 shadow-md bg-white border border-green-200 rounded-[16px]">
              <h3 className="text-lg font-bold font-outfit text-gray-900 mb-2 flex items-center gap-2">
                <span className="text-green-500">✅</span> Code Ready!
              </h3>
              <p className="text-sm text-gray-600 mb-4">Paste this HTML snippet into your website or blog.</p>

              <div className="relative">
                <textarea
                  readOnly
                  value={embedCode}
                  rows={4}
                  className="w-full px-3 py-2 bg-gray-50 border border-gray-200 rounded-lg text-sm text-gray-700 font-mono focus:outline-none resize-none"
                />
                <button
                  onClick={() => {
                    navigator.clipboard.writeText(embedCode);
                    setCopied(true);
                    setTimeout(() => setCopied(false), 2000);
                  }}
                  className={`absolute top-2 right-2 px-3 py-1 rounded text-xs font-semibold transition-all ${copied ? 'bg-green-100 text-green-700' : 'bg-gray-200 text-gray-700 hover:bg-gray-300'}`}
                >
                  {copied ? 'Copied!' : 'Copy'}
                </button>
              </div>
            </div>
          )}
        </section>

        <section className="w-full md:w-1/2 flex justify-center items-start">
             <div className="w-full max-w-sm bg-white rounded-3xl shadow-xl overflow-hidden relative border border-gray-200 flex flex-col items-center">
                 <div className="w-full h-32 bg-gradient-to-br from-indigo-500 via-purple-500 to-pink-500 relative flex items-center justify-center">
                     <span className="text-5xl drop-shadow-md text-white font-bold">{discountPercent || '20'}% OFF</span>
                 </div>

                 <div className="w-full p-8 flex flex-col items-center text-center">
                     <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2">
                         {title || 'Special Discount'}
                     </h2>
                     <p className="text-sm text-gray-600 mb-6 leading-relaxed">
                         Share this page with a friend to reveal your exclusive promo code!
                     </p>

                     <div className="w-full p-4 bg-gray-100 rounded-xl mb-6 relative overflow-hidden group">
                         <div className="absolute inset-0 backdrop-blur-md bg-white/50 flex items-center justify-center z-10 transition-opacity">
                             <span className="text-sm font-bold text-gray-800 flex items-center gap-2">
                                <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"></path></svg>
                                Share to Unlock
                             </span>
                         </div>
                         <span className="font-mono font-bold text-xl text-gray-400 tracking-wider blur-sm select-none">
                             {discountCode || 'SAVE20'}
                         </span>
                     </div>

                     <button disabled className="w-full py-3 bg-[#1DA1F2] hover:bg-[#1A91DA] text-white font-bold rounded-xl shadow-md flex items-center justify-center gap-2 transition-colors">
                        <svg className="w-5 h-5 fill-current" viewBox="0 0 24 24"><path d="M23.953 4.57a10 10 0 01-2.825.775 4.958 4.958 0 002.163-2.723c-.951.555-2.005.959-3.127 1.184a4.92 4.92 0 00-8.384 4.482C7.69 8.095 4.067 6.13 1.64 3.162a4.822 4.822 0 00-.666 2.475c0 1.71.87 3.213 2.188 4.096a4.904 4.904 0 01-2.228-.616v.06a4.923 4.923 0 003.946 4.827 4.996 4.996 0 01-2.212.085 4.936 4.936 0 004.604 3.417 9.867 9.867 0 01-6.102 2.105c-.39 0-.779-.023-1.17-.067a13.995 13.995 0 007.557 2.209c9.053 0 13.998-7.496 13.998-13.985 0-.21 0-.42-.015-.63A9.935 9.935 0 0024 4.59z"/></svg>
                        Share on Twitter
                     </button>

                     <div className="mt-8">
                        <a href="#" className="text-xs font-semibold text-gray-400 uppercase tracking-widest">⚡ Powered by OHC</a>
                     </div>
                 </div>
             </div>
        </section>
      </main>

      {/* Soft Paywall Modal */}
      {showSoftPaywall && (
        <div className="fixed inset-0 bg-black/60 z-[60] flex items-center justify-center p-4">
          <div className="app-card w-full max-w-md rounded-2xl p-8 shadow-2xl relative overflow-hidden font-inter border border-purple-100 text-center bg-white">
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
              Viral Coupons are a Pro feature. Upgrade to our Pro plan to generate viral loops and drive exponential sales.
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
