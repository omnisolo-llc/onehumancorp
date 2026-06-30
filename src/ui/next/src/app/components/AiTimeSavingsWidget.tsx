"use client";

import React, { useState, useEffect } from 'react';

export default function AiTimeSavingsWidget() {
  const [hasClaimed, setHasClaimed] = useState(false);
  const [isClaiming, setIsClaiming] = useState(false);
  const [hasPro, setHasPro] = useState(false);
  const [savingsData, setSavingsData] = useState({
    hours_saved: 0,
    inquiries_handled: 0,
    appointments_scheduled: 0,
    carts_recovered: 0,
    auto_replied: 0
  });

  useEffect(() => {
    if (typeof localStorage !== 'undefined') {
      const proStatus = localStorage.getItem('has_pro') === 'true';
      setHasPro(proStatus);
    }

    // Fetch real time savings data
    fetch('/api/v1/growth/time-savings')
      .then(res => {
        if (res.ok) return res.json();
        throw new Error('Failed to fetch savings');
      })
      .then(data => {
        if (data && typeof data.hours_saved === 'number') {
          setSavingsData(data);
        }
      })
      .catch(err => console.error("Error fetching time savings:", err));
  }, []);

  const handleShareAndClaim = async () => {
    setIsClaiming(true);

    const message = `My AI agents on OneHumanCorp just saved me ${savingsData.hours_saved} hours this week! 🚀 #OneHumanCorp #SmallBiz #AI`;
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
        if (typeof localStorage !== 'undefined') {
          localStorage.setItem('has_pro', 'true');
        }
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
      <div className="glassmorphism p-6 border border-green-200 dark:border-green-900/30 shadow-lg mb-6 flex items-center justify-between bg-green-50/50 dark:bg-green-900/10">
        <div className="flex items-center gap-4">
          <div className="w-12 h-12 rounded-full bg-green-100 dark:bg-green-800 text-green-600 dark:text-green-300 flex items-center justify-center text-2xl shadow-inner">
            🎉
          </div>
          <div>
            <h3 className="text-lg font-bold font-outfit text-gray-900 dark:text-white mb-1">Trial Extended!</h3>
            <p className="text-sm text-gray-600 dark:text-gray-300">Your Pro trial has been successfully extended by 7 days. Enjoy the extra time!</p>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="glassmorphism p-6 border border-indigo-200 dark:border-indigo-900/30 shadow-lg mb-6 relative overflow-hidden group">
      <div className="absolute -top-10 -right-10 w-32 h-32 bg-indigo-500/10 rounded-full blur-2xl"></div>

      <div className="flex flex-col md:flex-row gap-6 items-center justify-between relative z-10">
        <div className="flex items-start gap-4">
          <div className="w-12 h-12 rounded-full bg-indigo-100 dark:bg-indigo-900/50 text-indigo-600 dark:text-indigo-400 flex items-center justify-center text-2xl shadow-inner shrink-0 group-hover:scale-110 transition-transform">
            ⏱️
          </div>
          <div>
            <div className="inline-flex items-center gap-2 mb-1 px-2 py-0.5 rounded text-xs font-bold uppercase tracking-wider text-indigo-600 dark:text-indigo-400 bg-indigo-50 dark:bg-indigo-900/30">
              Weekly Insight
            </div>
            <h3 className="text-xl font-bold font-outfit text-gray-900 dark:text-white mb-2">
              You saved {savingsData.hours_saved} hours this week
            </h3>
            <p className="text-sm text-gray-600 dark:text-gray-300">
              Your AI agents handled {savingsData.inquiries_handled} customer inquiries (Auto-Replied: {savingsData.auto_replied || 0}), scheduled {savingsData.appointments_scheduled} appointments, and recovered {savingsData.carts_recovered} abandoned carts.
            </p>
          </div>
        </div>

        <div className="w-full md:w-auto shrink-0 flex flex-col items-center md:items-end">
           <button
            onClick={handleShareAndClaim}
            disabled={isClaiming}
            className={`w-full md:w-auto px-6 py-3 min-h-[44px] min-w-[44px] rounded-xl font-bold transition-all shadow-md flex items-center justify-center gap-2
              ${isClaiming ? 'bg-gray-100 text-gray-400 cursor-not-allowed' : 'bg-[#1DA1F2] hover:bg-[#1a91da] text-white hover:shadow-lg hover:-translate-y-0.5'}`}
          >
            {isClaiming ? (
              <>
                <svg className="animate-spin h-5 w-5 mr-2 text-gray-400" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                  <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
                  <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                </svg>
                Verifying Share...
              </>
            ) : (
              <>
                <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 24 24"><path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.008 5.94H5.078z"/></svg>
                Share to get 7 Days Pro
              </>
            )}
          </button>
          {!hasPro && (
             <p className="text-xs text-gray-400 mt-2 text-center md:text-right">
               Unlock premium tools by sharing your success.
             </p>
          )}
        </div>
      </div>
    </div>
  );
}
