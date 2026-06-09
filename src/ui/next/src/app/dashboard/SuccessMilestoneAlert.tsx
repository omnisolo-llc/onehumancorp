"use client";

import { useEffect, useState } from "react";

function tenantId() {
  if (typeof window === "undefined") return "default";
  return localStorage.getItem("tenant_id") || localStorage.getItem("tenant") || "default";
}

import { useRouter } from "next/navigation";

type Milestone = {
  id: string;
  title: string;
  description: string;
  reached: boolean;
  reward_claimed: boolean;
  reward_type: string;
};

export function SuccessMilestoneAlert() {
  const router = useRouter();
  const [milestone, setMilestone] = useState<Milestone | null>(null);
  const [loading, setLoading] = useState(true);
  const [copied, setCopied] = useState(false);

  useEffect(() => {
    async function checkMilestone() {
      try {
        const tenant = encodeURIComponent(tenantId());
        const res = await fetch(`/api/v1/growth/milestones/check?tenant=${tenant}`);
        if (res.ok) {
          const data = await res.json();
          // Find first reached but unclaimed milestone
          const reachedMilestone = data.milestones?.find((m: Milestone) => m.reached && !m.reward_claimed);
          if (reachedMilestone) {
            setMilestone(reachedMilestone);
          }
        }
      } catch (e) {
        console.error("Failed to check milestones", e);
      } finally {
        setLoading(false);
      }
    }
    checkMilestone();
  }, []);

  if (loading || !milestone) {
    return null;
  }

  const handleShare = async () => {
    router.push(`/milestones?claim=${milestone?.id}`);
  };

  return (
    <div className="mb-6 ohc-glass p-6 rounded-[24px] border border-white/40 dark:border-white/10 shadow-xl animate-in fade-in slide-in-from-top-4 duration-700" data-testid="success-milestone-alert">
      <div className="flex flex-col md:flex-row gap-4 items-center justify-between">
        <div className="flex items-center gap-4">
          <div className="text-4xl bg-gradient-to-br from-indigo-100 to-purple-100 dark:from-indigo-900/30 dark:to-purple-900/30 w-16 h-16 rounded-full flex items-center justify-center shadow-inner">
            🚀
          </div>
          <div>
            <h3 className="text-2xl font-bold font-outfit text-gray-900 dark:text-white flex items-center gap-2">
              {milestone.title}
            </h3>
            <p className="text-gray-600 dark:text-gray-300 mt-1">
              {milestone.description}
            </p>
          </div>
        </div>
        <button
          onClick={handleShare}
          className="w-full md:w-auto px-8 py-4 bg-indigo-600 hover:bg-indigo-700 text-white rounded-2xl font-extrabold shadow-lg hover:shadow-2xl transition-all hover:-translate-y-1 active:scale-95 flex items-center justify-center gap-2 group"
          data-testid="milestone-share-btn"
        >
          Claim {milestone.reward_type}
          <svg className="w-5 h-5 group-hover:translate-x-1 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 7l5 5m0 0l-5 5m5-5H6" />
          </svg>
        </button>
      </div>
    </div>
  );
}
