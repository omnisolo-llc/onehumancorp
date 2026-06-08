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
    <section className="app-panel mb-6">
      <div className="app-panel-header">
        <div>
          <h2 className="app-panel-title">Viral Loop Performance</h2>
          <div className="app-list-subtitle">Track your referral program and team growth.</div>
        </div>
        <div className="flex items-center gap-2 px-3 py-1 bg-indigo-50 rounded-full border border-indigo-100">
          <span className="text-xs font-medium text-indigo-600">Active</span>
        </div>
      </div>
      <div className="app-panel-body">
        <div className="grid grid-cols-1 md:grid-cols-4 gap-6">
          <div className="ohc-hybrid-panel p-5 shadow-sm flex flex-col justify-between border border-gray-100 rounded-xl bg-white">
            <div className="text-sm font-medium mb-1 text-indigo-800">Invites Sent</div>
            <div className="text-3xl font-bold font-outfit text-indigo-900">{invitesSent}</div>
          </div>
          <div className="ohc-hybrid-panel p-5 shadow-sm flex flex-col justify-between border border-gray-100 rounded-xl bg-white">
            <div className="text-sm font-medium mb-1 text-indigo-800">Active Referrals</div>
            <div className="text-3xl font-bold font-outfit text-indigo-900">{activeReferrals}</div>
          </div>
          <div className="ohc-hybrid-panel p-5 shadow-sm flex flex-col justify-between border border-gray-100 rounded-xl bg-white">
            <div className="text-sm font-medium mb-1 text-indigo-800">Revenue from Referrals</div>
            <div className="text-3xl font-bold font-outfit text-indigo-900">${revenue.toFixed(2)}</div>
          </div>
          <div className="ohc-hybrid-panel p-5 shadow-sm flex flex-col justify-between border border-gray-100 rounded-xl bg-white">
            <div className="text-sm font-medium mb-1 text-indigo-800">Pending Rewards</div>
            <div className="text-3xl font-bold font-outfit text-indigo-900">${pendingRewards.toFixed(2)}</div>
          </div>
        </div>
        <div className="mt-4">
            <Link href="/referrals" className="app-button inline-flex">View Referral Details</Link>
        </div>
      </div>
    </section>
  );
}
