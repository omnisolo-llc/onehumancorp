"use client";

import { useEffect, useState } from "react";
import { AppShell } from "../components/AppShell";

function tenantId() {
  if (typeof window === "undefined") return "default";
  return localStorage.getItem("tenant_id") || localStorage.getItem("tenant") || "default";
}

type Opportunity = {
  id: string;
  tenant_id: string;
  lead_id: string;
  title: string;
  stage: string;
  estimated_value: number;
  priority: string;
  created_at: string;
};

export default function PipelinePage() {
  const [opportunities, setOpportunities] = useState<Opportunity[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState("");

  const stages = ["Qualified", "Proposal", "Negotiation", "Won", "Lost"];

  useEffect(() => {
    loadPipeline();
  }, []);

  async function loadPipeline() {
    setLoading(true);
    setError("");
    try {
      const res = await fetch(`/api/ui/pipeline?tenant_id=${encodeURIComponent(tenantId())}`);
      if (!res.ok) throw new Error("Failed to load pipeline");
      const data = await res.json();
      setOpportunities(data.opportunities || []);
    } catch (e: any) {
      setError(e?.message || "Failed to load pipeline");
    } finally {
      setLoading(false);
    }
  }

  function getOppsByStage(stage: string) {
    return opportunities.filter((o) => o.stage === stage);
  }

  return (
    <AppShell
      title="Sales Pipeline"
      subtitle="Track leads and opportunities across the sales cycle."
    >
      {error && <div className="mb-4 app-badge bad">{error}</div>}

      <div className="flex overflow-x-auto pb-6 space-x-4 snap-x">
        {stages.map((stage) => (
          <div key={stage} className="snap-center min-w-[280px] w-[280px] flex-shrink-0 flex flex-col bg-white/40 dark:bg-white/5 backdrop-blur-xl border border-white/40 dark:border-white/10 rounded-2xl p-4">
            <div className="flex justify-between items-center mb-4">
              <h3 className="font-semibold text-gray-900 dark:text-white">{stage}</h3>
              <span className="bg-gray-200 dark:bg-gray-800 text-xs font-medium px-2.5 py-0.5 rounded-full text-gray-600 dark:text-gray-400">
                {getOppsByStage(stage).length}
              </span>
            </div>

            <div className="space-y-3 flex-1 overflow-y-auto">
              {loading ? (
                <div className="text-sm text-gray-500 text-center py-4">Loading...</div>
              ) : getOppsByStage(stage).length === 0 ? (
                <div className="text-sm text-gray-500 text-center py-4 italic border-2 border-dashed border-gray-200 dark:border-gray-800 rounded-xl">Empty</div>
              ) : (
                getOppsByStage(stage).map((opp) => (
                  <div key={opp.id} className="bg-white dark:bg-gray-800 shadow-sm border border-gray-100 dark:border-gray-700 rounded-xl p-4 transition-all hover:shadow-md cursor-pointer" data-testid={`pipeline-card-${opp.id}`}>
                    <div className="flex justify-between items-start mb-2">
                      <h4 className="font-medium text-sm text-gray-900 dark:text-white line-clamp-2 leading-tight">{opp.title}</h4>
                    </div>
                    <div className="flex justify-between items-center mt-3">
                      <span className="text-sm font-semibold text-green-600 dark:text-green-400">${Number(opp.estimated_value).toFixed(2)}</span>
                      <span className={`text-xs px-2 py-1 rounded-md font-medium ${
                        opp.priority === 'High' ? 'bg-red-100 text-red-700 dark:bg-red-900/30 dark:text-red-400' :
                        opp.priority === 'Medium' ? 'bg-yellow-100 text-yellow-700 dark:bg-yellow-900/30 dark:text-yellow-400' :
                        'bg-blue-100 text-blue-700 dark:bg-blue-900/30 dark:text-blue-400'
                      }`}>
                        {opp.priority}
                      </span>
                    </div>
                  </div>
                ))
              )}
            </div>
          </div>
        ))}
      </div>
    </AppShell>
  );
}
