"use client";

import { useEffect, useState } from "react";
import { AppShell } from "../components/AppShell";

type Opportunity = {
  id: string;
  tenant_id: string;
  lead_id?: string;
  title: string;
  stage: string;
  estimated_value_cents: number;
  priority: string;
};

function tenantId() {
  if (typeof window === "undefined") return "default";
  return localStorage.getItem("tenant_id") || localStorage.getItem("tenant") || "default";
}

export default function PipelinePage() {
  const [opportunities, setOpportunities] = useState<Opportunity[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState("");

  useEffect(() => {
    loadOpportunities();
  }, []);

  async function loadOpportunities() {
    setLoading(true);
    setError("");
    try {
      const res = await fetch(`/api/opportunities?tenant_id=${encodeURIComponent(tenantId())}`);
      if (!res.ok) throw new Error("Failed to load opportunities");
      const data = await res.json();
      setOpportunities(Array.isArray(data) ? data : []);
    } catch (e: any) {
      setError(e?.message || "Failed to load opportunities");
    } finally {
      setLoading(false);
    }
  }

  async function moveStage(id: string, stage: string) {
    try {
      const res = await fetch(`/api/opportunities/stage?tenant_id=${encodeURIComponent(tenantId())}`, {
        method: "PUT",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ id, stage })
      });
      if (!res.ok) throw new Error("Failed to update stage");
      await loadOpportunities();
    } catch (e) {
      console.error(e);
    }
  }

  const stages = ["Qualified", "Proposal", "Negotiation", "Won", "Lost"];

  return (
    <AppShell
      title="Sales Pipeline"
      subtitle="Track your opportunities from inquiry to close."
      statusItems={[
        { label: "Active Deals", value: String(opportunities.filter(o => !["Won", "Lost"].includes(o.stage)).length), tone: "good" }
      ]}
    >
      <div className="mb-6 p-6 rounded-[16px] glassmorphism border border-white/40 dark:border-white/10">
        <h2 className="text-2xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Deal Pipeline</h2>
        <p className="text-gray-600 dark:text-gray-400">Drag and drop opportunities between stages.</p>
      </div>

      <div className="flex gap-4 overflow-x-auto pb-4">
        {stages.map(stage => (
          <div key={stage} className="min-w-[300px] flex-1 bg-gray-50/50 dark:bg-gray-800/30 rounded-xl p-4 border border-gray-200 dark:border-gray-700">
            <h3 className="font-semibold text-gray-900 dark:text-gray-100 mb-4">{stage}</h3>
            <div className="space-y-3">
              {opportunities.filter(o => o.stage === stage).map(opp => (
                <div key={opp.id} className="bg-white dark:bg-gray-800 p-4 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700">
                  <div className="font-medium text-gray-900 dark:text-gray-100 mb-1">{opp.title}</div>
                  <div className="text-sm text-gray-500 mb-3">${(opp.estimated_value_cents / 100).toFixed(2)} - {opp.priority}</div>

                  <div className="flex gap-2">
                    {stages.indexOf(stage) > 0 && (
                      <button onClick={() => moveStage(opp.id, stages[stages.indexOf(stage) - 1])} className="text-xs px-2 py-1 bg-gray-100 dark:bg-gray-700 rounded hover:bg-gray-200" data-testid={`move-back-${opp.id}`}>← Move</button>
                    )}
                    {stages.indexOf(stage) < stages.length - 1 && (
                      <button onClick={() => moveStage(opp.id, stages[stages.indexOf(stage) + 1])} className="text-xs px-2 py-1 bg-gray-100 dark:bg-gray-700 rounded hover:bg-gray-200 ml-auto" data-testid={`move-forward-${opp.id}`}>Move →</button>
                    )}
                  </div>
                </div>
              ))}
              {opportunities.filter(o => o.stage === stage).length === 0 && (
                <div className="text-sm text-gray-400 text-center py-4" data-testid={`empty-stage-${stage}`}>No deals</div>
              )}
            </div>
          </div>
        ))}
      </div>
    </AppShell>
  );
}
