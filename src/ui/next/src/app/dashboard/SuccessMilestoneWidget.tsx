"use client";

import React, { useState, useEffect } from "react";

export function SuccessMilestoneWidget() {
  const [milestone, setMilestone] = useState<{ title: string; subtitle: string; shareText: string; reward: string } | null>(null);
  const [isShared, setIsShared] = useState(false);
  const [tenantId, setTenantId] = useState("default");

  useEffect(() => {
    let currentTenant = "default";
    let token = "";
    if (typeof window !== "undefined") {
      currentTenant = localStorage.getItem("tenant_id") || localStorage.getItem("tenant") || "default";
      token = localStorage.getItem("token") || "";
      setTenantId(currentTenant);
    }

    const headers: Record<string, string> = {};
    if (token) {
      headers["Authorization"] = `Bearer ${token}`;
    }

    fetch(`/api/v1/growth/milestone?tenant_id=${encodeURIComponent(currentTenant)}`, { headers })
      .then(res => res.json())
      .then(data => {
        if (data && !data.error && data.title) {
          setMilestone(data);
        }
      })
      .catch(err => console.error("Failed to fetch milestone", err));
  }, []);

  if (!milestone) return null;

  const referralLink = `/onboarding?ref=${tenantId}&source=milestone_share`;
  const fullShareText = `${milestone.shareText} https://ohc.app${referralLink}\n\n⚡ Powered by OHC`;

  const handleShare = () => {
    navigator.clipboard.writeText(fullShareText);
    setIsShared(true);
    setTimeout(() => setIsShared(false), 3000);
  };

  return (
    <section
      data-testid="success-milestone-widget"
      className="mb-6 shadow-lg transform transition-all hover:scale-[1.01] rounded-2xl p-6"
      style={{
        background: "rgba(255, 255, 255, 0.65)",
        backdropFilter: "blur(30px) saturate(210%)",
        WebkitBackdropFilter: "blur(30px) saturate(210%)",
        border: "1px solid rgba(255, 255, 255, 0.4)",
      }}
    >
      {/* For dark mode we would ideally use a dark theme selector, but inline styles override classes.
          To support both we will use standard classes that map to tokens. */}
      <style>{`
        @media (prefers-color-scheme: dark) {
          [data-testid="success-milestone-widget"] {
            background: rgba(22, 22, 26, 0.7) !important;
            border: 1px solid rgba(255, 255, 255, 0.1) !important;
          }
        }
      `}</style>
      <div className="app-panel-header border-b border-gray-200 dark:border-gray-800 pb-4 flex justify-between items-start">
        <div>
          <h2 className="app-panel-title text-gray-900 dark:text-[#F5F5F7] flex items-center gap-2 font-bold text-xl">
            <span>🏆</span> {milestone.title}
          </h2>
          <div className="app-list-subtitle text-gray-700 dark:text-gray-300 font-medium mt-1">{milestone.subtitle}</div>
        </div>
        <div className="flex items-center gap-2 px-4 py-1.5 bg-green-100 dark:bg-green-900/30 rounded-full border border-green-200 dark:border-green-800 shadow-sm">
          <span className="text-sm font-bold text-green-700 dark:text-green-400">{milestone.reward}</span>
        </div>
      </div>
      <div className="app-panel-body pt-5">
        <div className="bg-white dark:bg-black/20 backdrop-blur-[30px] saturate-[210%] p-4 rounded-xl border border-gray-100 dark:border-gray-800 mb-4">
          <p className="text-sm text-gray-700 dark:text-gray-300 italic">"{fullShareText}"</p>
        </div>

        <div className="flex flex-col sm:flex-row gap-3">
          <button
            onClick={handleShare}
            className={`flex-1 min-h-[44px] min-w-[44px] py-3 px-4 rounded-xl font-bold font-outfit text-sm transition-all flex items-center justify-center gap-2 ${
              isShared
                ? "bg-[#34C759] text-white shadow-md shadow-green-200"
                : "bg-indigo-600 text-white hover:bg-indigo-700 shadow-md shadow-indigo-200"
            }`}
          >
            {isShared ? (
              <><span>✓</span> Copied to Clipboard!</>
            ) : (
              <><span>🔗</span> Copy & Share to Unlock</>
            )}
          </button>

          <a
            href={`https://twitter.com/intent/tweet?text=${encodeURIComponent(fullShareText)}`}
            target="_blank"
            rel="noopener noreferrer"
            className="flex-1 min-h-[44px] min-w-[44px] py-3 px-4 rounded-xl font-bold font-outfit text-sm bg-[#1DA1F2] text-white hover:bg-[#1a91da] shadow-md shadow-blue-200 transition-all flex items-center justify-center gap-2"
          >
            🐦 Share on X
          </a>
        </div>
      </div>
    </section>
  );
}
