'use client';
import React, { useEffect, useState } from 'react';
import Link from 'next/link';

type OHCLedgerEntry = {
  id: string;
  tenant_id: string;
  event_type: string;
  department: string;
  payload: any;
  created_at: string;
};

export default function AgentAuditDashboard() {
  const [activities, setActivities] = useState<OHCLedgerEntry[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    let mounted = true;
    async function fetchActivities() {
      try {
        const tenant = typeof window !== "undefined" ? localStorage.getItem("tenant_id") || localStorage.getItem("tenant") || "default" : "default";
        const res = await fetch(`/api/agents/approvals/ledger?tenant_id=${tenant}`, {
            headers: {
              "x-tenant-id": tenant,
              "x-user-id": "default",
            },
        });
        if (res.ok) {
            const data = await res.json();
            if (mounted && data.entries) {
                setActivities(data.entries);
            }
        }
      } catch (err) {
        console.error("Failed to load activities", err);
      } finally {
        if (mounted) setLoading(false);
      }
    }
    fetchActivities();
    return () => { mounted = false; };
  }, []);

 return (
 <div className="min-h-screen bg-gray-50 text-gray-900 p-8 font-inter">
 <header className="mb-8 flex items-center justify-between">
 <div className="flex items-center gap-4">
 <Link href="/inbox" className="text-blue-500 hover:text-blue-700">
 &lt; Back to Inbox
 </Link>
 <h1 className="text-3xl font-bold font-outfit text-gray-900">Agent Audit Dashboard</h1>
 </div>
 </header>

 <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
 <div className="md:col-span-2 space-y-6">
 <section className="app-panel">
 <div className="app-panel-header">
 <h2 className="app-panel-title">Cost Tracker</h2>
 </div>
 <div className="app-panel-body">
 <div className="text-4xl font-bold text-gray-900">$1,245.00</div>
 <div className="text-sm text-gray-500 mt-2">Total organizational spend</div>
 </div>
 </section>

 <section className="app-card">
 <h2 className="text-xl font-bold font-outfit mb-4">Operations</h2>
 <div className="flex items-center gap-4">
 <div className="w-3 h-3 rounded-full bg-green-500" ></div>
 <div>Agent Health: Optimal</div>
 </div>
 </section>

 <section className="app-card">
 <h2 className="text-xl font-bold font-outfit mb-4">Marketing & Advertising</h2>
 <div className="flex items-center gap-4">
 <div className="w-3 h-3 rounded-full bg-green-500" ></div>
 <div>Campaigns Sync: Active</div>
 </div>
 </section>
 </div>

 <div className="md:col-span-1">
 <section className="app-panel h-full">
 <div className="app-panel-header">
 <h2 className="app-panel-title text-indigo-600">Cross-Agent Feed</h2>
 </div>
 <div className="app-panel-body space-y-4">
    {loading && <div className="text-sm text-gray-500">Loading...</div>}
    {!loading && activities.length === 0 && <div className="text-sm text-gray-500">No activities found.</div>}
    {activities.map((activity) => {
        let desc = 'Action completed';
        try {
            const p = typeof activity.payload === 'string' ? JSON.parse(activity.payload) : activity.payload;
            desc = p?.original_payload?.description || desc;
        } catch (e) {}

        return (
            <div key={activity.id} className="p-3 glassmorphism bg-white/50 dark:bg-black/50 rounded-lg text-sm text-gray-800 dark:text-gray-200">
                <span className="font-bold text-indigo-600 dark:text-indigo-400">[{new Date(activity.created_at).toLocaleTimeString()}]</span> {desc} ({activity.department})
            </div>
        );
    })}
 </div>
 </section>
 </div>
 </div>
 </div>
 );
}
