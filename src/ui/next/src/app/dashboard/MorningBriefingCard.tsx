import React, { useState, useEffect } from "react";
import Link from "next/link";

interface MorningBriefingCardProps {
  tenantId: string;
}

export default function MorningBriefingCard({ tenantId }: MorningBriefingCardProps) {
  const [briefing, setBriefing] = useState<{ summary: string; action_pills: string[] } | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    // In a real implementation, this would fetch from /api/agents/approvals
    // or listen to the unified agent feed to find the Morning Briefing draft.
    // We mock the fetch sequence here as defined by the CUJ.
    const fetchBriefing = async () => {
      setLoading(true);
      try {
        // Simulate delay for fetching from Decision Assistant
        await new Promise((resolve) => setTimeout(resolve, 800));
        setBriefing({
          summary: "Good morning! You have 3 custom cake deliveries today totaling $450, and 2 unanswered DMs.",
          action_pills: ["[Review 2 Unread DMs]", "[View Bookings]"]
        });
      } catch (err) {
        console.error("Failed to load morning briefing", err);
      } finally {
        setLoading(false);
      }
    };

    fetchBriefing();
  }, [tenantId]);

  if (loading) {
    return (
      <div className="glassmorphism p-6 rounded-[16px] border border-white/40 dark:border-white/10 shadow-sm animate-pulse mb-6">
        <div className="h-6 bg-gray-200 dark:bg-gray-700 rounded w-1/4 mb-4"></div>
        <div className="h-4 bg-gray-200 dark:bg-gray-700 rounded w-3/4"></div>
      </div>
    );
  }

  if (!briefing) return null;

  return (
    <div className="glassmorphism p-6 rounded-[16px] border border-indigo-100 dark:border-indigo-900/30 shadow-md bg-gradient-to-r from-indigo-50/50 to-white/50 dark:from-indigo-900/10 dark:to-[#1D1D1F]/50 mb-6">
      <div className="flex items-start gap-4">
        <div className="hidden sm:flex w-12 h-12 rounded-full bg-indigo-100 dark:bg-indigo-900/30 items-center justify-center text-2xl">
          🌅
        </div>
        <div className="flex-1">
          <h2 className="text-xl font-bold font-outfit text-indigo-900 dark:text-indigo-300 mb-2">Morning Briefing</h2>
          <p className="text-sm md:text-base text-gray-700 dark:text-gray-300 font-inter leading-relaxed mb-4">
            {briefing.summary}
          </p>
          <div className="flex flex-wrap gap-2">
            {briefing.action_pills.map((pill, idx) => (
              <button
                key={idx}
                className="px-4 py-2 bg-indigo-600 hover:bg-indigo-700 text-white text-sm font-medium rounded-full shadow-sm transition-transform active:scale-95 flex items-center justify-center"
              >
                {pill.replace(/[\[\]]/g, '')}
              </button>
            ))}
          </div>
        </div>
      </div>
    </div>
  );
}
