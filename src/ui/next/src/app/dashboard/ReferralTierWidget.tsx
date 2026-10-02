"use client";

import React, { useState, useEffect } from 'react';

type ReferralTierData = {
  current_tier: string;
  next_tier?: string | null;
  referrals_needed_for_next?: number | null;
  total_conversions: number;
};

export function ReferralTierWidget() {
  const [data, setData] = useState<ReferralTierData>({
    current_tier: "Bronze",
    next_tier: "Silver",
    referrals_needed_for_next: 5,
    total_conversions: 0,
  });
  const [copied, setCopied] = useState(false);
  const [tenantId, setTenantId] = useState("e2e-tenant");

  useEffect(() => {
    if (typeof window !== "undefined") {
      const stored = localStorage.getItem("business_display_name") || "e2e-tenant";
      setTenantId(stored);

      fetch("/api/v1/growth/referrals/tier")
        .then((res) => {
          if (!res.ok) throw new Error("Failed to load tier");
          return res.json();
        })
        .then((json: ReferralTierData) => {
          if (json && json.current_tier) {
            setData(json);
          }
        })
        .catch(() => {
          // Keep deterministic fallback defaults
        });
    }
  }, []);

  const inviteLink = `https://cloud.omnisolo.co/join?ref=${tenantId}`;

  const handleCopy = () => {
    if (navigator.clipboard) {
      navigator.clipboard.writeText(inviteLink);
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    }
  };

  const handleShareX = () => {
    const text = `Start your business on OmniSolo OneHumanCorp! Use my referral link: ${inviteLink}`;
    window.open(`https://twitter.com/intent/tweet?text=${encodeURIComponent(text)}`, "_blank");
  };

  return (
    <div
      id="referral-tier-widget"
      data-testid="referral-tier-widget"
      data-voice-assistant-surface="glass"
      className="omnisolo-growth-card glassmorphism p-6 mb-6 rounded-2xl shadow-sm border border-indigo-100 dark:border-indigo-900/50 bg-gradient-to-br from-white to-indigo-50/30 dark:from-gray-900 dark:to-indigo-900/20"
    >
      <h3 className="text-xl font-bold font-outfit text-gray-900 dark:text-white mb-2">Referral Tier</h3>
      <p id="referral-tier-status" className="text-sm text-gray-700 dark:text-gray-300 mb-2">
        You are on the {data.current_tier} Tier.
      </p>
      <p id="referral-tier-progress" className="text-sm font-semibold text-indigo-700 dark:text-indigo-400 mb-4">
        Total Conversions: {data.total_conversions}
        {data.next_tier && data.referrals_needed_for_next ? (
          <>
            <br />
            Just {data.referrals_needed_for_next} more referrals needed for {data.next_tier}!
          </>
        ) : (
          <>
            <br />
            You reached the highest tier!
          </>
        )}
      </p>
      <div className="flex flex-col gap-3">
        <input
          id="referral-link-input"
          aria-label="Referral link"
          type="text"
          readOnly
          value={inviteLink}
          className="w-full px-4 py-2 rounded-lg bg-white/70 dark:bg-black/30 border border-gray-200 dark:border-gray-700 text-gray-800 dark:text-gray-200 text-sm"
        />
        <div className="flex gap-2">
          <button
            id="referral-tier-copy-btn"
            onClick={handleCopy}
            className="flex-1 py-2 px-4 bg-indigo-600 hover:bg-indigo-700 text-white font-medium rounded-lg text-sm transition-colors min-h-[44px]"
          >
            {copied ? "Copied!" : "Copy Link"}
          </button>
          <button
            id="referral-tier-share-x-btn"
            onClick={handleShareX}
            className="flex-1 py-2 px-4 bg-black hover:bg-gray-800 text-white font-medium rounded-lg text-sm transition-colors min-h-[44px]"
          >
            Share on X
          </button>
        </div>
      </div>
    </div>
  );
}
