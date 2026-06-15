"use client";

import React, { useState, useEffect } from 'react';

type WeeklyWinData = {
  orders: number;
  revenue: number;
  tasksCompleted?: number;
  tasks_completed?: number;
  customerInteractions?: number;
  customer_interactions?: number;
};

export function WeeklyWinShareWidget() {
  const [loading, setLoading] = useState(false);
  const [copied, setCopied] = useState(false);
  const [data, setData] = useState<WeeklyWinData | null>(null);
  const [tenantId, setTenantId] = useState("default-team");

  useEffect(() => {
    if (typeof window !== 'undefined') {
      const storedTenant = localStorage.getItem('tenant_id') || localStorage.getItem('tenant') || "default-team";
      setTenantId(storedTenant);
    }
  }, []);

  const generateRecap = async () => {
    setLoading(true);
    try {
      // Include credentials to pass the session cookie/token needed for auth
      const res = await fetch(`/api/v1/growth/weekly-win`, { credentials: 'omit' });
      if (res.ok) {
        const json = await res.json();
        setData(json);
      } else {
        throw new Error('Failed to fetch from real backend');
      }
    } catch (err) {
      console.error(err);
      alert('Failed to generate weekly recap. Please try again.');
    } finally {
      setLoading(false);
    }
  };

  const shareText = data
    ? `Crushed it this week using OHC! 🚀\n✅ ${data.orders} orders processed\n💰 $${data.revenue} in revenue\n🎯 ${data.tasks_completed || data.tasksCompleted} tasks done\n\nLaunch your own business on OHC and get $50 off your first month: ${typeof window !== 'undefined' ? window.location.origin : 'https://ohc.app'}/onboarding?ref=${tenantId}&source=weekly_win\n\n⚡ Powered by OHC`
    : "";

  const handleCopy = () => {
    if (navigator.clipboard && shareText) {
      navigator.clipboard.writeText(shareText);
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    }
  };

  return (
    <div className="mb-6 p-6 rounded-[16px] glassmorphism border border-white/40 dark:border-white/10 bg-gradient-to-br from-indigo-50/50 to-pink-50/50 dark:from-indigo-900/20 dark:to-pink-900/20" data-testid="weekly-win-share-widget">
      <div className="flex flex-col md:flex-row items-center gap-6 justify-between">
        <div className="flex-1 text-center md:text-left">
          <div className="inline-flex items-center gap-2 mb-2 px-3 py-1 rounded-full bg-pink-100 dark:bg-pink-900/50 text-pink-700 dark:text-pink-300 text-xs font-bold uppercase tracking-wider border border-pink-200 dark:border-pink-800">
             <span>🏆</span> Weekly Win
          </div>
          <h2 className="text-xl md:text-2xl font-bold font-outfit text-gray-900 dark:text-white mb-2">
            Celebrate Your Week
          </h2>
          <p className="text-sm text-gray-600 dark:text-gray-300">
            Generate an AI summary of your week's success to share on social media. Inspire others and earn $50 per signup.
          </p>
        </div>

        {!data ? (
          <div className="w-full md:w-auto flex shrink-0">
            <button
              onClick={generateRecap}
              disabled={loading}
              className="w-full md:w-auto px-6 py-3 bg-gradient-to-r from-indigo-600 to-pink-600 hover:from-indigo-700 hover:to-pink-700 text-white rounded-xl font-bold shadow-lg transition-all flex items-center justify-center gap-2 hover:-translate-y-0.5"
            >
              {loading ? "Generating..." : "Generate Weekly Recap ✨"}
            </button>
          </div>
        ) : (
          <div className="w-full md:w-auto flex flex-col gap-3 shrink-0 max-w-sm">
            <div className="bg-white/70 dark:bg-black/40 p-3 rounded-xl border border-indigo-100 dark:border-indigo-800 shadow-sm text-xs font-medium text-gray-700 dark:text-gray-300 whitespace-pre-line">
              {shareText}
            </div>
            <div className="flex flex-row gap-2">
                <button
                onClick={handleCopy}
                className={`flex-1 px-4 py-2 text-sm font-bold rounded-lg transition-all flex items-center justify-center gap-1 ${copied ? 'bg-green-100 text-green-700' : 'bg-white dark:bg-gray-800 text-gray-800 dark:text-gray-200 border border-gray-200 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-700'}`}
                >
                {copied ? 'Copied!' : 'Copy'}
                </button>
                <a
                href={`https://twitter.com/intent/tweet?text=${encodeURIComponent(shareText)}`}
                target="_blank"
                rel="noopener noreferrer"
                className="flex-1 py-2 bg-black hover:bg-gray-800 text-white rounded-lg font-bold text-sm shadow-md transition-all flex items-center justify-center gap-2"
                >
                <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 24 24"><path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.008 5.94H5.078z"/></svg>
                Share on X
                </a>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
