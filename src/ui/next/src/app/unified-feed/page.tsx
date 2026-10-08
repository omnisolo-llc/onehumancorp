"use client";


import React, { useState, useEffect, useMemo } from "react";
import { useFeedDecisions } from './useFeedDecisions';

interface WorkItem {
  id: string;
  source: string;
  payload: import('@/lib/agent-feed-types').ActionPayload;
  status: string;
}

interface AgentDraft {
  id: string;
  response: string;
  status: string;
  action_type?: string;
}

interface FeedItem {
  workItem: WorkItem;
  draft?: AgentDraft;
}

export default function UnifiedFeed() {
  const decisions = useFeedDecisions();

  const getSourceStyle = (source: string) => {
    const s = source.toLowerCase();
    if (s.includes("operation") || s.includes("ops")) {
      return {
        bg: "bg-green-100 dark:bg-green-900/30",
        text: "text-green-700 dark:text-green-400",
        icon: "🔧",
      };
    } else if (s.includes("market") || s.includes("promoter")) {
      return {
        bg: "bg-purple-100 dark:bg-purple-900/30",
        text: "text-purple-700 dark:text-purple-400",
        icon: "📈",
      };
    } else if (
      s.includes("customer") ||
      s.includes("instagram dm") ||
      s.includes("chat")
    ) {
      return {
        bg: "bg-blue-100 dark:bg-blue-900/30",
        text: "text-blue-700 dark:text-blue-400",
        icon: "💬",
      };
    } else if (
      s.includes("payment") ||
      s.includes("stripe") ||
      s.includes("billing")
    ) {
      return {
        bg: "bg-yellow-100 dark:bg-yellow-900/30",
        text: "text-yellow-700 dark:text-yellow-400",
        icon: "💰",
      };
    }
    return {
      bg: "bg-gray-100 dark:bg-gray-800",
      text: "text-gray-700 dark:text-gray-300",
      icon: "🔔",
    };
  };
  const [editingId, setEditingId] = useState<string | null>(null);
  const [editDraftText, setEditDraftText] = useState("");
  const { loading, processingId } = decisions;
  useEffect(() => { setEditingId(null); setEditDraftText(""); }, [decisions.revision]);
  const feedItems = useMemo(() => {
      const rawItems = decisions.items;
      const pendingItems = rawItems.filter(
        (i) =>
          decisions.notices.get(i.id)?.kind === 'unknown' || (i.lifecycle_state !== "APPROVED" && i.lifecycle_state !== "DISMISSED"),
      );

      const mappedItems: FeedItem[] = pendingItems.map((raw) => {
        return {
          workItem: {
            id: raw.id,
            source: raw.event_source || "Unknown",
            payload: raw.context_payload,
            status: raw.lifecycle_state,
          },
          draft: raw.proposed_action
            ? {
                id: raw.id,
                response:
                  raw.proposed_action.draft_reply ||
                  raw.proposed_action.summary ||
                  JSON.stringify(raw.proposed_action),
                status: "draft",
                action_type:
                  raw.proposed_action.action_type ||
                  raw.proposed_action.feature_type,
              }
            : undefined,
        };
      });

      // Sort by urgency/priority
      mappedItems.sort((a, b) => {
        // Higher priority sources first
        const isPaymentA =
          a.workItem.source.toLowerCase().includes("payment") ||
          a.workItem.source.toLowerCase().includes("stripe");
        const isPaymentB =
          b.workItem.source.toLowerCase().includes("payment") ||
          b.workItem.source.toLowerCase().includes("stripe");

        if (isPaymentA && !isPaymentB) return -1;
        if (!isPaymentA && isPaymentB) return 1;

        // Then unread/urgent messages
        const isUrgentA =
          a.workItem.payload?.priority === "high" ||
          a.workItem.payload?.priority === "urgent";
        const isUrgentB =
          b.workItem.payload?.priority === "high" ||
          b.workItem.payload?.priority === "urgent";

        if (isUrgentA && !isUrgentB) return -1;
        if (!isUrgentA && isUrgentB) return 1;

        return 0; // Maintain order otherwise
      });

      return mappedItems;
  }, [decisions.items, decisions.notices]);

  const handleAction = (itemId: string, action: 'APPROVED' | 'DISMISSED', editedPayload?: string) => {
    // The hook applies acknowledged rows inside its owner fence. Do not apply
    // editor state later from a promise belonging to an older owner or view.
    void decisions.decide(itemId, action, editedPayload);
  };

  const handleApprove = (itemId: string) => handleAction(itemId, "APPROVED");
  const handleDismiss = (itemId: string) => handleAction(itemId, "DISMISSED");

  const handleEdit = (item: FeedItem) => {
    if (decisions.blocked(item.workItem.id)) return;
    setEditingId(item.workItem.id);
    setEditDraftText(item.draft?.response || "");
  };

  const handleSaveEditAndApprove = (itemId: string) => {
    void handleAction(itemId, "APPROVED", editDraftText);
  };

  if (loading) return <div className="p-4 text-center">Loading feed...</div>;

  return (
    <div className="w-full max-w-[375px] mx-auto min-h-screen bg-[#F5F5F7] dark:bg-[#1D1D1F] flex flex-col text-[#1D1D1F] dark:text-[#F5F5F7]">
      <a
        href="/promoter"
        className="block w-full text-center bg-purple-500 text-white py-2 rounded mt-4"
        data-testid="promoter-link"
      >
        Go to Promoter Agent
      </a>
      <header className="bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] border-b border-gray-200/50 dark:border-gray-800/50 p-4 sticky top-0 z-10 flex justify-between items-center" style={{ backdropFilter: 'blur(30px) saturate(210%)' }}>
        <h1 className="text-xl font-bold tracking-tight">Today</h1>
      </header>

      {decisions.error && <p role="alert" className="p-4 text-red-700">{decisions.error}</p>}
      {[...decisions.notices].map(([id, notice]) => <p key={id} role={notice.kind === 'unknown' || notice.kind === 'rejected' ? 'alert' : 'status'} aria-label="Decision status" className="p-4">{notice.message}</p>)}
      <p className="p-4 text-sm">Approving records a decision. Execution or delivery requires a separate verified outcome.</p>
      <button type="button" disabled={decisions.refreshing || processingId !== null} onClick={() => void decisions.refresh()}>Refresh recorded decisions</button>
      <main
        className="flex-1 overflow-y-auto p-4 space-y-4"
        data-testid="agent-feed"
      >
        {feedItems.length === 0 && !decisions.error && decisions.ready && ![...decisions.notices.values()].some(notice => notice.kind === 'unknown' || notice.kind === 'pending') ? (
          <div
            className="text-center text-gray-500 py-8 flex flex-col items-center gap-3 glassmorphism shadow-sm opacity-90"
            data-testid="triage-feed-empty"
          >
            <div className="text-3xl mb-2">✨</div>
            <h3 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7]">
              All caught up!
            </h3>
          </div>
        ) : (
          feedItems.map((item) => (
            <div
              key={item.workItem.id}
              className="w-full glassmorphism bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] rounded-[16px] shadow-sm overflow-hidden transition-all duration-300"
              style={{ backdropFilter: 'blur(30px) saturate(210%)' }}
              data-testid={`triage-card-${item.workItem.id}`}
            >
              <div
                className="p-4 pb-3 border-b border-gray-100/50 dark:border-gray-800/50 flex justify-between items-center cursor-pointer"
                onClick={() => {
                  if (!item.draft && editingId !== item.workItem.id) {
                     handleEdit(item);
                  }
                }}
                data-testid={`triage-card-header-${item.workItem.id}`}
                aria-expanded={editingId === item.workItem.id || !!item.draft ? 'true' : 'false'}
                aria-controls={`triage-details-${item.workItem.id}`}
              >
                <div className="flex items-center gap-2">
                  <span
                    className={`text-[10px] font-bold uppercase tracking-widest px-2.5 py-1 rounded-full flex items-center gap-1 ${getSourceStyle(item.workItem.source).bg} ${getSourceStyle(item.workItem.source).text}`}
                  >
                    <span>{getSourceStyle(item.workItem.source).icon}</span>
                    {item.workItem.source}
                  </span>
                </div>
                <span className="text-[11px] font-medium text-gray-400">
                  Just now
                </span>
              </div>
              <div className="p-4">
                <p className="text-[14px] leading-relaxed text-gray-800 dark:text-gray-200">
                  {item.workItem.payload?.msg ||
                    item.workItem.payload?.text ||
                    item.workItem.payload?.description ||
                    JSON.stringify(item.workItem.payload)}
                </p>
              </div>
              {item.draft &&
                item.draft.action_type === "subscription_win_back" && (
                  <div className="p-4 bg-orange-50/50 dark:bg-orange-900/20 border-t border-orange-100 dark:border-orange-900/50 flex flex-col gap-3">
                    <div className="flex items-center gap-2 mb-1">
                      <div className="w-8 h-8 rounded-full bg-orange-100 dark:bg-orange-800 flex items-center justify-center text-orange-600 dark:text-orange-300">
                        <svg
                          width="16"
                          height="16"
                          fill="none"
                          stroke="currentColor"
                          viewBox="0 0 24 24"
                        >
                          <path
                            strokeLinecap="round"
                            strokeLinejoin="round"
                            strokeWidth="2"
                            d="M13 10V3L4 14h7v7l9-11h-7z"
                          ></path>
                        </svg>
                      </div>
                      <h4 className="text-sm font-semibold text-orange-900 dark:text-orange-100">
                        At-Risk Subscriber Identified
                      </h4>
                    </div>
                    <div className="w-full text-[13px] leading-relaxed text-gray-700 dark:text-gray-300 italic border-l-2 border-orange-500 pl-3 py-1 bg-white/50 dark:bg-gray-800/50 rounded-r-lg">
                      "{item.draft.response}"
                    </div>
                    <div className="flex gap-2 w-full mt-2">
                      <button
                        className="flex-1 min-h-[44px] min-w-[44px] text-[13px] font-semibold bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 text-gray-700 dark:text-gray-300 rounded-xl hover:bg-gray-50 dark:hover:bg-gray-700 active:scale-[0.98] transition-all shadow-sm"
                        onClick={() => handleDismiss(item.workItem.id)}
                        disabled={decisions.blocked(item.workItem.id)}
                        data-testid="feed-dismiss-btn"
                      >
                        Dismiss
                      </button>
                      <button
                        className="flex-1 min-h-[44px] min-w-[44px] text-[13px] font-bold bg-orange-500 text-white rounded-xl hover:bg-orange-600 shadow-md shadow-orange-500/20 active:scale-[0.98] transition-all"
                        onClick={() => handleApprove(item.workItem.id)}
                        disabled={decisions.blocked(item.workItem.id)}
                        data-testid="feed-approve-btn"
                      >
                        {processingId === item.workItem.id
                          ? "Recording approval..."
                          : "Record approval"}
                      </button>
                    </div>
                  </div>
                )}
              {item.draft &&
                item.draft.action_type !== "subscription_win_back" && (
                  <div className="p-4 pt-3 bg-gray-50/50 dark:bg-gray-800/30 border-t border-gray-100/50 dark:border-gray-700/50 flex flex-col gap-4">
                    {editingId === item.workItem.id ? (
                      <div className="w-full text-[13px] leading-relaxed">
                        <textarea
                          className="w-full bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-xl p-3 text-gray-800 dark:text-gray-200 focus:outline-none focus:ring-2 focus:ring-[#0066FF]/50 resize-y min-h-[100px]"
                          value={editDraftText}
                          disabled={decisions.blocked(item.workItem.id)}
                          onChange={(e) => setEditDraftText(e.target.value)}
                          data-testid={`triage-edit-textarea-${item.workItem.id}`}
                        />
                      </div>
                    ) : (
                      <div className="w-full text-[13px] leading-relaxed text-gray-700 dark:text-gray-300 italic border-l-2 border-[#0066FF] pl-3 py-1">
                        "{item.draft.response}"
                      </div>
                    )}

                    {editingId === item.workItem.id ? (
                      <div className="flex gap-2 w-full">
                        <button
                          className="flex-1 min-h-[44px] min-w-[44px] text-[13px] font-semibold bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 text-gray-700 dark:text-gray-300 rounded-xl hover:bg-gray-50 dark:hover:bg-gray-700 active:scale-[0.98] transition-all shadow-sm"
                          onClick={() => {
                            setEditingId(null);
                            setEditDraftText("");
                          }}
                          disabled={decisions.blocked(item.workItem.id)}
                          data-testid={`triage-cancel-btn-${item.workItem.id}`}
                        >
                          Cancel
                        </button>
                        <button
                          className="flex-1 min-h-[44px] min-w-[44px] text-[13px] font-bold bg-[#0066FF] text-white rounded-xl hover:bg-[#0052CC] shadow-md shadow-[#0066FF]/20 active:scale-[0.98] transition-all"
                          onClick={() =>
                            handleSaveEditAndApprove(item.workItem.id)
                          }
                          disabled={decisions.blocked(item.workItem.id)}
                          data-testid={`triage-save-btn-${item.workItem.id}`}
                        >
                          {processingId === item.workItem.id
                            ? "Recording approval..."
                            : "Save & Approve"}
                        </button>
                      </div>
                    ) : (
                      <div className="flex gap-2 w-full">
                        <button
                          className="flex-1 min-h-[44px] min-w-[44px] text-[13px] font-semibold bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 text-gray-700 dark:text-gray-300 rounded-xl hover:bg-gray-50 dark:hover:bg-gray-700 active:scale-[0.98] transition-all shadow-sm"
                          onClick={() => handleDismiss(item.workItem.id)}
                          disabled={decisions.blocked(item.workItem.id)}
                          data-testid={`triage-dismiss-${item.workItem.id}`}
                        >
                          Dismiss
                        </button>
                        <button
                          className="flex-1 min-h-[44px] min-w-[44px] text-[13px] font-semibold bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 text-gray-700 dark:text-gray-300 rounded-xl hover:bg-gray-50 dark:hover:bg-gray-700 active:scale-[0.98] transition-all shadow-sm"
                          onClick={() => handleEdit(item)}
                          disabled={decisions.blocked(item.workItem.id)}
                          data-testid={`triage-review-btn-${item.workItem.id}`}
                        >
                          Edit
                        </button>
                        <button
                          className="flex-1 min-h-[44px] min-w-[44px] text-[13px] font-bold bg-[#0066FF] text-white rounded-xl hover:bg-[#0052CC] shadow-md shadow-[#0066FF]/20 active:scale-[0.98] transition-all"
                          onClick={() => handleApprove(item.workItem.id)}
                          disabled={decisions.blocked(item.workItem.id)}
                          data-testid={`triage-approve-${item.workItem.id}`}
                        >
                          {processingId === item.workItem.id
                            ? "Recording approval..."
                            : "Record approval"}
                        </button>
                      </div>
                    )}
                  </div>
                )}
            </div>
          ))
        )}
      </main>
    </div>
  );
}
