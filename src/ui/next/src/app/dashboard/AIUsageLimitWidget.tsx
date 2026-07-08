"use client";

import React, { useState, useEffect } from 'react';
import Link from 'next/link';

export function AIUsageLimitWidget() {
  const [tenantId, setTenantId] = useState("default-team");
  const [referralLink, setReferralLink] = useState("");
  const [generating, setGenerating] = useState(false);
  const [copied, setCopied] = useState(false);
  const [actionsUsed, setActionsUsed] = useState(0);
  const [totalActions, setTotalActions] = useState(100);

  useEffect(() => {
    if (typeof window !== 'undefined') {
      const storedTenant = localStorage.getItem('tenant_id') || localStorage.getItem('tenant');
      const finalTenant = storedTenant || "default-team";
      setTenantId(finalTenant);

      fetch(`/api/v1/billing/department-tier-usage?tenant_id=${encodeURIComponent(finalTenant)}`)
        .then(res => res.json())
        .then(data => {
          if (data && Array.isArray(data.departments)) {
            let used = 0;
            let limit = 0;
            for (const d of data.departments) {
              used += d.actions_used || 0;
              limit += d.action_limit || 0;
            }
            setActionsUsed(used);
            if (limit > 0) {
              setTotalActions(limit);
            }
          }
        })
        .catch(err => console.error("Failed to fetch usage", err));
    }
  }, []);

  const progressPercentage = (actionsUsed / totalActions) * 100;

  // Progress bar color based on usage
  let progressColor = "bg-[#34C759]";
  if (progressPercentage >= 80) progressColor = "bg-[#FF9500]";
  if (progressPercentage >= 95) progressColor = "bg-[#FF3B30]";

  const handleGenerateLink = () => {
    setGenerating(true);
    // Simulate network delay for link generation
    setTimeout(() => {
      const link = `${window.location.origin}/onboarding?ref=${tenantId}&source=ai_limit_paywall`;
      setReferralLink(link);
      setGenerating(false);
    }, 800);
  };

  const handleCopy = () => {
    if (navigator.clipboard && referralLink) {
      navigator.clipboard.writeText(referralLink);
      setCopied(true);

      // Optimistically unlock
      setTimeout(() => {
         setActionsUsed(Math.max(0, actionsUsed - 50));
      }, 1500);

      setTimeout(() => setCopied(false), 2000);
    }
  };

  const shareText = `Start your business on OHC! It's super easy. Use my link to get $50 off your first month: ${referralLink}`;

  const handleXShare = () => {
     window.open(`https://twitter.com/intent/tweet?text=${encodeURIComponent(shareText)}`, '_blank');
     // Optimistically unlock after sharing
     setTimeout(() => {
         setActionsUsed(Math.max(0, actionsUsed - 50));
     }, 1500);
  };

  return (
    <div className="mb-6 ohc-growth-card rounded-[12px] bg-white/65 backdrop-blur-[30px] backdrop-saturate-[2.1] border border-white/40 dark:bg-[#16161a]/70 dark:backdrop-blur-[30px] dark:backdrop-saturate-[2.1] dark:border-white/10 shadow-sm p-6 border border-orange-200/60 dark:border-orange-800/60 bg-white/40 dark:bg-black/20 backdrop-blur-[30px] saturate-[210%] relative overflow-hidden" data-testid="ai-usage-limit-widget">
      {/* Blurred background effect */}
      <div className="absolute inset-0 pointer-events-none opacity-10 bg-[url('data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIyMCIgaGVpZ2h0PSIyMCI+CjxjaXJjbGUgY3g9IjIiIGN5PSIyIiByPSIyIiBmaWxsPSIjZjU5ZTBiIiBvcGFjaXR5PSIwLjQiLz4KPC9zdmc+')]"></div>

      <div className="relative z-10 flex flex-col items-start gap-4">
        <div className="w-full flex justify-between items-end">
            <div>
              <div className="inline-flex items-center gap-2 mb-2 px-3 py-1 rounded-full bg-orange-100 dark:bg-orange-900/50 text-orange-700 dark:text-orange-300 text-xs font-bold uppercase tracking-wider border border-orange-200 dark:border-orange-800">
                <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 10V3L4 14h7v7l9-11h-7z" /></svg>
                AI Actions Limit
              </div>
              <h2 className="text-xl md:text-2xl font-bold font-outfit text-gray-900 dark:text-white mb-1">
                Approaching Free Tier Limit
              </h2>
              <p className="text-sm text-gray-600 dark:text-gray-300">
                Your AI agents are working hard. You have used {actionsUsed} of your {totalActions} free actions this month.
              </p>
            </div>
            <div className="text-right">
                <span className="text-3xl font-bold font-outfit text-gray-900 dark:text-white">{actionsUsed}</span>
                <span className="text-gray-500 dark:text-gray-400 font-medium text-sm"> / {totalActions}</span>
            </div>
        </div>

        {/* Progress Bar */}
        <div className="w-full h-3 bg-gray-200 dark:bg-gray-700 rounded-full overflow-hidden border border-gray-300 dark:border-gray-600 shadow-inner">
            <div
                className={`h-full transition-all duration-1000 ease-out ${progressColor}`}
                style={{ width: `${Math.min(100, progressPercentage)}%` }}
            ></div>
        </div>

        <div className="w-full mt-2 flex flex-col sm:flex-row gap-4 items-center">
             <Link href="/pricing" className="w-full sm:w-auto px-6 py-2.5 bg-gray-900 dark:bg-white text-white dark:text-gray-900 font-semibold rounded-xl hover:bg-black dark:hover:bg-gray-100 transition-colors shadow-sm text-center">
                Upgrade to Pro (Unlimited)
             </Link>
             <span className="text-gray-400 font-medium text-sm">or</span>

             {!referralLink ? (
                 <button
                   onClick={handleGenerateLink}
                   disabled={generating}
                   className="w-full sm:w-auto px-6 py-2.5 bg-orange-600 text-white font-semibold rounded-xl hover:bg-orange-700 transition-colors shadow-sm disabled:opacity-70 flex justify-center items-center gap-2"
                 >
                   {generating ? 'Generating...' : 'Share on X to get +50 Actions'}
                 </button>
             ) : (
                 <div className="flex flex-col sm:flex-row gap-2 w-full sm:w-auto">
                    <button
                       onClick={handleCopy}
                       className={`px-4 py-2 text-sm font-bold rounded-lg transition-all flex items-center justify-center gap-2 ${copied ? 'bg-green-100 text-green-700' : 'bg-orange-50 text-orange-700 hover:bg-orange-100 border border-orange-200'}`}
                    >
                       {copied ? 'Copied Link!' : 'Copy Link'}
                    </button>
                    <button
                      onClick={handleXShare}
                      className="px-4 py-2 bg-black hover:bg-gray-800 text-white rounded-lg font-bold text-sm shadow-sm transition-all flex items-center justify-center gap-2"
                    >
                      <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 24 24"><path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.008 5.94H5.078z"/></svg>
                      Share on X
                    </button>
                 </div>
             )}
        </div>
      </div>
    </div>
  );
}
