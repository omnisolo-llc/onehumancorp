"use client";

import React, { useState, Suspense } from 'react';
import { useSearchParams } from 'next/navigation';

function CouponEmbedContent() {
  const searchParams = useSearchParams();
  const title = searchParams.get('title') || 'Special Discount';
  const percent = searchParams.get('percent') || '20';
  const code = searchParams.get('code') || 'SAVE20';
  const tenant = searchParams.get('tenant_id') || 'my-store';
  const theme = searchParams.get('theme') || 'light';

  const [unlocked, setUnlocked] = useState(false);
  const [copied, setCopied] = useState(false);

  const handleShare = () => {
    // Open share window
    const shareText = `I just got a special discount from ${tenant}! Get yours here: ${window.location.origin}/embed/coupon?tenant_id=${tenant}`;
    window.open(`https://twitter.com/intent/tweet?text=${encodeURIComponent(shareText)}`, '_blank', 'width=600,height=400');

    // Simulate API call to register viral hook and unlock
    setTimeout(() => {
        setUnlocked(true);
    }, 1500);
  };

  const handleCopy = () => {
      navigator.clipboard.writeText(code);
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
  };

  const isDark = theme === 'dark';

  return (
    <div className={`min-h-screen flex items-center justify-center p-4 font-inter ${isDark ? 'bg-gray-900 text-white' : 'bg-transparent text-gray-900'}`}>
        <div className={`w-full max-w-sm rounded-3xl shadow-xl overflow-hidden relative border ${isDark ? 'border-gray-800 bg-gray-900' : 'border-gray-200 bg-white'} flex flex-col items-center`}>
            <div className="w-full h-32 bg-gradient-to-br from-indigo-500 via-purple-500 to-pink-500 relative flex items-center justify-center">
                <span className="text-5xl drop-shadow-md text-white font-bold">{percent}% OFF</span>
            </div>

            <div className="w-full p-8 flex flex-col items-center text-center">
                <h2 className={`text-2xl font-bold font-outfit mb-2 ${isDark ? 'text-white' : 'text-gray-900'}`}>
                    {title}
                </h2>

                {!unlocked ? (
                    <>
                        <p className={`text-sm mb-6 leading-relaxed ${isDark ? 'text-gray-400' : 'text-gray-600'}`}>
                            Share this page with a friend to reveal your exclusive promo code!
                        </p>

                        <div className={`w-full p-4 rounded-xl mb-6 relative overflow-hidden group ${isDark ? 'bg-gray-800' : 'bg-gray-100'}`}>
                            <div className="absolute inset-0 backdrop-blur-md bg-white/50 flex items-center justify-center z-10 transition-opacity">
                                <span className="text-sm font-bold text-gray-800 flex items-center gap-2">
                                    <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"></path></svg>
                                    Share to Unlock
                                </span>
                            </div>
                            <span className={`font-mono font-bold text-xl tracking-wider blur-sm select-none ${isDark ? 'text-gray-500' : 'text-gray-400'}`}>
                                {code}
                            </span>
                        </div>

                        <button onClick={handleShare} className="w-full py-3 bg-[#1DA1F2] hover:bg-[#1A91DA] text-white font-bold rounded-xl shadow-md flex items-center justify-center gap-2 transition-colors cursor-pointer z-20 relative">
                            <svg className="w-5 h-5 fill-current" viewBox="0 0 24 24"><path d="M23.953 4.57a10 10 0 01-2.825.775 4.958 4.958 0 002.163-2.723c-.951.555-2.005.959-3.127 1.184a4.92 4.92 0 00-8.384 4.482C7.69 8.095 4.067 6.13 1.64 3.162a4.822 4.822 0 00-.666 2.475c0 1.71.87 3.213 2.188 4.096a4.904 4.904 0 01-2.228-.616v.06a4.923 4.923 0 003.946 4.827 4.996 4.996 0 01-2.212.085 4.936 4.936 0 004.604 3.417 9.867 9.867 0 01-6.102 2.105c-.39 0-.779-.023-1.17-.067a13.995 13.995 0 007.557 2.209c9.053 0 13.998-7.496 13.998-13.985 0-.21 0-.42-.015-.63A9.935 9.935 0 0024 4.59z"/></svg>
                            Share on Twitter
                        </button>
                    </>
                ) : (
                    <div className="w-full animate-fade-in-up">
                        <p className={`text-sm mb-4 font-semibold ${isDark ? 'text-green-400' : 'text-green-600'}`}>
                            Unlocked successfully! 🎉
                        </p>

                        <div className={`w-full p-4 rounded-xl mb-6 border ${isDark ? 'bg-gray-800 border-gray-700' : 'bg-gray-50 border-gray-200'} flex items-center justify-between`}>
                            <span className={`font-mono font-bold text-xl tracking-wider ${isDark ? 'text-white' : 'text-gray-900'}`}>
                                {code}
                            </span>
                            <button onClick={handleCopy} className={`px-3 py-1.5 rounded-lg text-sm font-semibold transition-all ${copied ? 'bg-green-100 text-green-700' : isDark ? 'bg-gray-700 text-white hover:bg-gray-600' : 'bg-gray-200 text-gray-800 hover:bg-gray-300'}`}>
                                {copied ? 'Copied' : 'Copy'}
                            </button>
                        </div>

                        <p className={`text-xs ${isDark ? 'text-gray-400' : 'text-gray-500'}`}>
                            Apply this code at checkout to get {percent}% off your order.
                        </p>
                    </div>
                )}
            </div>
        </div>
        <style dangerouslySetInnerHTML={{__html: `
            @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
            .font-inter { font-family: 'Inter', sans-serif; }
            .font-outfit { font-family: 'Outfit', sans-serif; }
            @keyframes fade-in-up {
              0% { opacity: 0; transform: translateY(10px); }
              100% { opacity: 1; transform: translateY(0); }
            }
            .animate-fade-in-up { animation: fade-in-up 0.4s ease-out forwards; }
        `}} />
    </div>
  );
}

export default function CouponEmbedPage() {
  return (
    <Suspense fallback={<div className="min-h-screen flex items-center justify-center font-inter">Loading widget...</div>}>
      <CouponEmbedContent />
    </Suspense>
  );
}
