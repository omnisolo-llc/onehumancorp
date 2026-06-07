"use client";

import { useEffect, useState } from "react";

interface AdvisoryReport {
  summary: string;
  actionable_suggestion?: string;
}

export function AdvisoryCard() {
  const [report, setReport] = useState<AdvisoryReport | null>(null);
  const [loading, setLoading] = useState(true);
  const [actionTriggered, setActionTriggered] = useState(false);

  useEffect(() => {
    async function fetchReport() {
      try {
        const token = document.cookie
          .split('; ')
          .find(row => row.startsWith('token='))
          ?.split('=')[1];

        const res = await fetch("/api/v1/advisory/insights", {
          headers: {
            "Content-Type": "application/json",
            ...(token && { Authorization: `Bearer ${token}` }),
          },
        });
        if (res.ok) {
          const data = await res.json();
          setReport(data);
        }
      } catch (err) {
        console.error("Failed to fetch advisory report", err);
      } finally {
        setLoading(false);
      }
    }
    fetchReport();
  }, []);

  const handleAction = async () => {
    setActionTriggered(true);
    try {
      const token = document.cookie
        .split('; ')
        .find(row => row.startsWith('token='))
        ?.split('=')[1];

      await fetch("/api/v1/advisory/action", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          ...(token && { Authorization: `Bearer ${token}` }),
        },
      });
    } catch (err) {
      console.error("Failed to trigger advisory action", err);
      setActionTriggered(false);
    }
  };

  if (loading) {
    return (
      <div className="w-full max-w-[375px] bg-white/10 backdrop-blur-[20px] saturate-200 p-6 rounded-2xl border border-white/20 animate-pulse">
        <div className="h-6 bg-white/20 rounded w-3/4 mb-4"></div>
        <div className="h-4 bg-white/20 rounded w-1/2"></div>
      </div>
    );
  }

  if (!report) {
    return null;
  }

  return (
    <div className="w-full max-w-[375px] bg-white/10 backdrop-blur-[20px] saturate-200 p-6 rounded-2xl border border-white/20 shadow-xl mb-6 font-['Outfit']">
      <div className="flex items-center gap-2 mb-4">
        <span className="text-2xl">📈</span>
        <h3 className="text-lg font-bold text-gray-900 dark:text-white">Weekly Business Health</h3>
      </div>

      <p className="text-sm text-gray-700 dark:text-gray-200 mb-6 font-['Inter']">
        {report.summary}
      </p>

      {report.actionable_suggestion && (
        <div className="bg-white/5 rounded-xl p-4 border border-white/10">
          <p className="text-sm text-gray-800 dark:text-gray-300 font-medium mb-3">
            {report.actionable_suggestion}
          </p>
          <button
            onClick={handleAction}
            disabled={actionTriggered}
            className="w-full h-[44px] bg-blue-600 hover:bg-blue-700 text-white font-semibold rounded-lg flex items-center justify-center transition-colors touch-manipulation disabled:opacity-50"
          >
            {actionTriggered ? "Action Dispatched ✓" : "Yes, draft it!"}
          </button>
        </div>
      )}
    </div>
  );
}
