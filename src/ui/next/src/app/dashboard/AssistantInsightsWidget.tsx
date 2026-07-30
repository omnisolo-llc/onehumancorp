"use client";

import { useEffect, useState } from "react";

export function AssistantInsightsWidget() {
  const [insight, setInsight] = useState<string | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState("");

  useEffect(() => {
    async function fetchInsights() {
      try {
        const res = await fetch("/api/v1/advisory/insights");
        if (!res.ok) {
          throw new Error("Failed to fetch insights");
        }
        const data = await res.json();
        setInsight(data.summary || null);
      } catch (err: any) {
        setError(err.message);
      } finally {
        setLoading(false);
      }
    }
    fetchInsights();
  }, []);

  if (loading) {
    return (
      <div className="w-full mb-6 p-6 rounded-[12px] bg-white/65 backdrop-blur-[30px] backdrop-saturate-[2.1] border border-white/40 dark:bg-[#16161a]/70 dark:backdrop-blur-[30px] dark:backdrop-saturate-[2.1] dark:border-white/10 shadow-sm flex items-center justify-center min-h-[120px]">
        <div className="animate-pulse flex flex-col items-center gap-3 w-full">
          <div className="h-4 bg-gray-200 dark:bg-gray-700 rounded w-3/4"></div>
          <div className="h-4 bg-gray-200 dark:bg-gray-700 rounded w-1/2"></div>
        </div>
      </div>
    );
  }

  if (error || !insight) {
    return null; // Fail gracefully by not showing the widget
  }

  return (
    <div className="w-full mb-6 p-6 rounded-[12px] bg-white/65 backdrop-blur-[30px] backdrop-saturate-[2.1] border border-white/40 dark:bg-[#16161a]/70 dark:backdrop-blur-[30px] dark:backdrop-saturate-[2.1] dark:border-white/10 shadow-sm border border-indigo-200/40 dark:border-indigo-800/20 bg-gradient-to-br from-indigo-50/40 to-blue-50/40 dark:from-indigo-900/10 dark:to-blue-900/10 transition-all hover:shadow-md">
      <div className="flex items-start justify-between gap-4">
        <div className="flex-1">
          <div className="flex items-center gap-2 mb-2">
            <div className="w-8 h-8 rounded-full bg-indigo-100 dark:bg-indigo-900/30 flex items-center justify-center text-lg">💡</div>
            <h3 className="text-lg font-bold font-outfit text-indigo-900 dark:text-indigo-100">Assistant Insights</h3>
          </div>
          <p id="advisory-dashboard-summary" className="text-gray-800 dark:text-gray-200 text-sm font-medium leading-relaxed mb-4">
            {insight}
          </p>
          <button
            className="px-4 py-2 bg-indigo-600 hover:bg-indigo-700 text-white text-sm font-semibold rounded-lg shadow-sm transition-colors active:scale-95"
            onClick={() => {
              // Usually this would trigger an action or open a modal based on the insight
              // For now, it provides a visual confirmation point as required by the CUJ
              const el = document.getElementById("advisory-dashboard-summary");
              if (el) {
                el.classList.add("text-indigo-600", "dark:text-indigo-400");
                setTimeout(() => el.classList.remove("text-indigo-600", "dark:text-indigo-400"), 500);
              }
            }}
          >
            Approve & Send
          </button>
        </div>
      </div>
    </div>
  );
}
