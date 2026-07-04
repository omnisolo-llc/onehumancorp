'use client';

import React, { useState, useEffect } from 'react';

// Models based on the API response structure from /api/agent-feed
interface AgentFeedItem {
  id: string;
  tenant_id: string;
  event_source: string;
  context_payload?: any;
  proposed_action?: any;
  lifecycle_state: string;
  created_at: string;
  updated_at?: string;
}

const AgentDraftView = ({ draft, onApprove, onEdit, onDismiss, isProcessing }: { draft: any; onApprove: () => void; onEdit: () => void; onDismiss: () => void; isProcessing: boolean }) => {
  return (
    <div className="p-4 pt-0 flex flex-col gap-3">
      {/* Draft specific context */}
      {draft && (
        <div className="bg-[#0066FF]/10 dark:bg-[#0066FF]/20 backdrop-blur-[30px] saturate-[210%] p-3 flex flex-col gap-2 rounded-xl mb-2">
          <div className="text-[11px] uppercase tracking-wider font-bold text-[#0066FF] dark:text-[#3388FF]">
            Proposed Action
          </div>
          <div className="proposed-action text-[13px] leading-relaxed text-gray-900 dark:text-white whitespace-pre-wrap break-words">
            {draft.message || draft.draft_reply || JSON.stringify(draft)}
          </div>
        </div>
      )}

      {/* Buttons optimized for mobile with 44x44px touch targets */}
      <div className="flex flex-col sm:flex-row gap-2 w-full">
        <button
          onClick={onApprove}
          disabled={isProcessing}
          className="w-full flex-1 min-h-[44px] px-4 bg-[#0066FF] text-white font-medium hover:bg-[#0052CC] transition-all duration-200 rounded-[12px] shadow-md flex items-center justify-center disabled:opacity-50"
          data-testid="feed-approve-btn"
        >
          {isProcessing ? "Processing..." : "Approve & Send"}
        </button>

        <div className="flex gap-2 w-full">
          <button
            onClick={onEdit}
            disabled={isProcessing}
            className="w-full flex-1 min-h-[44px] px-4 border border-gray-300 dark:border-gray-600 bg-white/50 dark:bg-black/50 backdrop-blur-[30px] saturate-[210%] text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-white/70 dark:hover:bg-gray-800 transition-all duration-200 rounded-[12px] shadow-sm flex items-center justify-center disabled:opacity-50"
            data-testid="feed-edit-btn"
          >
            Edit
          </button>

          <button
            onClick={onDismiss}
            disabled={isProcessing}
            className="w-full flex-1 min-h-[44px] px-4 border border-gray-300 dark:border-gray-600 bg-white/50 dark:bg-black/50 backdrop-blur-[30px] saturate-[210%] text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-white/70 dark:hover:bg-gray-800 transition-all duration-200 rounded-[12px] shadow-sm flex items-center justify-center disabled:opacity-50"
            data-testid="feed-dismiss-btn"
          >
            Dismiss
          </button>
        </div>
      </div>
    </div>
  );
};

const WorkItemCard = ({ item, onApprove, onEdit, onDismiss, isProcessing }: { item: AgentFeedItem; onApprove: (id: string) => void; onEdit: (id: string) => void; onDismiss: (id: string) => void; isProcessing: boolean }) => {
  return (
    <div
      data-testid={`triage-card-${item.id}`}
      className="glassmorphism bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] rounded-[24px] shadow-sm flex flex-col mb-4 overflow-hidden transition-all duration-300"
    >
      <div className="p-5 border-b border-[rgba(255,255,255,0.2)] bg-[rgba(255,255,255,0.4)] dark:bg-[rgba(22,22,26,0.5)] backdrop-blur-[30px] backdrop-saturate-[210%]">
        <div className="flex justify-between items-start mb-3">
          <span className="text-xs font-semibold uppercase tracking-wider text-blue-600 bg-blue-50 px-3 py-1.5 rounded-full">
            {item.event_source.replace(/_/g, ' ')}
          </span>
          <span className="text-xs text-gray-500 font-medium">
            {new Date(item.created_at).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })}
          </span>
        </div>
        <div className="text-[15px] font-medium text-gray-900 dark:text-white leading-snug break-words line-clamp-3 mb-2">
          {item.context_payload?.customer_message || item.context_payload?.summary || item.context_payload?.message || JSON.stringify(item.context_payload)}
        </div>
      </div>

      {item.proposed_action && (
        <AgentDraftView
          draft={item.proposed_action}
          onApprove={() => onApprove(item.id)}
          onEdit={() => onEdit(item.id)}
          onDismiss={() => onDismiss(item.id)}
          isProcessing={isProcessing}
        />
      )}
    </div>
  );
};

