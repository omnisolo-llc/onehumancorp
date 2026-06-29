"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';

export default function ViralPostGeneratorPage() {
  const router = useRouter();
  const [productName, setProductName] = useState('');
  const [keyBenefit, setKeyBenefit] = useState('');
  const [generatedPost, setGeneratedPost] = useState('');
  const [hasPro, setHasPro] = useState(false);
  const [showPaywall, setShowPaywall] = useState(false);
  const [copied, setCopied] = useState(false);
  const [removeBranding, setRemoveBranding] = useState(false);
  const [tenantId, setTenantId] = useState('my-store');
  const [hasSharedToUnlock, setHasSharedToUnlock] = useState(false);

  useEffect(() => {
    if (typeof window !== 'undefined') {
      setHasPro(localStorage.getItem('has_pro') === 'true');
      const tenant = localStorage.getItem('tenant') || 'my-store';
      setTenantId(tenant);
      const hasShared = localStorage.getItem('ohc_post_gen_shared') === 'true';
      setHasSharedToUnlock(hasShared);
      if (hasShared) {
        setRemoveBranding(true);
      }
    }
  }, []);

  const handleBrandingToggle = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (!hasPro && !hasSharedToUnlock) {
      e.preventDefault();
      setShowPaywall(true);
    } else {
      setRemoveBranding(e.target.checked);
    }
  };

  const handleGenerate = () => {
    if (!productName || !keyBenefit) return;
    const post = `Just dropped something special! 🚀 Introducing the new ${productName}. If you've been looking for ${keyBenefit}, this is for you.\n\nShop now: https://${tenantId}.ohc.app\n\n${!removeBranding ? '⚡ Powered by OHC' : ''}`;
    setGeneratedPost(post);
  };

  const claimTrialExtension = () => {
    const referralUrl = `${window.location.origin}/onboarding?ref=${tenantId}`;
    window.open(`https://twitter.com/intent/tweet?text=${encodeURIComponent('I just unlocked powerful AI tools for my business on One Human Corp! Start your own business today: ' + referralUrl)}`, '_blank');
    if (typeof localStorage !== 'undefined') {
        localStorage.setItem('ohc_post_gen_shared', 'true');
    }
    setHasSharedToUnlock(true);
    setRemoveBranding(true);
    setShowPaywall(false);
  };

  return (
    <div className="flex flex-col min-h-screen font-inter" style={{ backgroundColor: '#F5F5F7' }}>
      {/* Header */}
      <header className="px-6 py-4 flex items-center justify-between border-b" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', borderBottom: '1px solid rgba(255, 255, 255, 0.4)', position: 'sticky', top: 0, zIndex: 50 }}>
         <h1 className="text-2xl font-bold font-outfit" style={{ color: '#1D1D1F', letterSpacing: '-0.02em' }}>Promoter Agent Post Generator 🚀</h1>
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
                <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2">Automate Your Marketing</h2>
                <p className="text-gray-600 text-sm">
                    Generate highly converting social media posts instantly. Let our AI promoter agent do the heavy lifting.
                </p>
            </div>

            <div className="p-6 shadow-md" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)', borderRadius: '16px' }}>
                <h2 className="text-xl font-semibold font-outfit mb-4" style={{ color: '#1D1D1F' }}>Post Details</h2>
                <div className="flex flex-col gap-4">
                    <div>
                        <label className="block text-sm font-medium text-gray-700 mb-1">Product Name</label>
                        <input
                            type="text"
                            placeholder="e.g. Signature Coffee Blend"
                            value={productName}
                            onChange={(e) => setProductName(e.target.value)}
                            className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500"
                        />
                    </div>
                    <div>
                        <label className="block text-sm font-medium text-gray-700 mb-1">Key Benefit</label>
                        <input
                            type="text"
                            placeholder="e.g. a bold start to your morning"
                            value={keyBenefit}
                            onChange={(e) => setKeyBenefit(e.target.value)}
                            className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500"
                        />
                    </div>

                    <div className="pt-4 border-t border-gray-100">
                        <label className="flex items-start gap-3 cursor-pointer group">
                        <input
                            type="checkbox"
                            checked={removeBranding}
                            onChange={handleBrandingToggle}
                            className="mt-1 w-4 h-4 text-indigo-600 rounded focus:ring-indigo-500"
                        />
                        <div>
                            <span className="text-sm font-medium text-gray-900">Remove "Powered by OHC" branding</span>
                            <p className="text-xs text-gray-500 mt-1">Make the post 100% white-labeled. Requires Pro plan.</p>
                        </div>
                        </label>
                    </div>

                    <button
                        onClick={handleGenerate}
                        className="w-full mt-2 py-3 bg-indigo-600 hover:bg-indigo-700 text-white font-bold rounded-xl shadow-md transition-all active:scale-[0.98] text-sm flex items-center justify-center gap-2"
                    >
                        Generate Post
                    </button>
                </div>
            </div>
        </section>

        {/* Live Preview */}
        <section className="w-full md:w-1/2 flex flex-col gap-4">
             <h2 className="text-xl font-semibold font-outfit" style={{ color: '#1D1D1F' }}>Generated Post</h2>

             <div className="w-full min-h-[400px] bg-white rounded-2xl shadow-sm border border-gray-200 relative overflow-hidden flex flex-col p-6">
                 {generatedPost ? (
                     <div className="flex-1 whitespace-pre-wrap text-gray-800 text-sm leading-relaxed">
                         {generatedPost}
                     </div>
                 ) : (
                     <div className="flex-1 flex flex-col items-center justify-center text-gray-400">
                         <div className="w-16 h-16 bg-gray-100 rounded-full flex items-center justify-center text-2xl mb-4">
                             📝
                         </div>
                         <p className="text-sm font-medium">Fill out the details to generate a post</p>
                     </div>
                 )}

                 {generatedPost && (
                    <button
                        id="copy-post-btn"
                        onClick={() => {
                            navigator.clipboard.writeText(generatedPost);
                            setCopied(true);
                            setTimeout(() => setCopied(false), 2000);
                        }}
                        className={`mt-4 w-full py-3 rounded-lg text-sm font-semibold transition-all ${copied ? 'bg-green-100 text-green-700' : 'bg-gray-900 text-white hover:bg-black'}`}
                    >
                        {copied ? 'Copied!' : 'Copy to Clipboard'}
                    </button>
                 )}
             </div>
        </section>
      </main>

      {/* Soft Paywall Modal */}
      {showPaywall && (
        <div className="fixed inset-0 bg-black/60 z-[60] flex items-center justify-center p-4">
          <div className="bg-white w-full max-w-md rounded-2xl p-8 shadow-2xl relative overflow-hidden font-inter border border-blue-100 text-center">
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

            <div className="w-16 h-16 bg-indigo-50 rounded-2xl flex items-center justify-center mx-auto mb-6 border border-indigo-100">
              <span className="text-3xl text-indigo-600">🚀</span>
            </div>

            <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-3">Upgrade to Pro</h2>
            <p className="text-gray-600 mb-8 text-sm leading-relaxed">
              Make the post 100% white-labeled. Upgrade to Pro to remove the "Powered by OHC" watermark.
            </p>

            <button
              onClick={() => { setShowPaywall(false); router.push('/pricing'); }}
              className="w-full py-4 rounded-xl font-bold text-white mb-3 transition-all shadow-md hover:shadow-lg hover:opacity-90 bg-indigo-600 hover:bg-indigo-700 flex justify-center items-center gap-2"
            >
              Upgrade to Pro
            </button>

            <button
              onClick={claimTrialExtension}
              className="w-full py-3.5 rounded-xl font-bold transition-all shadow-sm bg-black text-white border-2 border-black hover:bg-gray-800 flex items-center justify-center gap-2"
            >
              <svg className="w-5 h-5" style={{ width: '20px', height: '20px' }} fill="currentColor" viewBox="0 0 24 24"><path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.008 5.94H5.078z"/></svg>
              Share on X to Unlock for Free
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
