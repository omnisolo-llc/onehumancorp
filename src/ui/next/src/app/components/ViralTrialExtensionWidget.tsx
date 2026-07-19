"use client";

import React, { useState } from 'react';

export function ViralTrialExtensionWidget() {
  const [isClaiming, setIsClaiming] = useState(false);
  const [hasClaimed, setHasClaimed] = useState(false);

  const handleShareAndClaim = async () => {
    setIsClaiming(true);

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
        console.error('Failed to claim trial extension');
        alert("Failed to claim trial extension. Please try again.");
      }
    } catch (error) {
      console.error('Error claiming trial extension:', error);
      alert("Error claiming trial extension. Please try again.");
    } finally {
      setIsClaiming(false);
    }
  };

  if (hasClaimed) {
    return (
      <div className="mt-4 p-4 bg-green-50 rounded-xl border border-green-100 text-center animate-fade-in">
        <div className="text-green-600 text-2xl mb-2">🎉</div>
        <h4 className="font-bold text-gray-900 text-sm mb-1 font-outfit">Trial Extended!</h4>
        <p className="text-xs text-gray-600">You've unlocked 7 days of Pro for free.</p>
      </div>
    );
  }

  return (
    <div className="mt-4 p-4 bg-indigo-50/50 rounded-xl border border-indigo-100 relative overflow-hidden group">
      <div className="absolute top-0 right-0 w-16 h-16 bg-indigo-100 rounded-bl-full -z-10 group-hover:scale-110 transition-transform"></div>
      <h4 className="font-bold text-gray-900 text-sm mb-2 font-outfit flex items-center gap-2">
        <span className="text-indigo-600">🚀</span> Want 7 Extra Days of Pro?
      </h4>
      <p className="text-xs text-gray-600 mb-3">
        Share on X (Twitter) to unlock a free week of advanced features.
      </p>
      <button
        onClick={handleShareAndClaim}
        disabled={isClaiming}
        className={`w-full py-2 bg-indigo-600 text-white rounded-lg text-xs font-semibold hover:bg-indigo-700 transition-colors flex items-center justify-center gap-2 ${isClaiming ? 'opacity-70 cursor-wait' : ''}`}
      >
        {isClaiming ? (
           <>
             <svg className="animate-spin h-3 w-3 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
               <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
               <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
             </svg>
             Verifying...
           </>
        ) : (
          "Share to Unlock"
        )}
      </button>
    </div>
  );
}