export default function UnifiedFeed() {
  const [feedItems, setFeedItems] = useState<AgentFeedItem[]>([]);
  const [loading, setLoading] = useState(true);
  const [processingId, setProcessingId] = useState<string | null>(null);

  const fetchFeed = async () => {
    try {
      const res = await fetch('/api/agent-feed');
      if (res.ok) {
        const data = await res.json();
        setFeedItems(data.items || []);
      }
    } catch (err) {
      console.error("Failed to fetch agent feed", err);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchFeed();
  }, []);

  const handleAction = async (id: string, actionStatus: string) => {
    setProcessingId(id);
    try {
      // Optimistic UI update
      setFeedItems((prev) => prev.filter((item) => item.id !== id));

      await fetch(`/api/agent-feed/${id}/state`, {
        method: "PUT",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ state: actionStatus }),
      });
    } catch (err) {
      console.error("Failed to process action", err);
      // Optional: Re-fetch or revert on error
      fetchFeed();
    } finally {
      setProcessingId(null);
    }
  };

  if (loading) {
    return (
      <div className="w-full max-w-[375px] mx-auto min-h-screen bg-[#F5F5F7] dark:bg-[#1D1D1F] flex items-center justify-center font-outfit">
        <div className="text-gray-500 font-medium">Loading feed...</div>
      </div>
    );
  }

  return (
    <div className="w-full max-w-[375px] mx-auto min-h-screen bg-[#F5F5F7] dark:bg-[#1D1D1F] flex flex-col font-outfit relative">
      <header className="bg-white/80 dark:bg-black/80 backdrop-blur-[30px] saturate-[210%] border-b border-[rgba(255,255,255,0.2)] p-4 sticky top-0 z-10">
        <h1 className="text-[22px] font-bold text-[#1D1D1F] dark:text-[#F5F5F7] tracking-tight">Today</h1>
        <p className="text-[13px] text-gray-500 font-medium">Unified Action Feed</p>
      </header>

      <main className="flex-1 overflow-y-auto p-4 flex flex-col gap-4">
        {feedItems.length === 0 ? (
          <div className="flex flex-col items-center justify-center py-12 px-4 text-center">
            <div className="w-16 h-16 bg-blue-50 dark:bg-blue-900/20 rounded-full flex items-center justify-center mb-4">
              <span className="text-2xl">✨</span>
            </div>
            <h2 className="text-[17px] font-semibold text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">All caught up!</h2>
            <p className="text-[14px] text-gray-500">Your AI assistant has handled all outstanding items.</p>
          </div>
        ) : (
          feedItems.map((item) => (
            <WorkItemCard
              key={item.id}
              item={item}
              onApprove={(id) => handleAction(id, 'APPROVED')}
              onEdit={(id) => {
                // In a real app, open a modal. For this test, just act as if it's handled.
                console.log(`Edit item ${id}`);
              }}
              onDismiss={(id) => handleAction(id, 'DISMISSED')}
              isProcessing={processingId === item.id}
            />
          ))
        )}
      </main>
    </div>
  );
}
