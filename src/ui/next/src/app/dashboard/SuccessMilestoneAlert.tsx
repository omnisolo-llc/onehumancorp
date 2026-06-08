"use client";

import { useEffect, useState } from "react";

function tenantId() {
  if (typeof window === "undefined") return "default";
  return localStorage.getItem("tenant_id") || localStorage.getItem("tenant") || "default";
}

type Milestone = {
  id: string;
  title: string;
  description: string;
  reached: boolean;
};

export function SuccessMilestoneAlert() {
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
          const reachedMilestone = data.milestones?.find((m: Milestone) => m.reached);
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
    const inviteUrl = `${window.location.origin}/onboarding?ref=${tenantId()}`;
    try {
      await navigator.clipboard?.writeText(inviteUrl);
      setCopied(true);
      setTimeout(() => setCopied(false), 3000);
    } catch {
      // ignore
    }
  };

  const getIcon = (id: string) => {
    switch (id) {
        case 'first_sale': return '🎉';
        case '10th_order': return '📈';
        case '100_visitors': return '🚀';
        case '5_referrals': return '🤝';
        case 'revenue_1k': return '💰';
        default: return '✨';
    }
  };

  const [shareTarget, setShareTarget] = useState('/onboarding?ref=milestone');
  useEffect(() => {
    setShareTarget(`${window.location.origin}/onboarding?ref=milestone`);
  }, []);

  const shareText = `I just hit a huge business milestone (${milestone.title}) using OHC! Launch your own store today: ${shareTarget}`;

  return (
    <div className="mb-6 glassmorphism p-6 rounded-[24px] border border-indigo-200/50 dark:border-indigo-800/30 shadow-xl bg-gradient-to-br from-indigo-50/80 via-white/80 to-purple-50/80 dark:from-indigo-950/40 dark:via-gray-900/60 dark:to-purple-900/40 relative overflow-hidden group" data-testid="success-milestone-alert">
      {/* Decorative background elements */}
      <div className="absolute -top-10 -right-10 w-32 h-32 bg-indigo-500/10 rounded-full blur-3xl pointer-events-none"></div>
      <div className="absolute -bottom-10 -left-10 w-32 h-32 bg-purple-500/10 rounded-full blur-3xl pointer-events-none"></div>

      <div className="flex flex-col lg:flex-row gap-6 items-center justify-between relative z-10">
        <div className="flex flex-col sm:flex-row items-center sm:items-start text-center sm:text-left gap-5">
          <div className="relative">
            <div className="absolute inset-0 bg-indigo-500 blur opacity-20 rounded-full group-hover:opacity-40 transition-opacity"></div>
            <div className="text-5xl bg-white dark:bg-gray-800 w-20 h-20 rounded-full flex items-center justify-center shadow-[0_8px_30px_rgb(0,0,0,0.12)] border border-indigo-100 dark:border-indigo-800 relative z-10 transform group-hover:scale-110 transition-transform duration-300">
              {getIcon(milestone.id)}
            </div>
          </div>
          <div>
            <div className="inline-flex items-center gap-1.5 px-3 py-1 rounded-full bg-indigo-100 dark:bg-indigo-900/50 text-indigo-700 dark:text-indigo-300 text-xs font-bold uppercase tracking-wider mb-2 border border-indigo-200 dark:border-indigo-800">
              <span className="w-2 h-2 rounded-full bg-indigo-500 animate-pulse"></span>
              Milestone Unlocked
            </div>
            <h3 className="text-2xl sm:text-3xl font-bold font-outfit text-gray-900 dark:text-white leading-tight mb-1">
              {milestone.title}
            </h3>
            <p className="text-gray-600 dark:text-gray-300 text-sm sm:text-base max-w-lg">
              {milestone.description}
            </p>
          </div>
        </div>

        <div className="flex flex-col w-full lg:w-auto gap-3 shrink-0 mt-4 lg:mt-0">
            <div className="flex flex-col sm:flex-row gap-2">
                <button
                onClick={handleShare}
                className={`w-full sm:w-auto px-6 py-3 rounded-xl font-bold shadow-md transition-all flex items-center justify-center gap-2 ${copied ? 'bg-green-100 text-green-700 border border-green-200' : 'bg-white dark:bg-gray-800 text-gray-800 dark:text-gray-200 border border-gray-200 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-700'}`}
                data-testid="milestone-share-btn"
                >
                <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8 5H6a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2v-1M8 5a2 2 0 002 2h2a2 2 0 002-2M8 5a2 2 0 012-2h2a2 2 0 012 2m0 0h2a2 2 0 012 2v3m2 4H10m0 0l3-3m-3 3l3 3" /></svg>
                {copied ? "Copied Link!" : "Copy Link"}
                </button>
                <a
                href={`https://twitter.com/intent/tweet?text=${encodeURIComponent(shareText)}`}
                target="_blank"
                rel="noopener noreferrer"
                className="w-full sm:w-auto px-6 py-3 bg-black hover:bg-gray-800 text-white rounded-xl font-bold shadow-lg transition-all flex items-center justify-center gap-2 hover:-translate-y-0.5"
                >
                <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 24 24"><path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.008 5.94H5.078z"/></svg>
                Share on X
                </a>
            </div>
            <a
                href="/milestones"
                className="w-full px-6 py-3 bg-gradient-to-r from-indigo-600 to-purple-600 hover:from-indigo-700 hover:to-purple-700 text-white rounded-xl font-bold shadow-lg transition-all flex items-center justify-center gap-2 hover:-translate-y-0.5 border border-indigo-500/50"
            >
                <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4" /></svg>
                Generate Share Card
            </a>
        </div>
      </div>
    </div>
  );
}
