"use client";

import { useEffect, useState } from "react";

function tenantId() {
  if (typeof window === "undefined") return "default";
  return localStorage.getItem("tenant_id") || localStorage.getItem("tenant") || "default";
}

export function SuccessMilestoneAlert() {
  const [milestone, setMilestone] = useState<{ reached: boolean; type?: string; message?: string } | null>(null);
  const [loading, setLoading] = useState(true);
  const [copied, setCopied] = useState(false);

  useEffect(() => {
    async function checkMilestone() {
      try {
        const tenant = encodeURIComponent(tenantId());
        const res = await fetch(`/api/v1/growth/milestones/check?tenant_id=${tenant}`);
        if (res.ok) {
          const data = await res.json();
          setMilestone(data);
        } else {
          setMilestone({ reached: false });
        }
      } catch (e) {
        setMilestone({ reached: false });
      } finally {
        setLoading(false);
      }
    }
    checkMilestone();
  }, []);

  if (loading || !milestone || !milestone.reached) {
    return null;
  }

  const handleShare = async () => {
    const inviteUrl = `${window.location.origin}/onboarding?ref=${tenantId()}`;
    try {
      await navigator.clipboard?.writeText(inviteUrl);
      setCopied(true);
      setTimeout(() => setCopied(false), 3000);
    } catch {
      // ignore
    }
  };

  return (
    <div className="mb-6 glassmorphism p-6 rounded-[16px] border border-white/40 dark:border-white/10" data-testid="success-milestone-alert">
      <div className="flex flex-col md:flex-row gap-4 items-center justify-between">
        <div className="flex items-center gap-4">
          <div className="text-4xl bg-gradient-to-br from-indigo-100 to-purple-100 dark:from-indigo-900/30 dark:to-purple-900/30 w-16 h-16 rounded-full flex items-center justify-center shadow-inner">
            🚀
          </div>
          <div>
            <h3 className="text-2xl font-bold font-outfit text-gray-900 dark:text-white flex items-center gap-2">
              Milestone Reached!
            </h3>
            <p className="text-gray-600 dark:text-gray-300 mt-1">
              {milestone.message || "You've hit a new milestone!"}
            </p>
          </div>
        </div>
        <button
          onClick={handleShare}
          className="w-full md:w-auto px-6 py-3 bg-gradient-to-r from-indigo-600 to-purple-600 hover:from-indigo-700 hover:to-purple-700 text-white rounded-xl font-bold shadow-lg hover:shadow-xl transition-all hover:-translate-y-0.5"
          data-testid="milestone-share-btn"
        >
          {copied ? "Copied Link!" : "Generate Share Card"}
        </button>
      </div>
    </div>
  );
}
