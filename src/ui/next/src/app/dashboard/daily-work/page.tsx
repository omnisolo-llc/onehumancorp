"use client";

import React, { useEffect, useState } from "react";
import { DailyWorkCard, DailyWorkItem } from "./DailyWorkCard";

export default function DailyWorkFeed() {
  const [items, setItems] = useState<DailyWorkItem[]>([]);
  const [loading, setLoading] = useState(true);

  const fetchFeed = async () => {
    try {
      const res = await fetch("/api/ui/dashboard/daily-work");
      if (res.ok) {
        const data = await res.json();
        setItems(data.items || []);
      }
    } catch (err) {
      console.error("Failed to fetch daily work feed", err);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchFeed();
  }, []);

  const handleAction = async (id: string, actionStatus: string) => {
    try {
      // Optimistic UI update
      setItems((prev) => prev.filter((item) => item.id !== id));

      await fetch(`/api/ui/dashboard/daily-work/action?id=${id}`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ action_status: actionStatus }),
      });
    } catch (err) {
      console.error("Failed to process action", err);
      // Optional: Re-fetch or revert on error
      fetchFeed();
    }
  };

  return (
    <div className="min-h-screen bg-[#F5F5F7] dark:bg-[#1D1D1F] p-4 sm:p-6 lg:p-8 font-inter">
      <div className="max-w-3xl mx-auto">
        <header className="mb-6">
          <h1 className="text-2xl font-semibold text-[#1D1D1F] dark:text-[#F5F5F7]">
            Today's Focus
          </h1>
          <p className="text-sm text-gray-500 mt-1">
            AI Autonomous Work Triage
          </p>
        </header>

        {loading ? (
          <div className="flex justify-center p-8">
            <span className="text-gray-500">Loading your work feed...</span>
          </div>
        ) : items.length === 0 ? (
          <div className="glassmorphism p-8 text-center border border-gray-200 dark:border-gray-800">
            <h2 className="text-lg font-medium text-[#1D1D1F] dark:text-[#F5F5F7]">
              You're all caught up!
            </h2>
            <p className="text-sm text-gray-500 mt-2">
              No new work items need your attention right now.
            </p>
          </div>
        ) : (
          <div className="flex flex-col gap-4">
            {items.map((item) => (
              <DailyWorkCard
                key={item.id}
                item={item}
                onApprove={(id) => handleAction(id, "APPROVED")}
                onDismiss={(id) => handleAction(id, "DISMISSED")}
              />
            ))}
          </div>
        )}
      </div>
    </div>
  );
}
