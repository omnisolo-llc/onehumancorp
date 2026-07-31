"use client";

import React, { useState } from 'react';
import Link from 'next/link';

export default function TrialExtensionPage() {
  const [isClaiming, setIsClaiming] = useState(false);
  const [hasClaimed, setHasClaimed] = useState(false);
  const [error, setError] = useState('');

  const handleShareAndClaim = async () => {
    setIsClaiming(true);
    setError('');

    const message = "I just set up my AI-powered storefront using OneHumanCorp! 🚀 Get your own assistant-led business hub today. #OneHumanCorp #SmallBiz";
    const shareUrl = `https://twitter.com/intent/tweet?text=${encodeURIComponent(message)}`;

    // Open the share window
    window.open(shareUrl, '_blank');

    try {
      const response = await fetch('/api/v1/growth/trial-extension/claim', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        }
      });

      if (response.ok) {
        setHasClaimed(true);
      } else {
        setError("Pro activation could not be confirmed. Please try again.");
      }
    } catch {
      setError("The Pro activation service is unavailable. Please try again later.");
    } finally {
      setIsClaiming(false);
    }
  };

  return (
    <div className="flex flex-col min-h-screen font-inter bg-[#F5F5F7] dark:bg-black transition-colors duration-300">
      <header className="px-4 md:px-6 py-4 flex items-center justify-between border-b sticky top-0 z-50 glassmorphism backdrop-blur-[30px] saturate-[210%] bg-white/60 dark:bg-black/40 border-white/40 dark:border-white/10 shadow-sm">
        <h1 className="text-xl md:text-2xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] tracking-tight">Interactive Pro Activation</h1>
        <Link href="/dashboard" className="px-4 py-2 min-h-[44px] flex items-center justify-center bg-white/80 dark:bg-[#1C1C1E] border border-[#E5E5EA] dark:border-[#38383A] rounded-xl text-xs md:text-sm font-medium text-black dark:text-white hover:bg-gray-100 dark:hover:bg-gray-800 transition-all shadow-sm">
          Back to Dashboard
        </Link>
      </header>

      <main className="p-4 md:p-8 flex-1 w-full max-w-4xl mx-auto flex flex-col items-center justify-center">
        <div className="w-full glassmorphism backdrop-blur-[30px] saturate-[210%] bg-white/65 dark:bg-[#1C1C1E]/65 rounded-[24px] shadow-xl overflow-hidden border border-white/40 dark:border-white/10 p-8 md:p-12 text-center max-w-2xl relative transition-all">

          <div className="absolute top-0 left-0 w-full h-2 bg-gradient-to-r from-blue-500 via-indigo-500 to-purple-500"></div>

          {!hasClaimed ? (
            <div className="animate-fade-in flex flex-col items-center">
              <div className="w-20 h-20 bg-indigo-50 dark:bg-indigo-950/40 text-indigo-600 dark:text-indigo-400 rounded-full flex items-center justify-center text-4xl mb-6 shadow-inner border border-indigo-100/50 dark:border-indigo-900/30">
                ⏳
              </div>
              <h2 className="text-2xl md:text-3xl font-bold font-outfit text-gray-900 dark:text-white mb-4 tracking-tight">Activate Pro Access?</h2>
              <p className="text-gray-600 dark:text-gray-300 mb-8 text-base md:text-lg leading-relaxed max-w-md">
                Share your new storefront on X, then ask the OHC entitlement service to activate Pro access for this account.
              </p>

              <button
                onClick={handleShareAndClaim}
                disabled={isClaiming}
                className={`w-full sm:w-auto min-h-[44px] px-8 py-4 bg-black dark:bg-white text-white dark:text-black font-bold rounded-xl shadow-lg transition-all flex items-center justify-center gap-3 text-base md:text-lg hover:opacity-90 active:scale-95 disabled:opacity-50 ${isClaiming ? 'cursor-wait' : 'hover:-translate-y-0.5'}`}
              >
                {isClaiming ? (
                  <>
                    <svg className="animate-spin -ml-1 mr-2 h-5 w-5 text-white dark:text-black" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                      <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
                      <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                    </svg>
                    Verifying Share...
                  </>
                ) : (
                  <>
                    <svg className="w-5 h-5 md:w-6 md:h-6" fill="currentColor" viewBox="0 0 24 24"><path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.008 5.94H5.078z"/></svg>
                    Share on X to Activate Pro
                  </>
                )}
              </button>
              {error && <p className="mt-4 text-sm font-semibold text-red-600 dark:text-red-400" role="alert">{error}</p>}
            </div>
          ) : (
            <div className="animate-fade-in flex flex-col items-center">
              <div className="w-20 h-20 bg-green-50 dark:bg-green-950/40 text-green-600 dark:text-green-400 rounded-full flex items-center justify-center text-4xl mb-6 shadow-inner border border-green-100/50 dark:border-green-900/30">
                🎉
              </div>
              <h2 className="text-2xl md:text-3xl font-bold font-outfit text-gray-900 dark:text-white mb-4 tracking-tight">Pro Access Activated</h2>
              <p className="text-gray-600 dark:text-gray-300 mb-8 text-base md:text-lg leading-relaxed max-w-md">
                Thank you for sharing! The backend has successfully confirmed and activated your Pro access. Enjoy all premium capabilities.
              </p>

              <Link href="/dashboard" className="inline-flex w-full sm:w-auto min-h-[44px] px-8 py-4 bg-indigo-600 dark:bg-indigo-500 text-white font-bold rounded-xl shadow-lg transition-all hover:bg-indigo-500 dark:hover:bg-indigo-400 hover:-translate-y-0.5 active:scale-95 text-base md:text-lg items-center justify-center">
                Return to Dashboard
              </Link>
            </div>
          )}
        </div>
      </main>
      <div className="mt-8 text-center pb-8"><a href="/api/v1/growth/referrals/click?target=/onboarding&ref=trial_extension" className="text-sm font-bold text-gray-400 dark:text-gray-500 hover:text-indigo-600 dark:hover:text-indigo-400 transition-colors uppercase tracking-widest font-outfit">⚡ Powered by OHC</a></div>

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
        @keyframes fadeIn { from { opacity: 0; transform: translateY(10px); } to { opacity: 1; transform: translateY(0); } }
        .animate-fade-in { animation: fadeIn 0.5s ease-out forwards; }
      `}} />
    </div>
  );
}
