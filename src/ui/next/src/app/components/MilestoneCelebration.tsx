"use client";

import React, { useState, useEffect } from 'react';

type Milestone = {
  id: string;
  title: string;
  description: string;
  reached: boolean;
  reached_at?: string;
};

export function MilestoneCelebration({ tenantId }: { tenantId: string }) {
  const [recentMilestone, setRecentMilestone] = useState<Milestone | null>(null);
  const [dismissed, setDismissed] = useState(false);
  const [copied, setCopied] = useState(false);

  useEffect(() => {
    async function checkMilestones() {
      try {
        const res = await fetch(`/api/v1/growth/milestones/check?tenant=${tenantId}`);
        if (res.ok) {
          const data = await res.json();
          const milestones: Milestone[] = data.milestones;

          // Find the most recent milestone reached in the last 24 hours
          const now = new Date();
          const oneDayAgo = new Date(now.getTime() - (24 * 60 * 60 * 1000));

          const recent = milestones
            .filter(m => m.reached && m.reached_at)
            .find(m => {
                // Use ISO 8601 compatible parsing
                const reachedAt = new Date(m.reached_at!.replace(' ', 'T'));
                return reachedAt > oneDayAgo;
            });

          if (recent) {
            setRecentMilestone(recent);
          }
        }
      } catch (err) {
        console.error("Failed to check milestones for celebration", err);
      }
    }

    checkMilestones();
  }, [tenantId]);

  if (!recentMilestone || dismissed) return null;

  const shareText = `I just reached a huge milestone on One Human Corp: ${recentMilestone.title}! Launch your own store today: ohc://join?ref=${tenantId}`;

  const copyToClipboard = () => {
    navigator.clipboard.writeText(shareText);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div className="mb-6 p-1 rounded-2xl bg-gradient-to-r from-indigo-500 via-purple-500 to-pink-500 animate-pulse-slow shadow-lg">
      <div className="bg-white dark:bg-gray-900 rounded-[14px] p-5 flex flex-col md:flex-row items-center justify-between gap-4">
        <div className="flex items-center gap-4 text-center md:text-left">
          <div className="text-4xl">🎉</div>
          <div>
            <h3 className="font-bold font-outfit text-gray-900 dark:text-white text-lg">Congratulations!</h3>
            <p className="text-sm text-gray-600 dark:text-gray-400">You just hit: <span className="font-semibold text-indigo-600 dark:text-indigo-400">{recentMilestone.title}</span></p>
          </div>
        </div>

        <div className="flex items-center gap-3 w-full md:w-auto">
          <button
            onClick={copyToClipboard}
            className={`flex-1 md:flex-none px-6 py-2 rounded-xl text-sm font-bold transition-all ${copied ? 'bg-green-100 text-green-700' : 'bg-indigo-600 text-white hover:bg-indigo-700 shadow-md'}`}
          >
            {copied ? 'Copied!' : 'Share Success'}
          </button>
          <button
            onClick={() => setDismissed(true)}
            className="p-2 text-gray-400 hover:text-gray-600 dark:hover:text-gray-200 transition-colors"
            aria-label="Dismiss"
          >
            <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" /></svg>
          </button>
        </div>
      </div>
    </div>
  );
}
