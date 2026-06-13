'use client';

import React, { useEffect, useState } from 'react';
import { CustomerInquiryCard } from './components/CustomerInquiryCard';
import { ApprovalCard } from './components/ApprovalCard';
import { DailySummaryCard } from './components/DailySummaryCard';
import { AgentMessage } from './components/AgentMessage';

interface FeedItem {
  id: string;
  tenant_id: string;
  event_source: string;
  context_payload?: any;
  proposed_action?: any;
  lifecycle_state: string;
  created_at: string;
  updated_at: string;
}

export default function AgentWorkFeedPage() {
  const [items, setItems] = useState<FeedItem[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    async function fetchFeed() {
      try {
        const res = await fetch('/api/agent-feed');
        if (!res.ok) {
          throw new Error('Failed to fetch feed');
        }
        const data = await res.json();
        setItems(data.items || []);
      } catch (err: any) {
        setError(err.message);
      } finally {
        setLoading(false);
      }
    }

    fetchFeed();
  }, []);

  const handleAction = async (id: string, state: string) => {
    try {
      const res = await fetch(`/api/agent-feed/${id}`, {
        method: 'PATCH',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ state }),
      });
      if (!res.ok) throw new Error('Action failed');

      // Update UI optimistically or refetch
      setItems((prev) => prev.filter((item) => item.id !== id));
    } catch (err: any) {
      alert(err.message);
    }
  };

  const renderCard = (item: FeedItem) => {
    const type = item.event_source;

    if (type === 'CUSTOMER_INQUIRY' || type === 'INQUIRY') {
      return (
        <AgentMessage key={item.id}>
          <CustomerInquiryCard
            id={item.id}
            customerName={item.context_payload?.customerName || 'Customer'}
            messageSnippet={item.context_payload?.message || item.context_payload?.summary || item.proposed_action?.description || 'You have a new inquiry.'}
            time={new Date(item.created_at).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })}
            onDraftReply={() => handleAction(item.id, 'APPROVED')}
          />
        </AgentMessage>
      );
    }

    if (type === 'APPROVAL' || type === 'QUOTE_APPROVAL') {
      return (
        <AgentMessage key={item.id}>
          <ApprovalCard
            id={item.id}
            title={item.proposed_action?.title || 'Review Required'}
            amount={item.context_payload?.amount || 0}
            customerName={item.context_payload?.customerName || 'Customer'}
            onAccept={() => handleAction(item.id, 'APPROVED')}
            onEdit={() => alert(`Edit ${item.id}`)}
          />
        </AgentMessage>
      );
    }

    if (type === 'SUMMARY' || type === 'DAILY_SUMMARY') {
      return (
        <AgentMessage key={item.id}>
          <DailySummaryCard
            id={item.id}
            date={new Date(item.created_at).toLocaleDateString()}
            summaryText={item.context_payload?.summary || 'Summary unavailable.'}
            onViewDetails={() => alert(`View details ${item.id}`)}
          />
        </AgentMessage>
      );
    }

    // Default fallback card
    return (
      <AgentMessage key={item.id}>
        <div className="bg-white/80 dark:bg-gray-800/80 backdrop-blur-md rounded-2xl p-4 shadow-sm border border-gray-100 dark:border-gray-700 w-full">
          <h3 className="font-medium text-gray-900 dark:text-white text-sm mb-1">{item.proposed_action?.title || 'Action Required'}</h3>
          <p className="text-sm text-gray-600 dark:text-gray-300 mb-4">{item.context_payload?.summary || item.proposed_action?.description || 'A new update requires your attention.'}</p>
          <div className="flex gap-2">
            <button
              onClick={() => handleAction(item.id, 'APPROVED')}
              className="flex-1 bg-indigo-600 hover:bg-indigo-700 text-white font-medium py-2 px-4 rounded-xl min-h-[44px] transition-colors text-sm"
              data-testid={`accept-btn-${item.id}`}
            >
              Approve
            </button>
            <button
              onClick={() => handleAction(item.id, 'DISMISSED')}
              className="flex-1 bg-gray-100 dark:bg-gray-700 hover:bg-gray-200 dark:hover:bg-gray-600 text-gray-900 dark:text-white font-medium py-2 px-4 rounded-xl min-h-[44px] transition-colors text-sm"
            >
              Dismiss
            </button>
          </div>
        </div>
      </AgentMessage>
    );
  };

  return (
    <div className="flex flex-col h-screen bg-gray-50 dark:bg-gray-900 overflow-hidden font-sans w-full mx-auto" style={{ maxWidth: '414px' }}>
      {/* Header */}
      <header className="bg-white/80 dark:bg-gray-800/80 backdrop-blur-md border-b border-gray-200 dark:border-gray-700 px-4 py-3 flex justify-between items-center z-10 sticky top-0">
        <h1 className="text-lg font-semibold text-gray-900 dark:text-white">Assistant</h1>
        <div className="w-8 h-8 rounded-full bg-gray-200 dark:bg-gray-700 overflow-hidden border-2 border-white dark:border-gray-800 shadow-sm">
          <svg className="w-full h-full text-gray-400" fill="currentColor" viewBox="0 0 24 24">
            <path d="M24 20.993V24H0v-2.996A14.977 14.977 0 0112.004 15c4.904 0 9.26 2.354 11.996 5.993zM16.002 8.999a4 4 0 11-8 0 4 4 0 018 0z" />
          </svg>
        </div>
      </header>

      {/* Main Feed Content */}
      <main className="flex-1 overflow-y-auto p-4 pb-24 w-full">
        <div className="space-y-6 w-full max-w-full">

          <AgentMessage>
            <div className="bg-white dark:bg-gray-800 rounded-2xl rounded-tl-sm p-3 mb-2 shadow-sm text-sm text-gray-800 dark:text-gray-200 w-full">
              Good morning! You have new updates and actions waiting in your feed.
            </div>
          </AgentMessage>

          {loading && <p className="text-center text-gray-500 text-sm">Loading feed...</p>}
          {error && <p className="text-center text-red-500 text-sm">Error: {error}</p>}
          {!loading && !error && items.length === 0 && (
            <p className="text-center text-gray-500 text-sm">You have no pending actions in your feed.</p>
          )}

          {items.map(renderCard)}

          {/* Spacer to ensure scrollability past the bottom nav */}
          <div className="h-4"></div>
        </div>
      </main>

      {/* Bottom Navigation */}
      <nav className="bg-white/90 dark:bg-gray-800/90 backdrop-blur-xl border-t border-gray-200 dark:border-gray-700 pb-safe absolute bottom-0 w-full z-10" style={{ maxWidth: '414px' }}>
        <div className="flex justify-around items-center h-16">
          <button className="flex flex-col items-center justify-center w-full h-full text-indigo-600 dark:text-indigo-400">
            <svg className="w-6 h-6 mb-1" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path fillRule="evenodd" d="M18 10c0 3.866-3.582 7-8 7a8.841 8.841 0 01-4.083-.98L2 17l1.338-3.123C2.493 12.767 2 11.434 2 10c0-3.866 3.582-7 8-7s8 3.134 8 7zM7 9H5v2h2V9zm8 0h-2v2h2V9zM9 9h2v2H9V9z" clipRule="evenodd" /></svg>
            <span className="text-[10px] font-medium">Feed</span>
          </button>
          <button className="flex flex-col items-center justify-center w-full h-full text-gray-500 dark:text-gray-400 hover:text-gray-900 dark:hover:text-gray-100">
            <svg className="w-6 h-6 mb-1" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z" /></svg>
            <span className="text-[10px] font-medium">Customers</span>
          </button>
          <button className="flex flex-col items-center justify-center w-full h-full text-gray-500 dark:text-gray-400 hover:text-gray-900 dark:hover:text-gray-100">
            <svg className="w-6 h-6 mb-1" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10" /></svg>
            <span className="text-[10px] font-medium">Ops</span>
          </button>
          <button className="flex flex-col items-center justify-center w-full h-full text-gray-500 dark:text-gray-400 hover:text-gray-900 dark:hover:text-gray-100">
            <svg className="w-6 h-6 mb-1" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
            <span className="text-[10px] font-medium">Money</span>
          </button>
        </div>
      </nav>
    </div>
  );
}
