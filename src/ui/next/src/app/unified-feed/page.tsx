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
          <div className="text-center text-gray-500 py-8 flex flex-col items-center gap-3 glassmorphism shadow-sm opacity-90" data-testid="triage-feed-empty">
             <div className="text-3xl mb-2">✨</div>
             <h3 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7]">
               All caught up!
             </h3>
          </div>
        ) : (
          feedItems.map((item) => (
            <div key={item.workItem.id} className="w-full bg-white/70 dark:bg-gray-900/70 backdrop-blur-lg shadow-sm border border-gray-200/50 dark:border-gray-700/50 rounded-2xl overflow-hidden transition-all duration-300" data-testid="agent-feed-card">
              <div className="p-4 pb-3 border-b border-gray-100/50 dark:border-gray-800/50 flex justify-between items-center">
                  <div className="flex items-center gap-2">
                      <span className="text-[10px] font-bold uppercase tracking-widest text-[#0066FF] bg-[#0066FF]/10 dark:bg-[#0066FF]/20 px-2.5 py-1 rounded-full">
                        {item.workItem.source}
                      </span>
                  </div>
                  <span className="text-[11px] font-medium text-gray-400">Just now</span>
              </div>
              <div className="p-4">
                 <p className="text-[14px] leading-relaxed text-gray-800 dark:text-gray-200">
                   {item.workItem.payload?.msg || item.workItem.payload?.text || item.workItem.payload?.description || JSON.stringify(item.workItem.payload)}
                 </p>
              </div>
              {item.draft && (
                <div className="p-4 pt-3 bg-gray-50/50 dark:bg-gray-800/30 border-t border-gray-100/50 dark:border-gray-700/50 flex flex-col gap-4">
                   <div className="w-full text-[13px] leading-relaxed text-gray-700 dark:text-gray-300 italic border-l-2 border-[#0066FF] pl-3 py-1">
                     "{item.draft.response}"
                   </div>
                   <div className="flex gap-2 w-full">
                     <button
                       className="flex-1 min-h-[44px] min-w-[44px] text-[13px] font-semibold bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 text-gray-700 dark:text-gray-300 rounded-xl hover:bg-gray-50 dark:hover:bg-gray-700 active:scale-[0.98] transition-all shadow-sm"
                       onClick={() => handleReject(item.workItem.id)}
                       disabled={processingId === item.workItem.id}
                       data-testid="unified-feed-reject-btn"
                     >
                       Reject
                     </button>
                     <button
                       className="flex-1 min-h-[44px] min-w-[44px] text-[13px] font-semibold bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 text-gray-700 dark:text-gray-300 rounded-xl hover:bg-gray-50 dark:hover:bg-gray-700 active:scale-[0.98] transition-all shadow-sm"
                       onClick={() => handleEdit(item.workItem.id)}
                       disabled={processingId === item.workItem.id}
                       data-testid="edit-proposal"
                     >
                       Edit
                     </button>
                     <button
                       className="flex-1 min-h-[44px] min-w-[44px] text-[13px] font-bold bg-[#0066FF] text-white rounded-xl hover:bg-[#0052CC] shadow-md shadow-[#0066FF]/20 active:scale-[0.98] transition-all"
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
