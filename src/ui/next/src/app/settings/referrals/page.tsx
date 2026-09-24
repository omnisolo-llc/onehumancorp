"use client";

import React, { useState } from "react";
import { AppShell } from "../../components/AppShell";

export default function SettingsReferralsPage() {
  const [referralLink, setReferralLink] = useState<string | null>(null);

  const handleGenerate = () => {
    const code = "ref_" + Math.random().toString(36).substring(2, 10);
    setReferralLink(`https://ohc.app/ref/${code}`);
  };

  return (
    <AppShell title="Referrals" subtitle="Manage your affiliate and referral program.">
      <div className="p-6 max-w-4xl mx-auto space-y-6">
        <h1 className="text-3xl font-bold font-outfit text-gray-900 dark:text-white">
          Referral Program
        </h1>

        <div className="p-6 bg-white dark:bg-gray-800 rounded-2xl shadow-sm border border-gray-100 dark:border-gray-700 space-y-4">
          <p className="text-sm text-gray-600 dark:text-gray-300">
            Invite fellow entrepreneurs and earn credit toward your Pro subscription.
          </p>

          <button
            onClick={handleGenerate}
            className="app-button primary px-6 py-2.5 bg-blue-600 text-white font-semibold rounded-xl hover:bg-blue-700 transition-colors"
          >
            Generate Referral Link
          </button>

          {referralLink && (
            <div className="p-4 bg-gray-50 dark:bg-gray-900 rounded-xl border border-gray-200 dark:border-gray-700">
              <span className="text-xs font-semibold text-gray-500 uppercase tracking-wider block mb-1">
                Your Link
              </span>
              <span className="font-mono text-sm text-blue-600 dark:text-blue-400 break-all">
                {referralLink}
              </span>
            </div>
          )}
        </div>
      </div>
    </AppShell>
  );
}
