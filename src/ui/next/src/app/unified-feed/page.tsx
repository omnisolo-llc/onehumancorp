'use client';

import React, { useState, useEffect } from 'react';

interface FeedItemRaw {
  id: string;
  tenant_id: string;
  event_source: string;
  context_payload?: any;
  proposed_action?: any;
  lifecycle_state: string;
  created_at: string;
  updated_at: string;
}

interface WorkItem {
  id: string;
  source: string;
  payload: any;
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
  const [feedItems, setFeedItems] = useState<FeedItem[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [processingId, setProcessingId] = useState<string | null>(null);

  const fetchFeed = async () => {
    try {
      const res = await fetch('/api/agent-feed');
      if (!res.ok) {
        throw new Error('Failed to fetch feed');
      }
      const data = await res.json();
      const rawItems = (data.items || []) as FeedItemRaw[];
      const pendingItems = rawItems.filter(i => i.lifecycle_state !== "APPROVED" && i.lifecycle_state !== "DISMISSED");

      const mappedItems: FeedItem[] = pendingItems.map(raw => {
         return {
           workItem: {
             id: raw.id,
             source: raw.event_source || 'Unknown',
             payload: raw.context_payload,
             status: raw.lifecycle_state,
           },
           draft: raw.proposed_action ? {
             id: raw.id,
             response: raw.proposed_action.draft_reply || raw.proposed_action.summary || JSON.stringify(raw.proposed_action),
             status: 'draft',
             action_type: raw.proposed_action.action_type
           } : undefined
         };
      });

      // Sort by urgency/priority
      mappedItems.sort((a, b) => {
        // Higher priority sources first
        const isPaymentA = a.workItem.source.toLowerCase().includes('payment') || a.workItem.source.toLowerCase().includes('stripe');
        const isPaymentB = b.workItem.source.toLowerCase().includes('payment') || b.workItem.source.toLowerCase().includes('stripe');

        if (isPaymentA && !isPaymentB) return -1;
        if (!isPaymentA && isPaymentB) return 1;

        // Then unread/urgent messages
        const isUrgentA = a.workItem.payload?.priority === 'high' || a.workItem.payload?.priority === 'urgent';
        const isUrgentB = b.workItem.payload?.priority === 'high' || b.workItem.payload?.priority === 'urgent';

        if (isUrgentA && !isUrgentB) return -1;
        if (!isUrgentA && isUrgentB) return 1;

        return 0; // Maintain order otherwise
      });

      setFeedItems(mappedItems);
    } catch (err: any) {
      setError(err.message);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchFeed();
  }, []);

  const handleAction = async (itemId: string, action: string) => {
    setProcessingId(itemId);
    try {
      const res = await fetch(`/api/agent-feed/${itemId}`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ state: action })
      });
      if (res.ok) {
        setFeedItems(prev => prev.filter(i => i.workItem.id !== itemId));
      }
    } catch (e) {
      console.error(e);
    } finally {
      setProcessingId(null);
    }
  };

  const handleApprove = (itemId: string) => handleAction(itemId, 'APPROVED');
  const handleReject = (itemId: string) => handleAction(itemId, 'DISMISSED');

  const handleEdit = (itemId: string) => {
    // Basic stub for edit flow
    alert('Edit draft triggered for ' + itemId);
  };

  if (loading) return <div className="p-4 text-center">Loading feed...</div>;
  if (error) return <div className="p-4 text-center text-red-500">Error: {error}</div>;

  return (
    <div className="w-full max-w-[375px] mx-auto min-h-screen bg-[#F5F5F7] dark:bg-[#1D1D1F] flex flex-col text-[#1D1D1F] dark:text-[#F5F5F7]">
<a href="/promoter" className="block w-full text-center bg-purple-500 text-white py-2 rounded mt-4" data-testid="promoter-link">Go to Promoter Agent</a>
      <header className="bg-white/80 dark:bg-black/80 backdrop-blur-md border-b border-gray-200/50 dark:border-gray-800/50 p-4 sticky top-0 z-10 flex justify-between items-center">
        <h1 className="text-xl font-bold tracking-tight">Today</h1>
      </header>

      <main className="flex-1 overflow-y-auto p-4 space-y-4" data-testid="agent-feed">
        {feedItems.length === 0 ? (
          <div className="triage-card empty text-center text-gray-500 py-8 flex flex-col items-center gap-3 glassmorphism shadow-sm opacity-90" data-testid="triage-feed-empty">
             <div className="text-3xl mb-2">✨</div>
             <h3 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7]">
               No items need your attention right now. Great job!
             </h3>
          </div>
        ) : (
          feedItems.map((item) => (
            <div key={item.workItem.id} className="triage-card w-full glassmorphism bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] rounded-[24px] shadow-sm flex flex-col mb-4 overflow-hidden transition-all duration-300" data-testid="agent-feed-card">
              <div className="p-5 border-b border-[rgba(255,255,255,0.2)] bg-[rgba(255,255,255,0.4)] dark:bg-[rgba(22,22,26,0.5)] backdrop-blur-[30px] backdrop-saturate-[210%] flex justify-between items-center">
                  <div className="flex items-center gap-2">
                      <span className="text-[10px] font-bold uppercase tracking-widest text-[#0066FF] bg-[#0066FF]/10 dark:bg-[#0066FF]/20 px-2.5 py-1 rounded-full">
                        {item.workItem.source}
                      </span>
                  </div>
                  <span className="text-[11px] font-medium text-gray-400">Just now</span>
              </div>
              <div className="p-4">
                 <p className="triage-context text-[15px] font-medium text-gray-900 dark:text-white leading-snug break-words line-clamp-2">
                   {item.workItem.payload?.msg || item.workItem.payload?.text || item.workItem.payload?.description || JSON.stringify(item.workItem.payload)}
                 </p>
              </div>
              {item.draft && (
                <div className="p-5 pt-2 flex flex-col gap-3 w-full border-t border-white/20 dark:border-white/10 bg-white/40 dark:bg-black/20 backdrop-blur-[30px] saturate-[210%]">
                   <div className="proposed-action border border-[#0066FF]/20 dark:border-[#0066FF]/30 bg-white/50 dark:bg-black/30 backdrop-blur-[30px] saturate-[210%] p-4 text-[13px] leading-relaxed text-gray-900 dark:text-white whitespace-pre-wrap break-words">
                     "{item.draft.response}"
                   </div>
                   <div className="text-[11px] uppercase tracking-wider font-bold text-[#0066FF] dark:text-[#3388FF] mb-2">Draft Reply:</div>
                   <div className="flex flex-col sm:flex-row gap-3 w-full">
                     <button
                       className="w-full flex-1 min-h-[44px] min-w-[44px] px-4 border border-gray-300 dark:border-gray-600 bg-white/50 dark:bg-black/50 backdrop-blur-[30px] saturate-[210%] text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-white/70 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center disabled:opacity-50 shadow-sm"
                       onClick={() => handleReject(item.workItem.id)}
                       disabled={processingId === item.workItem.id}
                       data-testid="unified-feed-reject-btn"
                     >
                       Reject
                     </button>
                     <button
                       className="w-full flex-1 min-h-[44px] min-w-[44px] px-4 border border-gray-300 dark:border-gray-600 bg-white/50 dark:bg-black/50 backdrop-blur-[30px] saturate-[210%] text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-white/70 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center disabled:opacity-50 shadow-sm"
                       onClick={() => handleEdit(item.workItem.id)}
                       disabled={processingId === item.workItem.id}
                       data-testid="edit-proposal"
                     >
                       Edit
                     </button>
                     <button
                       className="w-full flex-1 min-h-[44px] min-w-[44px] px-4 bg-[#0066FF] text-white font-medium hover:bg-[#0052CC] transition-all duration-200 shadow-md flex items-center justify-center disabled:opacity-50"
                       onClick={() => handleApprove(item.workItem.id)}
                       disabled={processingId === item.workItem.id}
                       data-testid="feed-approve-btn"
                     >
                       {processingId === item.workItem.id ? '...' : 'Approve & Send'}
                     </button>
                   </div>
                </div>
              )}
            </div>
          ))
        )}
      </main>
    </div>
  );
}
