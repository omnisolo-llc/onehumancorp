import React, { useState, useEffect } from "react";
import Link from "next/link";

interface Milestone {
  target: number;
  title: string;
  reward: string;
  reached: boolean;
}

interface MilestonesData {
  tenant_id: string;
  total_referrals: number;
  milestones: Milestone[];
}

export default function ReferralMilestonesWidget({
  tenantId = "default",
}: {
  tenantId?: string;
}) {
  const [data, setData] = useState<MilestonesData | null>(null);
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
        const response = await fetch(`/api/v1/growth/referrals/milestones/status?tenant_id=${encodeURIComponent(currentTenant)}`);
        if (response.ok) {
          const result = await response.json();
          setData(result);
        }
      } catch (error) {
        // Only log in development to avoid test noise
        if (process.env.NODE_ENV !== 'test' && process.env.CI !== "1") {
          console.error("Failed to fetch referral milestones", error);
        }
      } finally {
        setIsLoading(false);
      }
    };

    fetchData();
  }, [tenantId]);

  if (isLoading) {
    return (
      <div className="ohc-growth-card p-6 border border-indigo-100 bg-white/50 animate-pulse">
        <div className="h-6 bg-gray-200 rounded w-1/3 mb-4"></div>
        <div className="h-4 bg-gray-200 rounded w-1/2 mb-8"></div>
        <div className="space-y-3">
          <div className="h-10 bg-gray-200 rounded"></div>
          <div className="h-10 bg-gray-200 rounded"></div>
          <div className="h-10 bg-gray-200 rounded"></div>
        </div>
      </div>
    );
  }

  if (!data) return null;

  const currentReferrals = data.total_referrals;
  const milestones = data.milestones;

  // Find next milestone to calculate progress
  const nextMilestoneIndex = milestones ? milestones.findIndex(m => !m.reached) : -1;
  const nextMilestone = nextMilestoneIndex !== -1 ? milestones[nextMilestoneIndex] : null;
  const prevTarget = nextMilestoneIndex > 0 ? milestones[nextMilestoneIndex - 1].target : 0;

  let progressPercent = 100;
  if (nextMilestone) {
     const range = nextMilestone.target - prevTarget;
     const currentProgress = currentReferrals - prevTarget;
     progressPercent = Math.min(100, Math.max(0, (currentProgress / range) * 100));
  }

  const referralLink = typeof window !== 'undefined' ? `${window.location.origin}/onboarding?ref=${data.tenant_id}&source=milestone_widget` : `https://ohc.app/onboarding?ref=${data.tenant_id}&source=milestone_widget`;

  const handleCopy = () => {
    if (typeof navigator !== "undefined" && navigator.clipboard) {
      navigator.clipboard.writeText(referralLink);
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    }
  };

  return (
    <div className="ohc-growth-card flex flex-col p-6 backdrop-blur-[30px] saturate-[210%] bg-white/40 dark:bg-black/30 border border-white/20 dark:border-white/10 shadow-lg rounded-2xl relative overflow-hidden group transition-all hover:shadow-xl">
      <div className="absolute -top-24 -right-24 w-48 h-48 bg-pink-400/20 rounded-full blur-[60px] pointer-events-none group-hover:bg-pink-400/30 transition-colors"></div>
      <div className="absolute -bottom-24 -left-24 w-48 h-48 bg-indigo-400/20 rounded-full blur-[60px] pointer-events-none group-hover:bg-indigo-400/30 transition-colors"></div>

      <div className="relative z-10 flex flex-col h-full">
        <div className="flex items-center justify-between mb-2">
            <h3 className="text-xl font-bold font-outfit text-gray-900 dark:text-white flex items-center gap-2">
                <span className="text-2xl">🎯</span> Unlock Rewards
            </h3>
            <div className="text-sm font-bold bg-indigo-100 dark:bg-indigo-900/50 text-indigo-700 dark:text-indigo-300 px-3 py-1 rounded-full border border-indigo-200 dark:border-indigo-800">
                {currentReferrals} Referrals
            </div>
        </div>

        <p className="text-sm text-gray-600 dark:text-gray-300 mb-6">
            Share OHC with other business owners to unlock exclusive rewards and platform credits.
        </p>

        {nextMilestone && (
            <div className="mb-6">
                <div className="flex justify-between text-xs font-semibold mb-1">
                    <span className="text-gray-500 dark:text-gray-400">Progress to {nextMilestone.title}</span>
                    <span className="text-indigo-600 dark:text-indigo-400">{currentReferrals} / {nextMilestone.target}</span>
                </div>
                <div className="h-2.5 w-full bg-gray-200 dark:bg-gray-700 rounded-full overflow-hidden">
                    <div
                        className="h-full bg-gradient-to-r from-indigo-500 to-purple-500 transition-all duration-1000 ease-out"
                        style={{ width: `${progressPercent}%` }}
                    ></div>
                </div>
                <div className="text-xs text-indigo-600 dark:text-indigo-400 mt-1.5 font-medium text-right">
                    {nextMilestone.target - currentReferrals} more to get {nextMilestone.reward}
                </div>
            </div>
        )}

        <div className="space-y-3 mb-6 flex-1">
            {milestones && milestones.map((milestone, idx) => (
                <div
                    key={idx}
                    className={`flex items-center p-3 rounded-xl border ${milestone.reached ? 'bg-green-50 dark:bg-green-900/20 border-green-200 dark:border-green-800' : 'bg-white/50 dark:bg-black/20 border-gray-100 dark:border-gray-800 opacity-70'}`}
                >
                    <div className={`flex-shrink-0 w-8 h-8 rounded-full flex items-center justify-center mr-3 ${milestone.reached ? 'bg-green-100 dark:bg-green-800 text-green-600 dark:text-green-300' : 'bg-gray-100 dark:bg-gray-800 text-gray-400 dark:text-gray-500'}`}>
                        {milestone.reached ? '✓' : milestone.target}
                    </div>
                    <div className="flex-1">
                        <div className="text-sm font-bold text-gray-900 dark:text-white leading-tight">{milestone.title}</div>
                        <div className="text-xs font-semibold text-gray-500 dark:text-gray-400">{milestone.reward}</div>
                    </div>
                </div>
            ))}
        </div>

        <div className="mt-auto flex gap-2">
            <button
                onClick={handleCopy}
                className="flex-1 min-h-[44px] bg-indigo-600 hover:bg-indigo-700 text-white font-bold py-2.5 px-4 rounded-xl transition-all shadow-md shadow-indigo-200 dark:shadow-indigo-900/50 flex items-center justify-center gap-2"
            >
                {copied ? (
                    <><span>✓</span> Copied!</>
                ) : (
                    <><span>🔗</span> Copy Referral Link</>
                )}
            </button>
        </div>
      </div>
    </div>
  );
}
