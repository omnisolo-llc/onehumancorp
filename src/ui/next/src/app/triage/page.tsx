"use client";

import { useEffect, useState, useMemo } from "react";
import { AppShell } from "../components/AppShell";

type TriageItem = {
  id: string;
  source: string;
  content: string;
  original_content: string;
  translated_from_language?: string;
  draft_reply?: string;
  status: string;
  created_at: string;
};

function tenantId() {
  if (typeof window === "undefined") return "default";
  return localStorage.getItem("tenant_id") || localStorage.getItem("tenant") || "default";
}

function getTone(status: string) {
  const normalized = (status || "").toLowerCase();
  if (["closed", "sent", "resolved"].includes(normalized)) return "good";
  if (["open", "pending", ""].includes(normalized)) return "warn";
  if (["failed", "blocked"].includes(normalized)) return "bad";
  return "neutral";
}

export default function WorkTriagePage() {
  const [items, setItems] = useState<TriageItem[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState("");
  const [actionStatus, setActionStatus] = useState("");

  useEffect(() => {
    async function fetchItems() {
      setLoading(true);
      setError("");
      try {
        const res = await fetch(`/api/ui/inbox/messages?tenant_id=${encodeURIComponent(tenantId())}`);
        if (!res.ok) throw new Error("Failed to load triage items");
        const data = await res.json();
        setItems(Array.isArray(data) ? data : []);
      } catch (e: any) {
        setError(e?.message || "Failed to load triage items");
      } finally {
        setLoading(false);
      }
    }
    fetchItems();
  }, []);

  const openCount = useMemo(() => items.filter((item) => !["closed", "resolved", "sent"].includes((item.status || "").toLowerCase())).length, [items]);

  async function handleApproveAndSend(inboxMessageId: string) {
    try {
      const token = localStorage.getItem("token") || "";
      const res = await fetch(`/api/agents/approvals?limit=50`, {
        headers: { "Authorization": `Bearer ${token}` }
      });
      if (!res.ok) throw new Error("Failed to fetch approvals");
      const data = await res.json();
      const pendingApprovals = data.pending_approvals || [];

      const approval = pendingApprovals.find((a: any) => {
        try {
          const payload = typeof a.payload === 'string' ? JSON.parse(a.payload) : a.payload;
          return payload && payload.inbox_message_id === inboxMessageId;
        } catch (e) {
          return false;
        }
      });

      if (!approval) {
        setActionStatus("Could not find a pending approval for this message.");
        return;
      }

      const approveRes = await fetch(`/api/agents/approvals/${approval.id}`, {
        method: "POST",
        headers: { "Content-Type": "application/json", "Authorization": `Bearer ${token}` },
        body: JSON.stringify({ approved: true })
      });

      if (approveRes.ok) {
        setItems((prev) => prev.map((m) => m.id === inboxMessageId ? { ...m, status: "sent" } : m));
        setActionStatus("Draft approved and sent.");
      } else {
        setActionStatus("Failed to approve and send message.");
      }
    } catch (e) {
      console.error(e);
      setActionStatus("Error approving message.");
    }
  }

  // Group items by status
  const urgentItems = items.filter(i => getTone(i.status) === "warn");
  const completedItems = items.filter(i => getTone(i.status) === "good");

  return (
    <AppShell
      title="Work Triage"
      subtitle="Your intelligent command center for prioritizing work."
      statusItems={[
        { label: "Action Required", value: String(openCount), tone: openCount > 0 ? "warn" : "good" },
        { label: "Total", value: String(items.length), tone: "neutral" }
      ]}
    >
      {actionStatus && <div className="mb-6 app-badge good" role="status">{actionStatus}</div>}

      <div className="max-w-3xl mx-auto flex flex-col gap-8">
        {/* Urgent items */}
        <section>
          <h2 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-4">Needs Your Attention</h2>
          {error && <div className="app-empty">{error}</div>}
          {!error && loading ? (
             <div className="app-empty">Loading your work feed...</div>
          ) : !error && urgentItems.length === 0 ? (
             <div className="app-empty">You're all caught up! No urgent work items.</div>
          ) : (
            <div className="flex flex-col gap-4">
              {urgentItems.map(item => (
                <div key={item.id} className="glassmorphism rounded-[16px] border border-white/40 dark:border-white/10 overflow-hidden shadow-sm flex flex-col">
                  {/* Card Header */}
                  <div className="px-6 py-4 bg-white/50 dark:bg-black/20 border-b border-gray-100 dark:border-white/5 flex justify-between items-center">
                     <div className="flex items-center gap-3">
                       <span className="text-xl">{item.source?.toLowerCase().includes("instagram") ? "📷" : item.source?.toLowerCase().includes("whatsapp") ? "💬" : "✉️"}</span>
                       <div>
                         <div className="font-semibold text-gray-900 dark:text-gray-100">{item.source || "Unknown Context"}</div>
                         <div className="text-xs text-gray-500">{new Date(item.created_at).toLocaleString()}</div>
                       </div>
                     </div>
                     <span className={`app-badge warn`}>Action Needed</span>
                  </div>
                  {/* Card Body */}
                  <div className="p-6 flex flex-col gap-5">
                    {/* Context / Original Message */}
                    <div>
                      <div className="text-xs font-bold text-gray-400 uppercase tracking-wider mb-2">Customer Inquiry</div>
                      <p className="text-sm text-gray-800 dark:text-gray-200 bg-gray-50 dark:bg-gray-800/50 p-3 rounded-lg border border-gray-100 dark:border-gray-700">
                        "{item.original_content || item.content}"
                      </p>
                    </div>

                    {/* AI Proposed Draft */}
                    {item.draft_reply && (
                      <div>
                        <div className="flex items-center gap-2 mb-2">
                           <span className="text-xs font-bold text-blue-500 uppercase tracking-wider flex items-center gap-1">
                             <svg className="w-3 h-3" fill="currentColor" viewBox="0 0 20 20"><path d="M10 2a8 8 0 100 16 8 8 0 000-16zm1 11H9v-2h2v2zm0-4H9V5h2v4z"></path></svg>
                             AI Proposed Reply
                           </span>
                        </div>
                        <p className="text-sm text-gray-800 dark:text-gray-200 bg-blue-50/50 dark:bg-blue-900/10 p-3 rounded-lg border border-blue-100 dark:border-blue-800/30">
                          {item.draft_reply}
                        </p>
                      </div>
                    )}

                    {/* Action buttons */}
                    <div className="flex flex-col sm:flex-row gap-3 pt-2">
                      <button
                        onClick={() => handleApproveAndSend(item.id)}
                        className="flex-1 min-h-[44px] px-4 rounded-[8px] bg-[#0066FF] text-white font-medium hover:bg-[#0052CC] transition-colors shadow-md flex items-center justify-center gap-2"
                        aria-label="Approve Draft"
                      >
                        ✨ Approve & Send Draft
                      </button>
                    </div>
                  </div>
                </div>
              ))}
            </div>
          )}
        </section>

        {/* Recently completed items */}
        {completedItems.length > 0 && (
           <section>
              <h2 className="text-lg font-bold font-outfit text-gray-600 dark:text-gray-400 mb-4 flex items-center gap-2">
                <svg className="w-5 h-5 text-green-500" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M5 13l4 4L19 7"></path></svg>
                Recently Handled
              </h2>
              <div className="flex flex-col gap-3">
                 {completedItems.map(item => (
                   <div key={item.id} className="bg-gray-50/50 dark:bg-gray-900/30 rounded-[12px] p-4 flex justify-between items-center opacity-80 border border-gray-100 dark:border-gray-800">
                     <div className="flex flex-col gap-1">
                        <span className="text-sm font-medium text-gray-800 dark:text-gray-300">{item.source}</span>
                        <span className="text-xs text-gray-500 truncate max-w-[250px]">{item.content}</span>
                     </div>
                     <span className="app-badge good">{item.status}</span>
                   </div>
                 ))}
              </div>
           </section>
        )}
      </div>
    </AppShell>
  );
}
