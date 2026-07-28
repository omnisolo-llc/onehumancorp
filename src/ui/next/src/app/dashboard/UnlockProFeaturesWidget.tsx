"use client";

import React, { useState, useEffect } from "react";

export function UnlockProFeaturesWidget() {
  const [invitesSent, setInvitesSent] = useState(0);
  const [tenantId, setTenantId] = useState("default");
  const [isCopied, setIsCopied] = useState(false);
  const [isShared, setIsShared] = useState(false);
  const targetInvites = 3;

  useEffect(() => {
    let currentTenant = "default";
    if (typeof localStorage !== "undefined") {
      currentTenant = localStorage.getItem("business_display_name") || "default";
      setTenantId(currentTenant);
    }

    async function fetchMetrics() {
      try {
        const res = await fetch("/api/v1/growth/team-invites/aggregated-metrics");
        if (res.ok) {
          const data = await res.json();
          // Assuming the API returns a total_invites field, or active_referrals. We'll use total_invites for this widget.
          setInvitesSent(data.total_invites || 0);
        }
      } catch (err) {
        console.error("Failed to fetch invite metrics", err);
      }
    }

    fetchMetrics();
  }, []);

  const progress = Math.min((invitesSent / targetInvites) * 100, 100);
  const isUnlocked = invitesSent >= targetInvites;

  const referralLink = `/onboarding?ref=${tenantId}&source=unlock_pro`;
  const fullShareLink = typeof window !== "undefined" ? `${window.location.origin}${referralLink}` : `https://ohc.app${referralLink}`;
  const shareText = `I'm using OHC to manage my business operations. Join me and let's grow together! ${fullShareLink}`;

  const handleCopy = () => {
    if (navigator.clipboard) {
      navigator.clipboard.writeText(shareText);
      setIsCopied(true);
      setTimeout(() => setIsCopied(false), 3000);
    }
  };

  const handleShareX = () => {
    window.open(`https://twitter.com/intent/tweet?text=${encodeURIComponent(shareText)}`, "_blank");
    setIsShared(true);
    setTimeout(() => setIsShared(false), 3000);
  };

  return (
    <div
      data-testid="unlock-pro-features-widget"
      className="glassmorphism app-card p-6 mb-6 rounded-[12px] bg-white/65 backdrop-blur-[30px] backdrop-saturate-[2.1] border border-white/40 dark:bg-[#16161a]/70 dark:backdrop-blur-[30px] dark:backdrop-saturate-[2.1] dark:border-white/10 shadow-sm"
    >
      <div className="flex justify-between items-start mb-4">
        <div>
          <h3 className="font-bold text-gray-900 dark:text-white font-outfit text-xl flex items-center gap-2">
            <span className="text-2xl">✨</span> Referral Progress
          </h3>
          <p className="text-sm text-gray-600 dark:text-gray-300 mt-1">
            Track confirmed invites. Any associated reward must be verified by the billing service.
          </p>
        </div>
        <div className="bg-purple-100 dark:bg-purple-900/40 text-purple-700 dark:text-purple-300 text-xs font-bold px-3 py-1 rounded-full border border-purple-200 dark:border-purple-800">
          {invitesSent} / {targetInvites} Invites
        </div>
      </div>

      <div className="mb-6">
        <div className="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-3 mb-2 overflow-hidden border border-gray-300 dark:border-gray-600">
          <div
            className={`h-3 rounded-full transition-all duration-1000 ease-out ${
              isUnlocked ? "bg-gradient-to-r from-green-400 to-emerald-500" : "bg-gradient-to-r from-purple-500 to-indigo-600"
            }`}
            style={{ width: `${progress}%` }}
          ></div>
        </div>
        <div className="flex justify-between text-xs text-gray-500 dark:text-gray-400 font-medium px-1">
          <span>0</span>
          <span>{targetInvites}</span>
        </div>
      </div>

      {isUnlocked ? (
        <div className="bg-emerald-50 dark:bg-emerald-900/20 border border-emerald-200 dark:border-emerald-800/50 rounded-xl p-4 text-center">
          <div className="text-emerald-500 text-3xl mb-2">🎉</div>
          <h4 className="font-bold text-emerald-900 dark:text-emerald-300 mb-1">Invite target reached</h4>
          <p className="text-sm text-emerald-700 dark:text-emerald-400">
            Billing verification is required before any Pro entitlement is applied.
          </p>
        </div>
      ) : (
        <div className="flex flex-col sm:flex-row gap-3">
          <button
            onClick={handleCopy}
            className={`flex-1 min-h-[44px] py-2 px-4 rounded-xl font-bold font-outfit text-sm transition-all flex items-center justify-center gap-2 ${
              isCopied
                ? "bg-green-500 text-white shadow-md shadow-green-200 dark:shadow-none"
                : "bg-indigo-600 text-white hover:bg-indigo-700 shadow-md shadow-indigo-200 dark:shadow-none"
            }`}
          >
            {isCopied ? (
              <>
                <span>✓</span> Copied Link!
              </>
            ) : (
              <>
                <span>🔗</span> Copy Invite Link
              </>
            )}
          </button>
          <button
            onClick={handleShareX}
            className="flex-1 min-h-[44px] py-2 px-4 rounded-xl font-bold font-outfit text-sm bg-[#1DA1F2] text-white hover:bg-[#1a91da] shadow-md shadow-blue-200 dark:shadow-none transition-all flex items-center justify-center gap-2"
          >
            🐦 Share on X
          </button>
        </div>
      )}
    </div>
  );
}
