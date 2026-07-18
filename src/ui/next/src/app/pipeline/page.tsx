"use client";

import { useEffect, useState } from "react";
import { AppShell } from "../components/AppShell";

type Opportunity = {
  id: string;
  tenant_id: string;
  lead_id?: string;
  title: string;
  stage: string;
  estimated_value?: number;
  priority?: string;
  created_at?: string;
  updated_at?: string;
};

const STAGES = ["Qualified", "Proposal", "Negotiation", "Won", "Lost"];

function tenantId() {
  if (typeof window === "undefined") return "default";
  return localStorage.getItem("tenant_id") || localStorage.getItem("tenant") || "default";
}

function money(cents: number | undefined) {
  if (cents == null) return "$0.00";
  return new Intl.NumberFormat("en-US", {
    style: "currency",
    currency: "USD",
  }).format(cents / 100);
}

export default function PipelinePage() {
  const [opportunities, setOpportunities] = useState<Opportunity[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState("");

  useEffect(() => {
    loadPipeline();
  }, []);

  async function loadPipeline() {
    setLoading(true);
    setError("");
    try {
      const res = await fetch(`/api/ui/opportunities?tenant_id=${encodeURIComponent(tenantId())}`);
      if (!res.ok) throw new Error("Failed to load pipeline opportunities");
      const data = await res.json();
      setOpportunities(Array.isArray(data) ? data : []);
    } catch (e: any) {
      setError(e?.message || "Failed to load opportunities");
    } finally {
      setLoading(false);
    }
  }

  async function updateStage(opportunityId: string, newStage: string) {
    try {
      const res = await fetch(`/api/ui/opportunities/stage`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ opportunity_id: opportunityId, stage: newStage }),
      });
      if (!res.ok) throw new Error("Failed to update opportunity stage");

      setOpportunities(prev =>
        prev.map(opp =>
          opp.id === opportunityId ? { ...opp, stage: newStage } : opp
        )
      );
    } catch (e) {
      console.error(e);
    }
  }

  const getOpportunitiesByStage = (stageName: string) => {
    return opportunities.filter(opp => opp.stage === stageName);
  };

  return (
    <AppShell
      title="Sales Pipeline"
      subtitle="Track leads and active proposals across stages."
      statusItems={[
        { label: "Active Deals", value: String(opportunities.filter(o => !["Won", "Lost"].includes(o.stage)).length), tone: "good" }
      ]}
    >
      <div className="mb-6 p-6 glassmorphism border border-white/40 dark:border-white/10">
        <h2 className="text-2xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Deal Pipeline</h2>
        <p className="text-gray-600 dark:text-gray-400">Manage high-consideration projects from inquiry to revenue.</p>
      </div>

      {error && <div className="mb-4 app-badge bad" role="status">{error}</div>}

      <div className="flex overflow-x-auto gap-6 pb-4 snap-x">
        {STAGES.map((stage) => {
          const stageOpps = getOpportunitiesByStage(stage);
          const stageTotal = stageOpps.reduce((sum, o) => sum + (o.estimated_value || 0), 0);

          return (
            <div key={stage} className="min-w-[300px] flex-1 snap-center flex flex-col">
              <div className="flex justify-between items-center mb-4">
                <h3 className="font-bold text-gray-800 dark:text-gray-200">{stage}</h3>
                <div className="text-sm font-semibold text-gray-500">{stageOpps.length}</div>
              </div>

              <div className="text-sm font-medium text-gray-500 mb-4">{money(stageTotal)}</div>

              <div className="flex flex-col gap-3 min-h-[200px] p-2 bg-gray-50/50 dark:bg-gray-900/20 rounded-xl border border-gray-200/50 dark:border-gray-800/50">
                {stageOpps.length === 0 && (
                  <div className="text-sm text-center text-gray-400 py-8">Empty</div>
                )}

                {stageOpps.map(opp => (
                  <div key={opp.id} className="p-4 rounded-xl glassmorphism border border-white/60 dark:border-white/10 shadow-sm flex flex-col gap-2 relative group cursor-pointer hover:shadow-md transition-shadow">
                    <div className="font-semibold text-gray-900 dark:text-white">{opp.title}</div>
                    <div className="flex justify-between text-sm">
                      <span className="text-green-600 dark:text-green-400 font-medium">{money(opp.estimated_value)}</span>
                      <span className={`text-xs px-2 py-0.5 rounded-full ${opp.priority === 'High' ? 'bg-red-100 text-red-700' : 'bg-gray-100 text-gray-600'}`}>{opp.priority || 'Normal'}</span>
                    </div>

                    <div className="flex gap-2 mt-2 opacity-0 group-hover:opacity-100 transition-opacity">
                      <select
                        className="text-xs bg-white border border-gray-200 rounded px-2 py-1 w-full"
                        value={opp.stage}
                        onChange={(e) => updateStage(opp.id, e.target.value)}
                        data-testid={`stage-select-${opp.id}`}
                      >
                        {STAGES.map(s => <option key={s} value={s}>{s}</option>)}
                      </select>
                    </div>
                  </div>
                ))}
              </div>
            </div>
          );
        })}
      </div>
    </AppShell>
  );
}
