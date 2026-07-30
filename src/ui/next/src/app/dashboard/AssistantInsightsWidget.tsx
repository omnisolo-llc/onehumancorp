"use client";

import React, { useEffect, useState } from "react";

export interface AssistantInsightItem {
  id: string;
  intent: string;
  customer_info?: { name?: string; department?: string };
  suggested_actions?: Array<{ action_type?: string; message?: string }> | any;
  status: string;
}

export function AssistantInsightsWidget() {
  const [items, setItems] = useState<AssistantInsightItem[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState("");

  const fetchInsights = async () => {
    try {
      const res = await fetch("/api/v1/ui/dashboard/daily-work");
      if (res.ok) {
        const data = await res.json();
        setItems(data.items?.slice(0, 3) || []); // Surface 1-3 AI-generated "Next Best Actions"
      } else {
        setError("Failed to load insights");
      }
    } catch (err) {
      setError("Error loading insights");
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchInsights();
  }, []);

  const handleApprove = async (id: string) => {
    try {
      // Optimistic UI update
      setItems((prev) => prev.filter((item) => item.id !== id));

      await fetch(`/api/v1/ui/dashboard/daily-work/action/${id}`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ action_status: "APPROVED" }),
      });
    } catch (err) {
      console.error("Failed to approve action", err);
      fetchInsights();
    }
  };

  if (loading) {
    return (
      <section className="mb-6 w-full col-span-full">
        <div className="rounded-[12px] bg-white/65 backdrop-blur-[30px] backdrop-saturate-[2.1] border border-white/40 dark:bg-[#16161a]/70 dark:backdrop-blur-[30px] dark:backdrop-saturate-[2.1] dark:border-white/10 shadow-sm p-5">
          <h2 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Assistant Insights</h2>
          <p className="text-sm text-gray-500">Loading your insights...</p>
        </div>
      </section>
    );
  }

  if (error || items.length === 0) {
    return null;
  }

  return (
    <section className="mb-6 w-full col-span-full">
      <div data-testid="assistant-insights-widget" className="rounded-[12px] bg-white/65 backdrop-blur-[30px] backdrop-saturate-[2.1] border border-white/40 dark:bg-[#16161a]/70 dark:backdrop-blur-[30px] dark:backdrop-saturate-[2.1] dark:border-white/10 shadow-sm p-5">
        <h2 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-4">Assistant Insights</h2>
        <div className="flex flex-col gap-4">
          {items.map((item) => {
            let description = "Action required";
            let title = item.intent || "Next Best Action";

            if (Array.isArray(item.suggested_actions) && item.suggested_actions.length > 0) {
                const actionDetails = item.suggested_actions[0];
                if (actionDetails.message) description = actionDetails.message;
                if (actionDetails.action_type) title = actionDetails.action_type;
            } else if (item.suggested_actions?.action?.proposed_content) {
                description = item.suggested_actions.action.proposed_content;
                title = item.suggested_actions.description || title;
            } else if (item.suggested_actions?.description) {
                description = item.suggested_actions.description;
            } else if (item.intent === "recent_order") {
                title = "Recent Order";
                description = `Order status: ${item.status}`;
            }

            return (
              <div key={item.id} data-testid={`insight-item-${item.id}`} className="bg-white/50 dark:bg-[#1A1A1D]/50 p-4 rounded-xl border border-gray-100 dark:border-gray-800 shadow-sm flex flex-col sm:flex-row gap-4 justify-between sm:items-center">
                <div>
                  <h3 className="text-md font-semibold text-[#1D1D1F] dark:text-[#F5F5F7]">{title}</h3>
                  <p className="text-sm text-gray-600 dark:text-gray-400 mt-1">{description}</p>
                  {item.customer_info?.name && (
                    <p className="text-xs text-gray-500 mt-1">For: {item.customer_info.name}</p>
                  )}
                </div>
                <div>
                  <button
                    onClick={() => handleApprove(item.id)}
                    data-testid={`approve-insight-${item.id}`}
                    className="w-full sm:w-auto min-h-[44px] px-6 rounded-[8px] bg-[#0066FF] text-white font-medium hover:bg-[#0052CC] transition-all shadow-md flex items-center justify-center whitespace-nowrap"
                  >
                    Approve & Send
                  </button>
                </div>
              </div>
            );
          })}
        </div>
      </div>
    </section>
  );
}
