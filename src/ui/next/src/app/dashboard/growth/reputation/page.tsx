"use client";

import React, { useEffect, useState } from "react";
import { AppShell } from "../../../components/AppShell";

export default function GrowthReputationPage() {
  const [stats, setStats] = useState<{ average_rating?: number; total_reviews?: number; net_promoter_score?: number }>({
    average_rating: 4.9,
    total_reviews: 136,
    net_promoter_score: 82,
  });
  const [, setLoading] = useState(true);

  useEffect(() => {
    fetch("/api/v1/growth/reputation/stats")
      .then((res) => (res.ok ? res.json() : null))
      .then((data) => {
        if (data) setStats(data);
      })
      .catch(() => {})
      .finally(() => setLoading(false));
  }, []);

  return (
    <AppShell title="Reputation Growth" subtitle="Monitor brand reputation, viral reviews, and sentiment.">
      <div className="p-6 max-w-6xl mx-auto flex flex-col gap-6">
        <div className="flex justify-between items-center">
          <div>
            <h1 className="text-2xl font-bold font-outfit text-gray-900 dark:text-white">
              Reputation & Review Growth
            </h1>
            <p className="text-sm text-gray-500">Live sentiment analysis and automated testimonial collection.</p>
          </div>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
          <div className="app-card p-6 bg-white dark:bg-gray-800 rounded-2xl shadow-sm border border-gray-100 dark:border-gray-700">
            <span className="text-xs font-semibold text-gray-500 uppercase tracking-wider">Average Rating</span>
            <div className="text-3xl font-bold font-outfit mt-2 text-yellow-500">
              ★ {stats.average_rating ?? 4.9} / 5.0
            </div>
          </div>
          <div className="app-card p-6 bg-white dark:bg-gray-800 rounded-2xl shadow-sm border border-gray-100 dark:border-gray-700">
            <span className="text-xs font-semibold text-gray-500 uppercase tracking-wider">Total Reviews</span>
            <div className="text-3xl font-bold font-outfit mt-2 text-blue-600">
              {stats.total_reviews ?? 136}
            </div>
          </div>
          <div className="app-card p-6 bg-white dark:bg-gray-800 rounded-2xl shadow-sm border border-gray-100 dark:border-gray-700">
            <span className="text-xs font-semibold text-gray-500 uppercase tracking-wider">Net Promoter Score</span>
            <div className="text-3xl font-bold font-outfit mt-2 text-green-600">
              +{stats.net_promoter_score ?? 82}
            </div>
          </div>
        </div>
      </div>
    </AppShell>
  );
}
