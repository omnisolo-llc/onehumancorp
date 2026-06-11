"use client";

import { useEffect, useState } from "react";
import { AppShell } from "../../components/AppShell";
import { CheckCircleIcon, ExclamationTriangleIcon } from "@heroicons/react/24/outline";

type Incident = {
  id: string;
  tenant_id: string;
  title: string;
  description: string;
  status: string;
  resolution_plan: {
    actions: { type: string; payload: string; description: string }[];
    summary: string;
  };
  created_at: string;
};

function tenantId() {
  if (typeof window === "undefined") return "default";
  return localStorage.getItem("tenant_id") || localStorage.getItem("tenant") || "default";
}

export default function IncidentRoomPage() {
  const [incidents, setIncidents] = useState<Incident[]>([]);
  const [loading, setLoading] = useState(true);
  const [actionStatus, setActionStatus] = useState("");
  const [error, setError] = useState("");
  const [isBottomsheetOpen, setIsBottomsheetOpen] = useState(false);
  const [selectedIncident, setSelectedIncident] = useState<Incident | null>(null);

  useEffect(() => {
    loadIncidents();
  }, []);

  async function loadIncidents() {
    setLoading(true);
    setError("");
    try {
      const res = await fetch(`/api/incidents?tenant_id=${encodeURIComponent(tenantId())}`);
      if (!res.ok) throw new Error("Failed to load incidents");
      const data = await res.json();
      setIncidents(data);
    } catch (e: any) {
      setError(e?.message || "Failed to load incidents");
    } finally {
      setLoading(false);
    }
  }

  async function handleApprovePlan(id: string) {
    try {
      setActionStatus("Approving resolution plan...");
      const res = await fetch(`/api/incidents/${id}/approve?tenant_id=${encodeURIComponent(tenantId())}`, {
        method: "POST",
      });
      if (!res.ok) throw new Error("Failed to approve resolution plan");

      setActionStatus("Plan executed successfully.");
      setIsBottomsheetOpen(false);

      await loadIncidents();
      setTimeout(() => setActionStatus(""), 3000);
    } catch (e) {
      console.error(e);
      setActionStatus("Error approving plan.");
    }
  }

  function handleCardClick(incident: Incident) {
      setSelectedIncident(incident);
      setIsBottomsheetOpen(true);
  }

  return (
    <AppShell
      title="Incident Room"
      subtitle="Resolve anomalies and coordinate fixes instantly."
    >
      {actionStatus && <div className="mb-4 app-badge good" role="status">{actionStatus}</div>}
      {error && <div className="mb-4 app-badge bad" role="status">{error}</div>}

      <div className="max-w-2xl mx-auto w-full pb-20">
          <div className="mb-6">
              <h2 className="text-xl font-bold text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Urgent Incidents</h2>
              <p className="text-sm text-gray-500">Tap an incident to view the AI's proposed resolution plan.</p>
          </div>

          <div className="space-y-4">
              {loading ? (
                  <div className="text-center py-8 text-gray-500">Loading incidents...</div>
              ) : incidents.length === 0 ? (
                  <div className="text-center py-8 text-gray-500 glassmorphism rounded-xl">No active incidents.</div>
              ) : incidents.map(incident => (
                  <button
                      key={incident.id}
                      className="w-full text-left bg-white/65 dark:bg-[#16161A]/70 backdrop-blur-[30px] saturate-[210%] rounded-xl p-4 border border-white/40 dark:border-white/10 shadow-sm flex items-start space-x-3 transition hover:shadow-md active:scale-[0.98]"
                      onClick={() => handleCardClick(incident)}
                      data-testid={`incident-card-${incident.id}`}
                  >
                      <div className="mt-1">
                         <ExclamationTriangleIcon className="w-6 h-6 text-[#FF3B30] dark:text-[#DE1B1B]" />
                      </div>
                      <div className="flex-1">
                          <div className="flex justify-between items-start">
                             <h3 className="font-semibold text-[#1D1D1F] dark:text-[#F5F5F7] text-lg leading-tight">{incident.title}</h3>
                             <span className="app-badge bad whitespace-nowrap ml-2">Urgent</span>
                          </div>
                          <p className="mt-1 text-sm text-gray-700 dark:text-gray-300 line-clamp-2">{incident.description}</p>
                      </div>
                  </button>
              ))}
          </div>
      </div>

      {isBottomsheetOpen && selectedIncident && (
        <div className="fixed inset-0 z-50 flex items-end sm:items-center justify-center p-4 pb-0 sm:pb-4 pointer-events-auto">
          <div className="absolute inset-0 bg-black/40 backdrop-blur-sm transition-opacity" onClick={() => setIsBottomsheetOpen(false)}></div>

          <div className="relative w-full max-w-lg bg-white/80 dark:bg-[#16161A]/80 backdrop-blur-[30px] saturate-[210%] border border-white/40 dark:border-white/10 shadow-2xl rounded-t-[24px] sm:rounded-[24px] overflow-hidden transform transition-transform duration-300">

             <div className="p-6">
                <div className="flex justify-between items-center mb-4">
                    <h2 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7]">Resolution Plan</h2>
                    <button onClick={() => setIsBottomsheetOpen(false)} className="text-gray-500 hover:text-gray-700 dark:hover:text-gray-300">
                       <span className="sr-only">Close</span>
                       <svg className="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M6 18L18 6M6 6l12 12" /></svg>
                    </button>
                </div>

                <div className="mb-6 p-4 bg-gray-50/50 dark:bg-gray-800/50 rounded-xl border border-gray-100 dark:border-gray-700">
                   <h4 className="text-sm font-semibold text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Executive Summary</h4>
                   <p className="text-sm text-gray-700 dark:text-gray-300">{selectedIncident.resolution_plan?.summary || "AI is proposing a fix to address the incident."}</p>
                </div>

                <div className="space-y-4 mb-8">
                    <h4 className="text-sm font-semibold text-[#1D1D1F] dark:text-[#F5F5F7]">Proposed Actions</h4>
                    {selectedIncident.resolution_plan?.actions?.map((action, idx) => (
                        <div key={idx} className="flex space-x-3 items-start">
                            <div className="mt-0.5">
                                <CheckCircleIcon className="w-5 h-5 text-[#0066FF] dark:text-[#0071E3]" />
                            </div>
                            <div>
                                <p className="text-sm font-medium text-[#1D1D1F] dark:text-[#F5F5F7]">{action.description}</p>
                                {action.payload && (
                                   <div className="mt-1 text-xs text-gray-600 dark:text-gray-400 bg-gray-100/50 dark:bg-gray-800/50 p-2 rounded-md border border-gray-200 dark:border-gray-700">
                                       Draft: "{action.payload}"
                                   </div>
                                )}
                            </div>
                        </div>
                    ))}
                    {(!selectedIncident.resolution_plan?.actions || selectedIncident.resolution_plan.actions.length === 0) && (
                        <p className="text-sm text-gray-500">No specific actions proposed.</p>
                    )}
                </div>

                <div className="mt-4 pt-4 border-t border-gray-200 dark:border-gray-700 flex flex-col space-y-3">
                   <button
                       className="app-btn-primary w-full py-3 rounded-xl shadow-md flex justify-center items-center font-semibold text-[15px]"
                       onClick={() => handleApprovePlan(selectedIncident.id)}
                       data-testid="execute-plan-btn"
                   >
                       Execute Plan
                   </button>
                   <button
                       className="w-full py-3 rounded-xl text-gray-600 dark:text-gray-400 font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition"
                       onClick={() => setIsBottomsheetOpen(false)}
                   >
                       Cancel
                   </button>
                </div>
             </div>
          </div>
        </div>
      )}
    </AppShell>
  );
}
