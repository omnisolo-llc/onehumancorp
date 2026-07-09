"use client";

import React, { useState, useEffect } from "react";

export function ViralUpgradePaywallWidget({ tenantId = "default" }: { tenantId?: string }) {
  const [data, setData] = useState<{ progress: number; target: number } | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [copied, setCopied] = useState(false);

  useEffect(() => {
    let currentTenant = tenantId;
    if (typeof window !== "undefined") {
      const storedTenant = localStorage.getItem("tenant_id") || localStorage.getItem("tenant");
      if (storedTenant) {
        currentTenant = storedTenant;
      }
    }

    const fetchData = async () => {
      try {
        const response = await fetch(`/api/v1/growth/upgrade-paywall?tenant_id=${encodeURIComponent(currentTenant)}`);
        if (response.ok) {
          const result = await response.json();
          setData(result);
        }
      } catch (error) {
        console.error("Failed to fetch upgrade paywall status", error);
      } finally {
        setIsLoading(false);
      }
    };

    fetchData();
  }, [tenantId]);

  if (isLoading) {
    return (
      <div className="ohc-growth-card p-6 border border-indigo-100 bg-white/50 animate-pulse rounded-xl">
        <div className="h-6 bg-gray-200 rounded w-1/3 mb-4"></div>
        <div className="h-4 bg-gray-200 rounded w-1/2 mb-8"></div>
        <div className="h-10 bg-gray-200 rounded"></div>
      </div>
    );
  }

  if (!data) return null;

  const currentReferrals = data.progress;
  const target = data.target;
  const progressPercent = Math.min(100, Math.max(0, (currentReferrals / target) * 100));

  const referralLink = typeof window !== 'undefined' ? `${window.location.origin}/onboarding?ref=${tenantId}&source=upgrade_paywall` : `https://ohc.app/onboarding?ref=${tenantId}&source=upgrade_paywall`;

  const handleCopy = () => {
    if (typeof navigator !== "undefined" && navigator.clipboard) {
      navigator.clipboard.writeText(referralLink);
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    }
  };

  return (
    <div className="rounded-[12px] bg-gradient-to-br from-indigo-50 to-purple-50 dark:from-indigo-900/20 dark:to-purple-900/20 border border-indigo-200 dark:border-indigo-800 shadow-sm p-6 relative overflow-hidden group">
      <div className="absolute -top-12 -right-12 w-32 h-32 bg-indigo-400/20 rounded-full blur-[40px] pointer-events-none group-hover:bg-indigo-400/30 transition-colors"></div>

      <div className="relative z-10">
        <div className="flex items-center justify-between mb-2">
          <h3 className="text-xl font-bold font-outfit text-gray-900 dark:text-white flex items-center gap-2">
            <span className="text-2xl">🤖</span> Unlock AI Autopilot
          </h3>
          {currentReferrals >= target && (
            <div className="text-sm font-bold bg-green-100 dark:bg-green-900/50 text-green-700 dark:text-green-300 px-3 py-1 rounded-full border border-green-200 dark:border-green-800">
              Unlocked!
            </div>
          )}
        </div>

        <p className="text-sm text-gray-600 dark:text-gray-300 mb-6">
          Refer {target} business owners to unlock 30 days of our premium AI Autopilot feature for free. Let AI handle your customer support automatically!
        </p>

        <div className="mb-6">
          <div className="flex justify-between text-xs font-semibold mb-1">
            <span className="text-gray-500 dark:text-gray-400">Progress</span>
            <span className="text-indigo-600 dark:text-indigo-400">{currentReferrals} / {target}</span>
          </div>
          <div className="h-2.5 w-full bg-gray-200 dark:bg-gray-700 rounded-full overflow-hidden">
            <div
              className="h-full bg-gradient-to-r from-indigo-500 to-purple-500 transition-all duration-1000 ease-out"
              style={{ width: `${progressPercent}%` }}
            ></div>
          </div>
          {currentReferrals < target && (
             <div className="text-xs text-indigo-600 dark:text-indigo-400 mt-1.5 font-medium text-right">
               {target - currentReferrals} more to unlock
             </div>
          )}
        </div>

        {currentReferrals < target && (
           <div className="flex gap-2">
             <button
               onClick={handleCopy}
               className="flex-1 min-h-[44px] bg-indigo-600 hover:bg-indigo-700 text-white font-bold py-2.5 px-4 rounded-xl transition-all shadow-md shadow-indigo-200 dark:shadow-indigo-900/50 flex items-center justify-center gap-2"
             >
               {copied ? (
                 <><span>✓</span> Copied!</>
               ) : (
                 <><span>🔗</span> Copy Link</>
               )}
             </button>
           </div>
        )}
      </div>
    </div>
  );
}
