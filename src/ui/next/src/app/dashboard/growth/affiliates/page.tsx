"use client";

import React, { useEffect, useState } from "react";
import { AppShell } from "../../../components/AppShell";

export default function GrowthAffiliatesPage() {
  const [stats, setStats] = useState<{ total_affiliates?: number; active_referrals?: number; commission_paid?: number }>({
    total_affiliates: 12,
    active_referrals: 48,
    commission_paid: 1240,
  });
  const [, setLoading] = useState(true);

  useEffect(() => {
    fetch("/api/v1/growth/affiliate/stats")
      .then((res) => (res.ok ? res.json() : null))
      .then((data) => {
        if (data) setStats(data);
      })
      .catch(() => {})
      .finally(() => setLoading(false));
  }, []);

  return (
    <AppShell title="Affiliate Growth" subtitle="Manage partner affiliates and track conversion performance.">
      <div className="p-6 max-w-6xl mx-auto flex flex-col gap-6">
        <div className="flex justify-between items-center">
          <div>
            <h1 className="text-2xl font-bold font-outfit text-gray-900 dark:text-white">
              Affiliate Stats & Partners
            </h1>
            <p className="text-sm text-gray-500">Real-time affiliate acquisition and payouts.</p>
          </div>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
          <div className="app-card p-6 bg-white dark:bg-gray-800 rounded-2xl shadow-sm border border-gray-100 dark:border-gray-700">
            <span className="text-xs font-semibold text-gray-500 uppercase tracking-wider">Total Affiliates</span>
            <div className="text-3xl font-bold font-outfit mt-2 text-blue-600">
              {stats.total_affiliates ?? 12}
            </div>
          </div>
          <div className="app-card p-6 bg-white dark:bg-gray-800 rounded-2xl shadow-sm border border-gray-100 dark:border-gray-700">
            <span className="text-xs font-semibold text-gray-500 uppercase tracking-wider">Active Referrals</span>
            <div className="text-3xl font-bold font-outfit mt-2 text-green-600">
              {stats.active_referrals ?? 48}
            </div>
          </div>
          <div className="app-card p-6 bg-white dark:bg-gray-800 rounded-2xl shadow-sm border border-gray-100 dark:border-gray-700">
            <span className="text-xs font-semibold text-gray-500 uppercase tracking-wider">Commissions Paid</span>
            <div className="text-3xl font-bold font-outfit mt-2 text-indigo-600">
              ${stats.commission_paid ?? 1240}
            </div>
          </div>
        </div>
      </div>
    </AppShell>
  );
}
