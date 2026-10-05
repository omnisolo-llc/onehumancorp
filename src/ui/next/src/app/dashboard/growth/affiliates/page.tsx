"use client";

import React, { useEffect, useState } from "react";
import { AppShell } from "../../../components/AppShell";

type AffiliateStats = {
  totalAffiliates: number | null;
  totalCommissionCents: number | null;
};

function readStats(value: unknown): AffiliateStats {
  if (!value || typeof value !== "object" || Array.isArray(value)) throw new Error("Invalid statistics");
  const data = value as Record<string, unknown>;
  if (data.error != null) throw new Error("Statistics unavailable");
  return {
    totalAffiliates: typeof data.total_affiliates === "number" && Number.isSafeInteger(data.total_affiliates) && data.total_affiliates >= 0
      ? data.total_affiliates : null,
    totalCommissionCents: typeof data.total_commission_cents === "number" && Number.isSafeInteger(data.total_commission_cents)
      ? data.total_commission_cents : null,
  };
}

export default function GrowthAffiliatesPage() {
  const [stats, setStats] = useState<AffiliateStats | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(false);
  const [attempt, setAttempt] = useState(0);

  useEffect(() => {
    const controller = new AbortController();
    setStats(null);
    setLoading(true);
    setError(false);
    fetch("/api/v1/growth/affiliate/stats", {
      credentials: "same-origin", cache: "no-store", signal: controller.signal,
    })
      .then(async (res) => {
        if (!res.ok) throw new Error("Statistics request failed");
        return readStats(await res.json());
      })
      .then((data) => { if (!controller.signal.aborted) setStats(data); })
      .catch(() => { if (!controller.signal.aborted) setError(true); })
      .finally(() => { if (!controller.signal.aborted) setLoading(false); });
    return () => controller.abort();
  }, [attempt]);

  const metric = (value: number | null | undefined) => loading ? "Loading…" : value == null ? "Unavailable" : value.toLocaleString("en-US");

  return (
    <AppShell title="Affiliate Growth" subtitle="Affiliate statistics for the current business.">
      <div className="p-6 max-w-6xl mx-auto flex flex-col gap-6">
        <div className="flex justify-between items-center">
          <div>
            <h1 className="text-2xl font-bold font-outfit text-gray-900 dark:text-white">
              Affiliate Stats & Partners
            </h1>
            <p className="text-sm text-gray-500">Recorded affiliate and commission totals. Active referral counts and payout status are unavailable.</p>
          </div>
        </div>

        {loading && <p role="status">Loading affiliate statistics…</p>}
        {error && (
          <div role="alert" className="rounded-xl border border-red-200 bg-red-50 p-4 text-red-800">
            <p>Affiliate statistics could not be loaded. No totals are available.</p>
            <button type="button" onClick={() => setAttempt((current) => current + 1)} className="mt-2 min-h-[44px] rounded-lg border border-red-300 px-4 font-semibold">
              Retry statistics
            </button>
          </div>
        )}

        <div className="grid grid-cols-1 md:grid-cols-3 gap-6" aria-busy={loading}>
          <div className="app-card p-6 bg-white dark:bg-gray-800 rounded-2xl shadow-sm border border-gray-100 dark:border-gray-700">
            <span className="text-xs font-semibold text-gray-500 uppercase tracking-wider">Total Affiliates</span>
            <div className="text-3xl font-bold font-outfit mt-2 text-blue-600">
              {metric(stats?.totalAffiliates)}
            </div>
          </div>
          <div className="app-card p-6 bg-white dark:bg-gray-800 rounded-2xl shadow-sm border border-gray-100 dark:border-gray-700">
            <span className="text-xs font-semibold text-gray-500 uppercase tracking-wider">Active Referrals</span>
            <div className="text-3xl font-bold font-outfit mt-2 text-green-600">
              {metric(null)}
            </div>
          </div>
          <div className="app-card p-6 bg-white dark:bg-gray-800 rounded-2xl shadow-sm border border-gray-100 dark:border-gray-700">
            <span className="text-xs font-semibold text-gray-500 uppercase tracking-wider">Commission Total (cents)</span>
            <div className="text-3xl font-bold font-outfit mt-2 text-indigo-600">
              {metric(stats?.totalCommissionCents)}
            </div>
          </div>
        </div>
      </div>
    </AppShell>
  );
}
