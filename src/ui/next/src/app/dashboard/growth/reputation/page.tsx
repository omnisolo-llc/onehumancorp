"use client";

import React, { useEffect, useState } from "react";
import { AppShell } from "../../../../components/AppShell";

export default function ReputationDashboard() {
  const [stats, setStats] = useState<{ total_reviews: number, average_rating: number } | null>(null);
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    Promise.all([
      // Real API endpoint integration for fetching reputation analytics data in parallel
      fetch("/api/v1/growth/reputation/stats").then(res => res.ok ? res.json() : null),
    ]).then(([reputationData]) => {
      setStats(reputationData || { total_reviews: 0, average_rating: 0 });
      setIsLoading(false);
    }).catch(err => {
      console.error("Failed to fetch reputation stats", err);
      setIsLoading(false);
    });
  }, []);

  const avgRatingStr = stats ? stats.average_rating.toFixed(1) : "0.0";

  return (
    <AppShell title="Growth - Reputation" activePath="/dashboard/growth/reputation">
      <div className="p-6 max-w-7xl mx-auto w-full space-y-6">
        <h1 className="text-3xl font-bold font-outfit text-[#1D1D1F] dark:text-white tracking-tight">
          Reputation
        </h1>
        <p className="text-gray-600 dark:text-gray-400">
          Monitor your brand reputation and customer reviews across platforms.
        </p>

        {isLoading ? (
          <div className="animate-pulse flex flex-col gap-3">
            <div className="h-6 bg-gray-200 rounded w-1/3"></div>
            <div className="h-4 bg-gray-200 rounded w-1/2"></div>
          </div>
        ) : (
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div className="glassmorphism p-4 rounded-xl border border-gray-100 shadow-sm text-center">
              <div className="text-2xl font-bold font-outfit text-gray-900">{stats?.total_reviews}</div>
              <div className="text-xs text-gray-500 font-medium uppercase tracking-wide">Recent Reviews</div>
            </div>
            <div className="glassmorphism p-4 rounded-xl border border-gray-100 shadow-sm text-center">
              <div className="text-2xl font-bold font-outfit text-gray-900">{avgRatingStr}</div>
              <div className="text-xs text-gray-500 font-medium uppercase tracking-wide">Average Rating</div>
            </div>
          </div>
        )}
      </div>
    </AppShell>
  );
}
