"use client";

import React, { useState, useEffect } from "react";
import { WithTooltip } from "../../components/TooltipRegistry";

export function SuccessMilestoneWidget() {
  const [milestone, setMilestone] = useState<{ title: string; subtitle: string; shareText: string; reward: string } | null>(null);
  const [isShared, setIsShared] = useState(false);
  const [tenantId, setTenantId] = useState("my-store");

  useEffect(() => {
    let currentTenant = "my-store";
    if (typeof localStorage !== "undefined") {
      currentTenant = localStorage.getItem("tenant") || "my-store";
      setTenantId(currentTenant);
    }

    fetch(`/api/v1/growth/milestone?tenant_id=${currentTenant}`)
      .then(res => res.json())
      .then(data => {
        if (!data.error) {
          setMilestone(data);
        }
      })
      .catch(err => console.error("Failed to fetch milestone", err));
  }, []);

  if (!milestone) return null;

  const referralLink = `/onboarding?ref=${tenantId}&source=milestone_share`;
  const fullShareText = `${milestone.shareText} https://ohc.app${referralLink}`;

  const handleShare = () => {
    navigator.clipboard.writeText(fullShareText);
    setIsShared(true);
    setTimeout(() => setIsShared(false), 3000);
  };

  return (
    <section className="app-panel mb-6 border-2 border-indigo-200 bg-gradient-to-br from-indigo-50 to-white shadow-lg transform transition-all hover:scale-[1.01]">
      <div className="app-panel-header border-b border-indigo-100 pb-4">
        <div>
          <h2 className="app-panel-title text-indigo-900 flex items-center gap-2">
            <span>🏆</span> {milestone.title}
          </h2>
          <div className="app-list-subtitle text-indigo-700 font-medium mt-1">{milestone.subtitle}</div>
        </div>
        <div className="flex items-center gap-2 px-4 py-1.5 bg-green-100 rounded-full border border-green-200 shadow-sm">
          <span className="text-sm font-bold text-green-700">{milestone.reward}</span>
        </div>
      </div>
      <div className="app-panel-body pt-5">
        <div className="bg-white/60 backdrop-blur-sm p-4 rounded-xl border border-indigo-50 mb-4">
          <p className="text-sm text-gray-700 italic">"{fullShareText}"</p>
        </div>

        <div className="flex flex-col sm:flex-row gap-3">
          <button
            onClick={handleShare}
            className={`flex-1 py-3 px-4 rounded-xl font-bold font-outfit text-sm transition-all flex items-center justify-center gap-2 ${
              isShared
                ? "bg-green-500 text-white shadow-md shadow-green-200"
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
            className="flex-1 py-3 px-4 rounded-xl font-bold font-outfit text-sm bg-[#1DA1F2] text-white hover:bg-[#1a91da] shadow-md shadow-blue-200 transition-all flex items-center justify-center gap-2"
          >
            🐦 Share on X
          </a>
        </div>
      </div>
    </section>
  );
}
