'use client';

import { useState, useEffect, useCallback } from 'react';

type ApprovalState = 'pending' | 'approved' | 'rejected';

interface UnifiedFeedItem {
  id: string;
  department: string;
  agent_id?: string;
  type: string;
  status: ApprovalState;
  priority: number;
  proposed_action: any;
  context_payload: any;
  created_at: string;
}

interface ActivityItem {
  id: string;
  department: string;
  event_type: string;
  payload: any;
  created_at: string;
}

export function UnifiedAgentFeed() {
  const [activeTab, setActiveTab] = useState<"approvals" | "activity">("approvals");
  const [approvals, setApprovals] = useState<UnifiedFeedItem[]>([]);
  const [activities, setActivities] = useState<ActivityItem[]>([]);
  const [loading, setLoading] = useState(true);
  const [activityLoading, setActivityLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const fetchFeed = useCallback(async () => {
    try {
      const response = await fetch('/api/agents/workflows?status=pending');
      if (!response.ok) throw new Error('Failed to fetch approvals');
      const data = await response.json();
      setApprovals(data.data || []);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to load feed');
    } finally {
      setLoading(false);
    }
  }, []);

  const fetchActivity = useCallback(async () => {
    setActivityLoading(true);
    try {
      const response = await fetch('/api/agents/workflows?status=completed&limit=10');
      if (!response.ok) throw new Error('Failed to fetch activity');
      const data = await response.json();
      setActivities(data.data || []);
    } catch (err) {
      console.error('Failed to load activity', err);
    } finally {
      setActivityLoading(false);
    }
  }, []);

  useEffect(() => {
    if (activeTab === "approvals") {
      fetchFeed();
    } else {
      fetchActivity();
    }
  }, [activeTab, fetchFeed, fetchActivity]);

  useEffect(() => {
    const handleVoiceCommandProcessed = (event: Event) => {
      const customEvent = event as CustomEvent;
      const result = customEvent.detail;

      if (result && result.action && result.action.type === 'quote_draft') {
        const newItem: UnifiedFeedItem = {
          id: `voice-quote-${Date.now()}`,
          department: 'sales_assistant',
          type: 'approval_required',
          status: 'pending',
          priority: 100,
          proposed_action: {
            feature_type: 'quote_draft',
            description: result.action.details.description,
            amount: result.action.details.amount,
          },
          context_payload: {},
          created_at: new Date().toISOString()
        };

        setApprovals(prev => [newItem, ...prev]);
        setActiveTab('approvals');
      }
    };

    window.addEventListener('voice-command-processed', handleVoiceCommandProcessed);
    return () => {
      window.removeEventListener('voice-command-processed', handleVoiceCommandProcessed);
    };
  }, []);

  const handleDecision = async (id: string, approved: boolean) => {
    setApprovals(prev => prev.filter(item => item.id !== id));

    // For voice quote drafts
    if (id.startsWith('voice-quote-')) {
        if (approved) {
           const newActivity: ActivityItem = {
             id: `activity-${Date.now()}`,
             department: 'sales_assistant',
             event_type: 'Approved',
             payload: { original_payload: { description: 'Action completed' } },
             created_at: new Date().toISOString()
           };
           setActivities(prev => [newActivity, ...prev]);
        }
        return;
    }

    try {
      await fetch(`/api/agents/workflows`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          workflow_id: id,
          action: approved ? 'approve' : 'reject'
        })
      });
      // Optionally refresh activity feed in background
      if (activeTab === 'activity') fetchActivity();
    } catch (err) {
      console.error('Failed to submit decision', err);
      // Revert optimistic update
      fetchFeed();
    }
  };

  return (
    <section className="mt-8 mb-8">
      <div className="flex flex-col sm:flex-row sm:items-center justify-between mb-6 gap-4">
        <div>
          <h2 className="text-2xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] tracking-tight">
            Unified Agent Feed
          </h2>
          <p className="text-sm text-gray-500 dark:text-gray-400 mt-1">
            Work waiting for your approval.
          </p>
        </div>

        <div className="flex bg-gray-100 dark:bg-gray-800/50 rounded-[8px] p-1 border border-gray-200 dark:border-gray-700 w-full sm:w-auto h-11 items-center">
          <button
            onClick={() => setActiveTab("approvals")}
            className={`flex-1 sm:flex-none px-4 sm:px-6 h-full min-h-[44px] min-w-[44px] rounded-[6px] text-sm font-medium transition-all duration-200 flex items-center justify-center ${
              activeTab === "approvals"
                ? "bg-white dark:bg-gray-700 text-[#1D1D1F] dark:text-[#F5F5F7] shadow-sm"
                : "text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-200"
            }`}
            data-testid="tab-approvals"
            aria-label="View Pending Approvals"
          >
            Requires Action
            {approvals.length > 0 && activeTab !== "approvals" && (
              <span className="ml-2 inline-flex items-center justify-center w-5 h-5 text-[10px] font-bold text-white bg-[#0066FF] rounded-full">
                {approvals.length}
              </span>
            )}
          </button>
          <button
            onClick={() => setActiveTab("activity")}
            className={`flex-1 sm:flex-none px-4 sm:px-6 h-full min-h-[44px] min-w-[44px] rounded-[6px] text-sm font-medium transition-all duration-200 flex items-center justify-center ${
              activeTab === "activity"
                ? "bg-white dark:bg-gray-700 text-[#1D1D1F] dark:text-[#F5F5F7] shadow-sm"
                : "text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-200"
            }`}
            data-testid="tab-activity"
            aria-label="View Activity History"
          >
            Activity History
          </button>
        </div>
      </div>

      <div className="flex flex-col gap-4 min-w-[320px] max-w-full">
        {activeTab === "approvals" && (
          <>
            {loading && (
              <div className="w-full p-4 glassmorphism rounded-[16px] text-center text-gray-500">
                Loading Feed...
              </div>
            )}

            {error && (
              <div className="w-full p-4 bg-red-50 dark:bg-red-900/20 text-red-600 dark:text-red-400 rounded-[16px] border border-red-200 dark:border-red-800/30 text-sm">
                {error}
              </div>
            )}

            {!loading && !error && approvals.length === 0 && (
              <div className="w-full p-6 glassmorphism rounded-[16px] text-center border border-white/40 dark:border-white/10" data-testid="empty-feed">
                <div className="w-12 h-12 rounded-full bg-green-100 dark:bg-green-900/30 flex items-center justify-center mx-auto mb-3">
                  <svg className="w-6 h-6 text-green-600 dark:text-green-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" />
                  </svg>
                </div>
                <h3 className="text-lg font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7]">All caught up!</h3>
                <p className="text-sm text-gray-600 dark:text-gray-400 mt-1">
                  Your agents have no pending actions for you right now.
                </p>
              </div>
            )}

            {approvals.map((approval) => (
              <div
                key={approval.id}
                className="glassmorphism p-5 rounded-[16px] border border-white/40 dark:border-white/10 shadow-sm flex flex-col gap-4 animate-fade-in pointer-events-auto"
                data-testid={approval.proposed_action?.feature_type === 'quote_draft' ? 'draft-quote-card' : 'approval-card'}
              >
                <div className="flex items-center justify-between">
                  <span className="text-xs font-bold font-outfit uppercase tracking-wider text-indigo-600 dark:text-indigo-400 bg-indigo-50 dark:bg-indigo-900/30 px-2 py-1 rounded-md">
                    {approval.department.replace('_', ' ')}
                  </span>
                  {approval.priority > 50 && (
                    <span className="flex items-center text-xs font-bold font-outfit uppercase tracking-wider text-[#FF3B30] bg-red-50 dark:bg-red-900/30 px-2 py-1 rounded-md">
                      <span className="w-1.5 h-1.5 rounded-full bg-[#FF3B30] mr-1.5 animate-pulse"></span>
                      High Priority
                    </span>
                  )}
                </div>

                <div>
                  <h3 className="text-lg font-semibold font-inter text-[#1D1D1F] dark:text-[#F5F5F7] leading-snug">
                    {(approval.proposed_action || approval.context_payload)?.description || 'Action Required'}
                  </h3>
                  {(approval.proposed_action || approval.context_payload)?.amount !== undefined && (
                    <div className="mt-2 flex items-baseline gap-1">
                       <span className="text-sm font-medium text-gray-500">Amount:</span>
                       <span className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-white">
                         ${(approval.proposed_action || approval.context_payload).amount}
                       </span>
                    </div>
                  )}
                  {approval.context_payload?.customer_name && (
                    <p className="text-sm text-gray-600 dark:text-gray-400 mt-1">
                      Customer: <span className="font-medium text-[#1D1D1F] dark:text-gray-200">{approval.context_payload.customer_name}</span>
                    </p>
                  )}
                  {approval.context_payload?.summary && (
                    <p className="text-sm text-gray-600 dark:text-gray-400 mt-1 line-clamp-2">
                      {approval.context_payload.summary}
                    </p>
                  )}
                </div>

                <div className="flex flex-col gap-3 mt-2 min-h-[44px]">
                  {(approval.proposed_action || approval.context_payload)?.feature_type === 'flash_sale' ? (
                    <div className="flex flex-col sm:flex-row gap-3 w-full">
                      <button
                        onClick={() => handleDecision(approval.id, true)}
                        className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] bg-[#0066FF] text-white font-medium hover:bg-[#0052CC] transition-all duration-200 shadow-md flex items-center justify-center"
                        aria-label="Approve & Schedule Sale"
                        data-testid="approve-sale"
                      >
                        Approve & Schedule
                      </button>
                      <button
                        onClick={() => handleDecision(approval.id, false)}
                        className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center"
                        aria-label="Dismiss proposal"
                        data-testid="dismiss-sale"
                      >
                        Dismiss
                      </button>
                    </div>
                  ) : (approval.proposed_action || approval.context_payload)?.context?.weekly_health_report === true ? (
                    <div className="flex flex-col sm:flex-row gap-3 w-full">
                      <button
                        onClick={() => handleDecision(approval.id, true)}
                        className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] bg-green-600 text-white font-medium hover:bg-green-700 transition-all duration-200 shadow-md flex items-center justify-center"
                        aria-label="Draft it"
                        data-testid="approve-draft"
                      >
                        Yes, draft it!
                      </button>
                      <button
                        onClick={() => handleDecision(approval.id, false)}
                        className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center"
                        aria-label="Dismiss proposal"
                        data-testid="dismiss-draft"
                      >
                        Dismiss
                      </button>
                    </div>                  ) : (approval.proposed_action || approval.context_payload)?.remaining_stock !== undefined ? (
                    <div className="flex flex-col sm:flex-row gap-3 w-full">
                      <button
                        onClick={() => handleDecision(approval.id, true)}
                        className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] bg-amber-500 text-white font-medium hover:bg-amber-600 transition-all duration-200 shadow-md flex items-center justify-center"
                        aria-label="Approve Restock"
                        data-testid="approve-restock"
                      >
                        Approve Restock
                      </button>
                      <button
                        onClick={() => handleDecision(approval.id, false)}
                        className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center"
                        aria-label="Dismiss restock"
                        data-testid="dismiss-restock"
                      >
                        Dismiss
                      </button>
                    </div>
                  ) : (approval.proposed_action || approval.context_payload)?.feature_type === 'quote_draft' ? (
                    <>
                      <button
                        onClick={() => handleDecision(approval.id, true)}
                        className="w-full min-h-[44px] min-w-[44px] px-4 rounded-[8px] bg-[#0066FF] text-white font-medium hover:bg-[#0052CC] transition-all duration-200 shadow-md flex items-center justify-center mb-3"
                        aria-label="Approve & Send Proposal"
                        data-testid="approve-quote-draft"
                      >
                        Approve & Send Proposal
                      </button>
                      <div className="flex flex-col sm:flex-row gap-3 w-full">
                        <button
                          onClick={() => handleDecision(approval.id, false)}
                          className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center"
                          aria-label="Ask Agent to Adjust"
                          data-testid="reject-proposal"
                        >
                          Ask Agent to Adjust
                        </button>
                      </div>
                    </>
                  ) : (
                    <>
                      <button
                        onClick={() => handleDecision(approval.id, true)}
                        className="w-full min-h-[44px] min-w-[44px] px-4 rounded-[8px] bg-[#0066FF] text-white font-medium hover:bg-[#0052CC] transition-all duration-200 shadow-md flex items-center justify-center mb-3"
                        aria-label="Approve proposal"
                        data-testid="approve-proposal"
                      >
                        Approve
                      </button>
                      <div className="flex flex-col sm:flex-row gap-3 w-full">
                        <button
                          onClick={() => {}}
                          className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center"
                          aria-label="Edit proposal"
                          data-testid="edit-proposal"
                        >
                          Edit
                        </button>
                        <button
                          onClick={() => handleDecision(approval.id, false)}
                          className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center"
                          aria-label="Reject proposal"
                          data-testid="reject-proposal"
                        >
                          Deny
                        </button>
                      </div>
                    </>
                  )}
                </div>
              </div>
            ))}
          </>
        )}

        {activeTab === "activity" && (
          <>
            {activityLoading && (
              <div className="w-full p-4 glassmorphism rounded-[16px] text-center text-gray-500">
                Loading Activity Feed...
              </div>
            )}
            {!activityLoading && activities.length === 0 && (
              <div className="w-full p-6 glassmorphism rounded-[16px] text-center">
                <p className="text-sm text-gray-600 dark:text-gray-400 mt-1">
                  No recent activity found.
                </p>
              </div>
            )}
            <div className="flex flex-col gap-3 min-w-[320px] max-w-full">
            {activities.map((activity) => (
              <div
                key={activity.id}
                className="glassmorphism p-5 rounded-[16px] border border-white/40 dark:border-white/10 shadow-sm flex flex-col gap-3 opacity-90 min-h-[44px]"
              >
                <div className="flex items-center justify-between">
                  <span className="text-xs font-bold font-outfit uppercase tracking-wider text-indigo-600 dark:text-indigo-400 bg-indigo-50 dark:bg-indigo-900/30 px-2 py-1 rounded-md">
                    {activity.department.replace('_', ' ')}
                  </span>
                  {activity.event_type === 'Paused' || activity.event_type === 'PAUSED' ? (
                    <span className="text-xs font-bold font-outfit uppercase tracking-wider px-2 py-1 rounded-md text-yellow-600 bg-yellow-50 dark:text-yellow-400 dark:bg-yellow-900/30">
                      PAUSED
                    </span>
                  ) : (
                    <span className="text-xs font-bold font-outfit uppercase tracking-wider px-2 py-1 rounded-md text-green-600 bg-green-50 dark:text-green-400 dark:bg-green-900/30">
                      {activity.event_type === 'Approved' || activity.event_type === 'APPROVED' ? 'APPROVED' : activity.event_type}
                    </span>
                  )}
                </div>
                <h3 className="text-md font-semibold font-inter text-[#1D1D1F] dark:text-[#F5F5F7] leading-snug">
                  {(() => {
                    try {
                      const p = typeof activity.payload === 'string' ? JSON.parse(activity.payload) : activity.payload;
                      // Fallback logic specific to Paused state that gets stored inside proposed_content
                      if (p?.original_payload?.proposed_content?.includes("System is paused")) {
                          return p.original_payload.proposed_content;
                      }
                      return p?.original_payload?.description || 'Action completed';
                    } catch (e) {
                      return 'Action completed';
                    }
                  })()}
                </h3>
                <span className="text-xs text-gray-500 font-inter">{new Date(activity.created_at).toLocaleString()}</span>
              </div>
            ))}
            </div>
          </>
        )}
      </div>
    </section>
  );
}
