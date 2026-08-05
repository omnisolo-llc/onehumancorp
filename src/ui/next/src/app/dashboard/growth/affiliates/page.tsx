"use client";

import React, { useEffect, useState } from "react";
import { AppShell } from "../../../../components/AppShell";

export default function AffiliateDashboard() {
  const [stats, setStats] = useState<{ total_affiliates: number, total_commission_cents: number } | null>(null);
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    Promise.all([
      fetch("/api/v1/growth/affiliate/stats").then(res => res.ok ? res.json() : null),
    ]).then(([affiliatesData]) => {
      setStats(affiliatesData || { total_affiliates: 0, total_commission_cents: 0 });
      setIsLoading(false);
    }).catch(err => {
      console.error("Failed to fetch affiliate stats", err);
      setIsLoading(false);
    });
  }, []);

  const commissionDollars = stats ? (stats.total_commission_cents / 100).toFixed(2) : "0.00";

  return (
    <AppShell title="Growth - Affiliate" activePath="/dashboard/growth/affiliates">
      <div className="p-6 max-w-7xl mx-auto w-full space-y-6">
        <h1 className="text-3xl font-bold font-outfit text-[#1D1D1F] dark:text-white tracking-tight">
          Affiliate
        </h1>
        <p className="text-gray-600 dark:text-gray-400">
          Track and manage your affiliate marketing performance in real-time.
        </p>

        {isLoading ? (
          <div className="animate-pulse flex flex-col gap-3">
            <div className="h-6 bg-gray-200 rounded w-1/3"></div>
            <div className="h-4 bg-gray-200 rounded w-1/2"></div>
          </div>
        ) : (
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div className="glassmorphism p-4 rounded-xl border border-gray-100 shadow-sm text-center">
              <div className="text-2xl font-bold font-outfit text-gray-900">{stats?.total_affiliates}</div>
              <div className="text-xs text-gray-500 font-medium uppercase tracking-wide">Active Affiliates</div>
            </div>
            <div className="glassmorphism p-4 rounded-xl border border-gray-100 shadow-sm text-center">
              <div className="text-2xl font-bold font-outfit text-gray-900">${commissionDollars}</div>
              <div className="text-xs text-gray-500 font-medium uppercase tracking-wide">Paid Commissions</div>
            </div>
          </div>
        )}
      </div>
    </AppShell>
  );
}
