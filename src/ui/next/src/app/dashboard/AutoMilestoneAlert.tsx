"use client";

import React, { useState, useEffect } from 'react';
import Link from 'next/link';

export function AutoMilestoneAlert({ tenantId }: { tenantId: string }) {
  const [milestone, setMilestone] = useState<number | null>(null);
  const [loading, setLoading] = useState(true);
  const [dismissed, setDismissed] = useState(false);
  const [copied, setCopied] = useState(false);

  useEffect(() => {
    // Check local storage so we don't spam the user
    const dismissedMilestone = localStorage.getItem(`dismissed_milestone_${tenantId}`);

    async function checkMilestones() {
      try {
        const res = await fetch(`/api/ui/dashboard/metrics?tenant_id=${tenantId}`);
        if (res.ok) {
          const data = await res.json();
          const orders = data.total_sales || 0; // Using total_sales as order count for now based on available metrics

          let currentMilestone = null;
          if (orders >= 100) currentMilestone = 100;
          else if (orders >= 50) currentMilestone = 50;
          else if (orders >= 10) currentMilestone = 10;
          else if (orders >= 1) currentMilestone = 1;

          if (currentMilestone && dismissedMilestone !== String(currentMilestone)) {
             setMilestone(currentMilestone);
          }
        }
      } catch (err) {
        console.error("Failed to check milestones", err);
      } finally {
        setLoading(false);
      }
    }

    checkMilestones();
  }, [tenantId]);

  if (loading || !milestone || dismissed) return null;

  const milestoneTitle = milestone === 1 ? "First Order! 🎉" : `${milestone}th Order! 🚀`;
  const shareTarget = typeof window !== 'undefined' ? `${window.location.origin}/onboarding?ref=${tenantId}` : `/onboarding?ref=${tenantId}`;
  const shareText = `I just hit my ${milestoneTitle} milestone on my @OneHumanCorp store! 🚀 Start yours here: ${shareTarget}`;

  const handleDismiss = () => {
    localStorage.setItem(`dismissed_milestone_${tenantId}`, String(milestone));
    setDismissed(true);
  };

  const copyToClipboard = () => {
    navigator.clipboard.writeText(shareText);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div className="mb-6 p-6 rounded-[16px] relative overflow-hidden shadow-xl"
         style={{
           background: 'linear-gradient(135deg, rgba(79, 70, 229, 0.95), rgba(219, 39, 119, 0.95))',
           backdropFilter: 'blur(20px) saturate(200%)',
           border: '1px solid rgba(255, 255, 255, 0.3)'
         }}>

      {/* Decorative pulse element */}
      <div className="absolute top-0 right-0 w-64 h-64 bg-white opacity-20 rounded-full blur-3xl -mr-10 -mt-10 animate-pulse pointer-events-none"></div>

      <button onClick={handleDismiss} className="absolute top-4 right-4 text-white/70 hover:text-white" aria-label="Dismiss">
        <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M6 18L18 6M6 6l12 12"></path></svg>
      </button>

      <div className="relative z-10 flex flex-col items-center text-center text-white">
        <div className="w-16 h-16 bg-white/20 rounded-full flex items-center justify-center text-3xl backdrop-blur-md border border-white/30 mb-4 shadow-inner">
           🏆
        </div>
        <h2 className="text-2xl font-bold font-outfit mb-2 drop-shadow-md">Achievement Unlocked: {milestoneTitle}</h2>
        <p className="text-indigo-100 font-medium mb-6 drop-shadow-sm max-w-md">
          Your business is growing fast! Share your success on X and earn premium referral credits when someone signs up using your link.
        </p>

        <div className="flex flex-col sm:flex-row gap-3 w-full sm:w-auto">
          <a
            href={`https://twitter.com/intent/tweet?text=${encodeURIComponent(shareText)}`}
            target="_blank"
            rel="noopener noreferrer"
            onClick={handleDismiss}
            className="flex items-center justify-center gap-2 bg-black text-white px-6 py-3 rounded-xl font-bold text-sm shadow-lg hover:bg-gray-800 transition-all hover:-translate-y-0.5"
            data-testid="share-milestone-x"
          >
            <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 24 24"><path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.008 5.94H5.078z"/></svg>
            Share on X
          </a>
          <button
            onClick={copyToClipboard}
            className={`px-6 py-3 rounded-xl text-sm font-bold transition-all shadow-lg ${copied ? 'bg-green-100 text-green-700' : 'bg-white text-indigo-900 hover:bg-indigo-50 hover:-translate-y-0.5'}`}
            data-testid="share-milestone-copy"
          >
            {copied ? 'Copied Link!' : 'Copy Referral Link'}
          </button>
        </div>
      </div>
    </div>
  );
}
