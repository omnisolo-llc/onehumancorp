"use client";

import React, { useEffect, useState } from "react";
import Link from "next/link";

export function ViralLoopPerformanceWidget() {
  const [invitesSent, setInvitesSent] = useState(0);
  const [activeReferrals, setActiveReferrals] = useState(0);
  const [revenue, setRevenue] = useState(0);
  const [pendingRewards, setPendingRewards] = useState(0);

  useEffect(() => {
    async function fetchMetrics() {
      try {
        const res = await fetch("/api/v1/growth/team-invites/aggregated-metrics");
        if (res.ok) {
          const data = await res.json();
          setInvitesSent(data.total_invites || 0);
          setActiveReferrals(data.metrics?.active_referrals || 0);
          setRevenue(data.metrics?.revenue || 0);
          setPendingRewards(data.metrics?.pending_rewards || 0);
        } else {
          setInvitesSent(0);
          setActiveReferrals(0);
          setRevenue(0);
          setPendingRewards(0);
        }
      } catch (err) {
        console.error("Failed to fetch viral loop metrics", err);
      }
    }

    fetchMetrics();
  }, []);

  return (
    <section className="glassmorphism p-6 border border-white/40 dark:border-white/10 bg-gradient-to-r from-blue-50/50 to-indigo-50/50 dark:from-blue-900/20 dark:to-indigo-900/20 mb-6">
      <div className="flex items-center justify-between mb-6">
        <div>
          <h2 className="text-2xl font-bold font-outfit text-gray-900 dark:text-white mb-1">Viral Loop Performance</h2>
          <div className="text-sm text-gray-600 dark:text-gray-300">Track your referral program and team growth.</div>
        </div>
        <div className="flex items-center gap-2 px-3 py-1 bg-white/50 dark:bg-black/20 rounded-full border border-white/40 dark:border-white/10">
          <span className="text-xs font-bold uppercase tracking-wider text-indigo-600 dark:text-indigo-400">Active Loop</span>
        </div>
      </div>
      <div>
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
          <div className="app-card bg-white/40 dark:bg-black/20 backdrop-blur-[30px] saturate-[210%] border border-white/30 dark:border-white/5 flex flex-col justify-between p-5 rounded-[12px] shadow-sm hover:shadow-md transition-shadow">
            <div className="text-sm font-medium mb-2 text-indigo-800 dark:text-indigo-300">Invites Sent</div>
            <div className="text-3xl font-bold font-outfit text-indigo-900 dark:text-indigo-100">{invitesSent}</div>
          </div>
          <div className="app-card bg-white/40 dark:bg-black/20 backdrop-blur-[30px] saturate-[210%] border border-white/30 dark:border-white/5 flex flex-col justify-between p-5 rounded-[12px] shadow-sm hover:shadow-md transition-shadow">
            <div className="text-sm font-medium mb-2 text-indigo-800 dark:text-indigo-300">Active Referrals</div>
            <div className="text-3xl font-bold font-outfit text-indigo-900 dark:text-indigo-100">{activeReferrals}</div>
          </div>
          <div className="app-card bg-white/40 dark:bg-black/20 backdrop-blur-[30px] saturate-[210%] border border-white/30 dark:border-white/5 flex flex-col justify-between p-5 rounded-[12px] shadow-sm hover:shadow-md transition-shadow">
            <div className="text-sm font-medium mb-2 text-indigo-800 dark:text-indigo-300">Revenue from Referrals</div>
            <div className="text-3xl font-bold font-outfit text-indigo-900 dark:text-indigo-100">${revenue.toFixed(2)}</div>
          </div>
          <div className="app-card bg-white/40 dark:bg-black/20 backdrop-blur-[30px] saturate-[210%] border border-white/30 dark:border-white/5 flex flex-col justify-between p-5 rounded-[12px] shadow-sm hover:shadow-md transition-shadow">
            <div className="text-sm font-medium mb-2 text-indigo-800 dark:text-indigo-300">Pending Rewards</div>
            <div className="text-3xl font-bold font-outfit text-indigo-900 dark:text-indigo-100">${pendingRewards.toFixed(2)}</div>
          </div>
        </div>
        <div className="mt-6 flex justify-end">
            <Link href="/referrals" className="app-button bg-indigo-600 hover:bg-indigo-700 text-white border-none py-2 px-6 rounded-lg text-sm font-semibold transition-colors min-h-[44px] flex items-center justify-center">View Referral Details</Link>
        </div>
      </div>
    </section>
  );
}
