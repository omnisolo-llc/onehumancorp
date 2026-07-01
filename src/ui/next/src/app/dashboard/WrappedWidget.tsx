"use client";

import React, { useState, useEffect } from "react";
import { WalkthroughTarget } from "../../components/Walkthrough";

type WrappedStats = {
  totalSales: string;
  totalOrders: number;
  newCustomers: number;
  topProduct: string;
  aiHoursSaved: number;
};

type WrappedData = {
  year: number;
  title: string;
  subtitle: string;
  stats: WrappedStats;
  shareText: string;
  error?: string;
};

export function WrappedWidget() {
  const [data, setData] = useState<WrappedData | null>(null);
  const [isShared, setIsShared] = useState(false);
  const [tenantId, setTenantId] = useState("my-store");
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    let currentTenant = "my-store";
    if (typeof localStorage !== "undefined") {
      currentTenant = localStorage.getItem("tenant") || localStorage.getItem("tenant_id") || "my-store";
      setTenantId(currentTenant);
    }

    fetch(`/api/v1/growth/wrapped?tenant_id=${currentTenant}`)
      .then(res => res.json())
      .then(result => {
        if (!result.error) {
          setData(result);
        }
      })
      .catch(err => console.error("Failed to fetch wrapped data", err))
      .finally(() => setIsLoading(false));
  }, []);

  if (isLoading || !data || !data.stats) return null;

  const referralLink = `/onboarding?ref=${tenantId}&source=wrapped_share`;
  // Construct the full URL using window.location.origin if available
  const fullShareLink = typeof window !== 'undefined' ? `${window.location.origin}${referralLink}` : `https://ohc.app${referralLink}`;
  const fullShareText = `${data.shareText} ${fullShareLink}`;

  const handleShare = () => {
    if (typeof navigator !== "undefined" && navigator.clipboard) {
        navigator.clipboard.writeText(fullShareText);
        setIsShared(true);
        setTimeout(() => setIsShared(false), 3000);
    }
  };

  return (
    <WalkthroughTarget id="wrapped-summary">
    <section
        data-testid="wrapped-widget"
        className="mb-6 relative overflow-hidden rounded-[24px] border border-white/40 dark:border-white/10 shadow-xl bg-gradient-to-br from-pink-500/90 via-purple-500/90 to-indigo-600/90 dark:from-pink-900/80 dark:via-purple-900/80 dark:to-indigo-950/80 backdrop-blur-[40px] backdrop-saturate-[2] p-6 text-white group transform transition-all hover:scale-[1.01]"
    >
      {/* Decorative blurred blobs */}
      <div className="absolute -top-20 -right-20 w-64 h-64 bg-yellow-400/30 rounded-full blur-[80px] pointer-events-none"></div>
      <div className="absolute -bottom-20 -left-20 w-64 h-64 bg-blue-400/30 rounded-full blur-[80px] pointer-events-none"></div>

      <div className="relative z-10">
        <div className="flex justify-between items-start mb-6">
          <div>
            <div className="inline-flex items-center gap-1.5 px-3 py-1 rounded-full bg-white/20 text-white text-xs font-bold uppercase tracking-wider mb-3 border border-white/30 backdrop-blur-[30px] saturate-[210%]">
              <span className="w-2 h-2 rounded-full bg-pink-300 animate-pulse"></span>
              {data.year} Wrapped
            </div>
            <h2 className="text-3xl font-bold font-outfit leading-tight mb-2 text-white drop-shadow-md">
              {data.title}
            </h2>
            <p className="text-white/80 text-sm font-medium max-w-md">
              {data.subtitle}
            </p>
          </div>
          <div className="hidden sm:flex text-6xl opacity-20 transform rotate-12 group-hover:scale-110 transition-transform duration-500">
            📊
          </div>
        </div>

        {/* Stats Grid */}
        <div className="grid grid-cols-2 sm:grid-cols-4 gap-4 mb-6">
          <div className="bg-white/10 rounded-2xl p-4 border border-white/20 backdrop-blur-[30px] saturate-[210%]">
            <div className="text-xs text-white/70 font-semibold uppercase tracking-wider mb-1">Total Sales</div>
            <div className="text-2xl font-bold font-outfit text-white">{data.stats.totalSales}</div>
          </div>
          <div className="bg-white/10 rounded-2xl p-4 border border-white/20 backdrop-blur-[30px] saturate-[210%]">
            <div className="text-xs text-white/70 font-semibold uppercase tracking-wider mb-1">Orders</div>
            <div className="text-2xl font-bold font-outfit text-white">{data.stats.totalOrders}</div>
          </div>
          <div className="bg-white/10 rounded-2xl p-4 border border-white/20 backdrop-blur-[30px] saturate-[210%]">
            <div className="text-xs text-white/70 font-semibold uppercase tracking-wider mb-1">New Customers</div>
            <div className="text-2xl font-bold font-outfit text-white">{data.stats.newCustomers}</div>
          </div>
          <div className="bg-white/10 rounded-2xl p-4 border border-white/20 backdrop-blur-[30px] saturate-[210%] relative overflow-hidden">
             <div className="absolute top-0 right-0 w-16 h-16 bg-white/20 rounded-full blur-[20px] -mr-8 -mt-8"></div>
            <div className="text-xs text-white/70 font-semibold uppercase tracking-wider mb-1">AI Hours Saved</div>
            <div className="text-2xl font-bold font-outfit text-white flex items-center gap-2">
                {data.stats.aiHoursSaved}
                <span className="text-xl">🤖</span>
            </div>
          </div>
        </div>

        {/* Share Section */}
        <div className="flex flex-col sm:flex-row gap-3 mt-4">
          <button
            onClick={handleShare}
            data-testid="wrapped-share-btn"
            className={`flex-1 py-3 px-6 rounded-xl font-bold font-outfit text-sm transition-all flex items-center justify-center gap-2 ${
              isShared
                ? "bg-green-400 text-green-950 shadow-lg shadow-green-500/30"
                : "bg-white text-purple-900 hover:bg-gray-100 shadow-lg shadow-white/20 hover:-translate-y-0.5"
            }`}
          >
            {isShared ? (
              <><span>✓</span> Copied Link!</>
            ) : (
              <><span>🔗</span> Copy & Share Wrapped</>
            )}
          </button>

          <a
            href={`https://twitter.com/intent/tweet?text=${encodeURIComponent(fullShareText)}`}
            target="_blank"
            rel="noopener noreferrer"
            data-testid="wrapped-twitter-btn"
            className="flex-1 py-3 px-6 rounded-xl font-bold font-outfit text-sm bg-black/40 border border-white/20 text-white hover:bg-black/60 shadow-lg backdrop-blur-[30px] saturate-[210%] transition-all flex items-center justify-center gap-2 hover:-translate-y-0.5"
          >
            <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 24 24"><path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.008 5.94H5.078z"/></svg>
            Share on X
          </a>
        </div>
      </div>
    </section>
    </WalkthroughTarget>
  );
}
