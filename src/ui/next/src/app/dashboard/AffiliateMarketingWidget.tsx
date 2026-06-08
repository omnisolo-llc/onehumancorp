"use client";

import React, { useState, useEffect } from "react";
import Link from "next/link";

export default function AffiliateMarketingWidget() {
  const [stats, setStats] = useState({ total_affiliates: 0, total_commission_cents: 0 });
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    const fetchStats = async () => {
      try {
        const response = await fetch("/api/v1/growth/affiliate/stats");
        if (response.ok) {
          const data = await response.json();
          setStats(data);
        }
      } catch (error) {
        console.error("Failed to fetch affiliate stats", error);
      } finally {
        setIsLoading(false);
      }
    };

    fetchStats();
  }, []);

  const commissionDollars = (stats.total_commission_cents / 100).toFixed(2);

  return (
    <div className="app-card rounded-2xl p-6 shadow-sm border border-indigo-100 bg-gradient-to-br from-white to-indigo-50/30 font-inter">
      <div className="flex items-center justify-between mb-4">
        <h3 className="font-bold text-gray-900 font-outfit text-lg flex items-center gap-2">
          <span className="text-xl">💰</span> Viral Growth
        </h3>
      </div>

      {isLoading ? (
        <div className="animate-pulse flex flex-col gap-3">
          <div className="h-6 bg-gray-200 rounded w-1/3"></div>
          <div className="h-4 bg-gray-200 rounded w-1/2"></div>
        </div>
      ) : (
        <>
          <p className="text-sm text-gray-600 mb-6">
            Your affiliates are generating sales. Here is how your word-of-mouth engine is performing.
          </p>

          <div className="grid grid-cols-2 gap-4 mb-6">
            <div className="bg-white p-4 rounded-xl border border-gray-100 shadow-sm text-center">
              <div className="text-2xl font-bold font-outfit text-gray-900">{stats.total_affiliates}</div>
              <div className="text-xs text-gray-500 font-medium uppercase tracking-wide">Active Affiliates</div>
            </div>
            <div className="bg-white p-4 rounded-xl border border-gray-100 shadow-sm text-center">
              <div className="text-2xl font-bold font-outfit text-gray-900">${commissionDollars}</div>
              <div className="text-xs text-gray-500 font-medium uppercase tracking-wide">Paid Commissions</div>
            </div>
          </div>

          <Link href="/referrals" className="block w-full py-2.5 bg-indigo-600 hover:bg-indigo-700 text-white text-center rounded-lg text-sm font-semibold transition-colors">
            Manage Affiliates
          </Link>
        </>
      )}
    </div>
  );
}
