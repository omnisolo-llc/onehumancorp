"use client";

import { useEffect, useState } from "react";
import { AppShell } from "../components/AppShell";

type AgentProposal = {
  id: string;
  department: string;
  title: string;
  description: string;
  actionLabel: string;
  draftContent?: string;
  type: 'urgent' | 'proposal';
  timestamp: string;
};

export default function UnifiedAgentFeed() {
  const [feed, setFeed] = useState<AgentProposal[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState("");
  const [expandedCard, setExpandedCard] = useState<string | null>(null);

  useEffect(() => {
    async function loadFeed() {
      try {
        const res = await fetch("/api/agents/feed");
        if (!res.ok) throw new Error("Failed to load agent feed");
        const data = await res.json();
        setFeed(data.feed || []);
      } catch (e: any) {
        setError(e?.message || "Failed to load feed");
      } finally {
        setLoading(false);
      }
    }
    loadFeed();
  }, []);

  const handleActionClick = (item: AgentProposal) => {
    if (item.draftContent && expandedCard !== item.id) {
      setExpandedCard(item.id);
    } else {
      // Simulate action completion and remove from feed
      setFeed((prev) => prev.filter((f) => f.id !== item.id));
      setExpandedCard(null);
    }
  };

  return (
    <AppShell
      title="Unified Agent Feed"
      subtitle="Your active AI Agent operations and proposals."
    >
      <div className="max-w-[375px] mx-auto w-full pb-8">
        {error && <div className="app-badge bad mb-4">{error}</div>}

        {loading ? (
          <div className="text-center py-8 text-gray-500">Loading feed...</div>
        ) : feed.length === 0 ? (
          <div className="app-empty">You're all caught up! No active proposals.</div>
        ) : (
          <div className="flex flex-col gap-4">
            {feed.map((item) => {
              const isExpanded = expandedCard === item.id;

              return (
                <div
                  key={item.id}
                  className={`app-panel transition-all duration-300 ${
                    item.type === 'urgent'
                      ? 'border-l-4 border-l-red-500/50 dark:border-l-red-400/50'
                      : 'border-l-4 border-l-indigo-500/50 dark:border-l-indigo-400/50'
                  }`}
                >
                  <div className="p-4 flex flex-col gap-3">
                    <div className="flex items-center justify-between">
                      <span className="app-badge font-medium">
                        {item.department} Agent
                      </span>
                      <span className="text-xs text-gray-500">{item.timestamp}</span>
                    </div>

                    <p className="text-base text-gray-900 dark:text-gray-100 font-medium">
                      {item.description}
                    </p>

                    {isExpanded && item.draftContent && (
                      <div className="mt-2 p-3 bg-gray-50 dark:bg-gray-800/50 rounded-lg text-sm text-gray-700 dark:text-gray-300 whitespace-pre-wrap border border-gray-200 dark:border-gray-700">
                        {item.draftContent}
                      </div>
                    )}

                    <button
                      onClick={() => handleActionClick(item)}
                      className={`mt-2 w-full min-h-[44px] flex items-center justify-center rounded-lg font-medium transition-colors ${
                        item.type === 'urgent'
                          ? 'bg-gray-900 text-white hover:bg-gray-800 dark:bg-white dark:text-gray-900 dark:hover:bg-gray-100'
                          : isExpanded
                            ? 'bg-indigo-600 text-white hover:bg-indigo-700'
                            : 'bg-indigo-50 text-indigo-700 hover:bg-indigo-100 dark:bg-indigo-900/30 dark:text-indigo-300 dark:hover:bg-indigo-900/50'
                      }`}
                    >
                      {isExpanded ? 'Approve & Send' : item.actionLabel}
                    </button>
                  </div>
                </div>
              );
            })}
          </div>
        )}
      </div>
    </AppShell>
  );
}