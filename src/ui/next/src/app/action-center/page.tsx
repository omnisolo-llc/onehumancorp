"use client";

import React, { useState, useEffect } from 'react';
import { AppShell } from '../components/AppShell';
import { SyncManager } from '../../lib/sync/SyncManager';
import { getActions } from '../utils/offlineQueue';


type ApprovalRequest = {
  id: string;
  tenant_id: string;
  department: string;
  description: string;
  status: string;
  action_risk: string;
  payload?: any;
};

export default function ActionCenterPage() {
  const [approvals, setApprovals] = useState<ApprovalRequest[]>([]);
  const [loading, setLoading] = useState(true);
  const [actionStatus, setActionStatus] = useState("");
  const [isOffline, setIsOffline] = useState(false);
  const [offlineActionsCount, setOfflineActionsCount] = useState(0);


  const fetchApprovals = async () => {
    try {
      setLoading(true);
      const token = typeof window !== 'undefined' ? localStorage.getItem('token') || '' : '';
      const response = await fetch('/api/agents/approvals', {
        headers: {
          'Authorization': `Bearer ${token}`
        }
      });
      if (response.ok) {
        const data = await response.json();
        // Filter for Business Advisory ("The Advisor") recommendations
        const advisoryApprovals = (data.pending_approvals || []).filter(
          (a: ApprovalRequest) => a.department === 'business_advisory' || a.department === 'The Advisor' || a.department === 'operations' || a.department === 'Operations' || a.department === 'The Negotiator' || a.department === 'sales' || a.department === 'Sales'
        );
        setApprovals(advisoryApprovals);
      }
    } catch (error) {
      console.error("Failed to fetch approvals", error);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchApprovals();

    const updateOfflineCount = async () => {
      try {
        const actions = await getActions();
        setOfflineActionsCount(actions.length);
      } catch (err) {}
    };
    updateOfflineCount();

    setIsOffline(!navigator.onLine);

    const handleOnline = () => setIsOffline(false);
    const handleOffline = () => setIsOffline(true);
    window.addEventListener("online", handleOnline);
    window.addEventListener("offline", handleOffline);

    const handleQueueUpdated = () => updateOfflineCount();
    window.addEventListener('ohc_queue_updated', handleQueueUpdated);

    return () => {
      window.removeEventListener("online", handleOnline);
      window.removeEventListener("offline", handleOffline);
      window.removeEventListener('ohc_queue_updated', handleQueueUpdated);
    };
  }, []);


  const handleApprove = async (id: string) => {
    if (isOffline) {
      await SyncManager.getInstance().enqueue({
        id: crypto.randomUUID ? crypto.randomUUID() : Date.now().toString(),
        type: 'advisory_action',
        payload: { id, approved: true },
        timestamp: Date.now()
      });
      setApprovals(prev => prev.filter(a => a.id !== id));
      setActionStatus("Action approved offline.");
      setTimeout(() => setActionStatus(""), 3000);
      return;
    }

    try {
      const token = typeof window !== 'undefined' ? localStorage.getItem('token') || '' : '';
      setApprovals(prev => prev.filter(a => a.id !== id));
      const response = await fetch(`/api/agents/approvals/${id}`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${token}`
        },
        body: JSON.stringify({ approved: true })
      });
      if (response.ok) {
        setActionStatus("Action approved and executed..");
      } else {
        setActionStatus("Failed to approve action.");
        fetchApprovals();
      }
    } catch (error) {
      console.error("Failed to approve", error);
      setActionStatus("Error approving action.");
      fetchApprovals();
    }
  };

  const handleDismiss = async (id: string) => {
    if (isOffline) {
      await SyncManager.getInstance().enqueue({
        id: crypto.randomUUID ? crypto.randomUUID() : Date.now().toString(),
        type: 'advisory_action',
        payload: { id, approved: false },
        timestamp: Date.now()
      });
      setApprovals(prev => prev.filter(a => a.id !== id));
      setActionStatus("Action dismissed offline.");
      setTimeout(() => setActionStatus(""), 3000);
      return;
    }

     try {
      const token = typeof window !== 'undefined' ? localStorage.getItem('token') || '' : '';
      setApprovals(prev => prev.filter(a => a.id !== id));
      const response = await fetch(`/api/agents/approvals/${id}`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${token}`
        },
        body: JSON.stringify({ approved: false })
      });
      if (response.ok) {
        setActionStatus("Action dismissed.");
      } else {
        setActionStatus("Failed to dismiss action.");
        fetchApprovals();
      }
    } catch (error) {
      console.error("Failed to dismiss", error);
      setActionStatus("Error dismissing action.");
      fetchApprovals();
    }
  };

  const extractPayload = (description: string, rawPayload: any) => {
    if (rawPayload && typeof rawPayload === 'object' && Object.keys(rawPayload).length > 0) {
      return rawPayload;
    }
    const parts = description.split(" | Payload: ");
    if (parts.length > 1) {
      try {
        return JSON.parse(parts[1]);
      } catch (e) {
        return null;
      }
    }
    return null;
  };

  const cleanDescription = (description: string) => {
    return description.split(" | Payload: ")[0];
  };

  return (
    <AppShell
      title="Action Center"
      subtitle="Review weekly recommendations from your Business Advisor."
      statusItems={[
        { label: "Pending", value: String(approvals.length), tone: approvals.length > 0 ? "warn" : "good" },
      ]}
      actions={[{ label: "Team", href: "/team" }]}
    >
      <div className="w-full max-w-[375px] mx-auto md:max-w-none">
        {isOffline && (
          <div className="mb-4 w-full p-2 glassmorphism rounded-[8px] bg-yellow-100 dark:bg-yellow-900/30 text-yellow-800 dark:text-yellow-200 text-center text-sm font-semibold flex items-center justify-center gap-2">
            <span>📡</span> You are offline. Actions will sync when online.
          </div>
        )}
        {offlineActionsCount > 0 && (
          <div className="mb-4 w-full p-2 glassmorphism rounded-[8px] bg-blue-100 dark:bg-blue-900/30 text-blue-800 dark:text-blue-200 text-center text-sm font-semibold flex items-center justify-center gap-2">
            <span>🔄</span> Pending Sync ({offlineActionsCount})
          </div>
        )}
        {actionStatus && <div className="mb-4 app-badge good" role="status">{actionStatus}</div>}

        {loading ? (
          <div className="flex justify-center py-10">
            <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-gray-900"></div>
          </div>
        ) : approvals.length === 0 ? (
          <div className="app-empty">
            <h3 className="text-lg font-bold text-gray-900 mb-2">All Caught Up!</h3>
            <p className="text-sm text-gray-600">There are no pending actions for you right now.</p>
          </div>
        ) : (
          <div className="space-y-4">
            {approvals.map(approval => {
              const payload = extractPayload(approval.description, approval.payload);
              const isPromo = payload?.context?.actionable_suggestion || payload?.context?.summary;
              const title = cleanDescription(approval.description);

              return (
                <div key={approval.id} className="app-card flex flex-col gap-4 p-4 border border-blue-100 bg-blue-50/30 rounded-2xl shadow-sm">
                  <div className="flex items-center gap-2 text-blue-800 font-semibold text-sm">
                    <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 10V3L4 14h7v7l9-11h-7z" /></svg>
                    Pending Approval
                  </div>

                  <div>
                    <h3 className="font-bold text-gray-900 mb-1">{title}</h3>
                    {isPromo && (
                      <div className="mt-3 p-3 bg-white rounded-xl border border-gray-100 text-sm text-gray-700 italic shadow-sm">
                        <p className="mb-2"><strong>Summary:</strong> {payload.context.summary}</p>
                        <p><strong>Action:</strong> {payload.context.actionable_suggestion}</p>
                      </div>
                    )}
                    {payload?.context?.custom_quote && (
                       <div className="mt-3 p-3 bg-white rounded-xl border border-gray-100 text-sm text-gray-700 shadow-sm">
                          <p><strong>Quote ID:</strong> {payload.context.quote_id}</p>
                          <p><strong>Total Amount:</strong> ${payload.context.total_amount}</p>
                          <p><strong>Customer:</strong> {payload.context.customer_name || 'N/A'}</p>
                          <div className="mt-2 text-blue-600"><a href={`/proposals/${payload.context.quote_id}`}>Review Line Items</a></div>
                       </div>
                    )}
                    {payload?.context?.smart_pricing && (
                       <div className="mt-3 p-3 bg-white rounded-xl border border-gray-100 text-sm text-gray-700 shadow-sm">
                          <p><strong>Product:</strong> {payload.context.product_name}</p>
                          <p><strong>Proposed Price:</strong> ${payload.context.new_price} (was ${payload.context.old_price})</p>
                          <p><strong>Projection:</strong> {payload.context.sales_projection}</p>
                       </div>
                    )}
                  </div>

                  <div className="flex gap-3 mt-2">
                    <button
                      onClick={() => handleDismiss(approval.id)}
                      className="flex-1 py-3 px-4 rounded-xl font-semibold text-sm bg-white border border-gray-200 text-gray-700 hover:bg-gray-50 active:scale-[0.98] transition-all min-h-[44px]"
                    >
                      Dismiss
                    </button>
                    <button
                      onClick={() => handleApprove(approval.id)}
                      className="flex-1 py-3 px-4 rounded-xl font-bold text-sm bg-[#0066FF] text-white hover:bg-[#0052CC] shadow-md shadow-[#0066FF]/20 active:scale-[0.98] transition-all min-h-[44px]"
                    >
                      {payload?.context?.custom_quote ? 'Send Quote' : 'Approve & Send'}
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
