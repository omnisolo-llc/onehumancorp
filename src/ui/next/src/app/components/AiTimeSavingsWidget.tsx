"use client";

import React, { useState, useEffect } from 'react';

export default function AiTimeSavingsWidget() {
  const [hasClaimed, setHasClaimed] = useState(false);
  const [isClaiming, setIsClaiming] = useState(false);
  const [hasPro, setHasPro] = useState(false);
  const [savingsData, setSavingsData] = useState<any>(null);

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

  if (!savingsData) {
    return null;
  }

  if (hasClaimed) {
    return (
      <div className="rounded-[12px] bg-gradient-to-br from-green-50 to-emerald-50 dark:from-green-900/20 dark:to-emerald-900/20 border border-green-200 dark:border-green-800 shadow-sm p-6 mb-8 text-center animate-in fade-in zoom-in duration-300">
        <div className="w-16 h-16 bg-green-100 dark:bg-green-800/50 rounded-full flex items-center justify-center text-3xl mx-auto mb-4 text-green-600 dark:text-green-400">
          🎉
        </div>
        <h2 className="text-2xl font-bold font-outfit text-green-900 dark:text-green-100 mb-2">
          Trial Extended!
        </h2>
        <p className="text-green-700 dark:text-green-300">
          Your Pro trial has been successfully extended by 7 days. Enjoy the extra time!
        </p>
      </div>
    );
  }

  return (
    <div className="rounded-[12px] bg-white/65 backdrop-blur-[30px] backdrop-saturate-[2.1] border border-white/40 dark:bg-[#16161a]/70 dark:backdrop-blur-[30px] dark:backdrop-saturate-[2.1] dark:border-white/10 shadow-sm p-6 mb-8 relative overflow-hidden">
      <div className="absolute top-0 right-0 w-32 h-32 bg-indigo-500/10 rounded-full blur-3xl -z-10 translate-x-1/2 -translate-y-1/2"></div>

      <div className="flex flex-col md:flex-row items-center justify-between gap-6">
        <div className="flex items-start gap-4">
          <div className="w-12 h-12 rounded-xl bg-indigo-50 dark:bg-indigo-900/30 flex items-center justify-center text-2xl shrink-0">
            ⏳
          </div>
          <div>
            <h2 className="text-xl font-bold font-outfit text-gray-900 dark:text-white mb-1">
              You saved {savingsData.hours_saved} hours this week
            </h2>
            <p className="text-sm text-gray-600 dark:text-gray-400">
              Your AI agents handled {savingsData.inquiries_handled} customer inquiries and scheduled {savingsData.appointments_scheduled} appointments automatically.
            </p>
          </div>
        </div>

        <button
          onClick={handleShareAndClaim}
          disabled={isClaiming}
          className={`shrink-0 px-6 py-3 rounded-xl font-semibold text-white shadow-md transition-all flex items-center gap-2 ${
            isClaiming
              ? 'bg-indigo-400 cursor-wait'
              : 'bg-[#0066FF] hover:bg-blue-600 hover:-translate-y-0.5 hover:shadow-lg'
          }`}
        >
          {isClaiming ? (
            <>
              <svg className="animate-spin -ml-1 mr-2 h-4 w-4 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
                <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
              </svg>
              Verifying Share...
            </>
          ) : (
            <>
              <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 24 24">
                <path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.008 5.94H5.078z" />
              </svg>
              Share to get 7 Days Pro
            </>
          )}
        </button>
      </div>
    </div>
  );
}
