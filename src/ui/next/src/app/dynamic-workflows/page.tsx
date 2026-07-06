"use client";

import { useState } from "react";

export default function DynamicWorkflowsPage() {
  const [prompt, setPrompt] = useState("");
  const [loading, setLoading] = useState(false);
  const [workflowState, setWorkflowState] = useState<any>(null);
  const [error, setError] = useState<string | null>(null);

  const startWorkflow = async () => {
    setLoading(true);
    setError(null);
    try {
      const res = await fetch("/api/v1/dynamic-workflows", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ prompt, tenant_id: "default" })
      });
      const data = await res.json();
      if (!res.ok) throw new Error(data.error || "Failed to start workflow");
      setWorkflowState(data);
    } catch (e: any) {
      setError(e.message);
    } finally {
      setLoading(false);
    }
  };

  const confirmWorkflow = async (id: string) => {
    setLoading(true);
    setError(null);
    try {
      const res = await fetch(`/api/v1/dynamic-workflows/${id}/confirm`, {
        method: "POST"
      });
      const data = await res.json();
      if (!res.ok) throw new Error(data.error || "Failed to confirm workflow");
      setWorkflowState(data);
    } catch (e: any) {
      setError(e.message);
    } finally {
      setLoading(false);
    }
  };

  const refreshWorkflow = async (id: string) => {
    setLoading(true);
    setError(null);
    try {
      const res = await fetch(`/api/v1/dynamic-workflows/${id}`);
      const data = await res.json();
      if (!res.ok) throw new Error(data.error || "Failed to fetch workflow");
      setWorkflowState(data);
    } catch (e: any) {
      setError(e.message);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="p-4 md:p-8 max-w-4xl mx-auto min-h-screen bg-[#F5F5F7]">
      <h1 className="text-3xl font-bold mb-2 text-[#1D1D1F] tracking-tight">Dynamic Workflows Orchestrator</h1>
      <p className="mb-8 text-[#86868B] text-lg">Orchestrate subagents at scale with dynamic workflows</p>

      <div className="flex flex-col gap-6 mb-8 p-6 rounded-2xl bg-white shadow-sm">
        <label className="font-medium text-[#1D1D1F]">Task Prompt:</label>
        <textarea
          className="border border-[#D2D2D7] rounded-xl px-4 py-3 min-h-[120px] focus:outline-none focus:ring-2 focus:ring-[#0071E3]"
          placeholder="e.g. Audit every route handler under src/routes/ for missing authentication checks..."
          value={prompt}
          onChange={(e) => setPrompt(e.target.value)}
        />
        <button
          className="bg-[#0071E3] hover:bg-[#0077ED] text-white px-6 py-2.5 rounded-full shadow-sm font-medium self-end disabled:opacity-50"
          onClick={startWorkflow}
          disabled={loading || !prompt}
        >
          {loading ? "Processing..." : "Generate Workflow"}
        </button>
      </div>

      {error && (
        <div className="p-4 bg-red-50 text-red-700 rounded-xl mb-6 shadow-sm border border-red-200">
          {error}
        </div>
      )}

      {workflowState && (
        <div className="p-6 rounded-2xl bg-white shadow-sm">
          <div className="flex justify-between items-center mb-4">
             <h2 className="text-xl font-bold">Workflow Status: {workflowState.status}</h2>
             <button
               className="bg-gray-100 hover:bg-gray-200 text-gray-800 px-4 py-2 rounded-full font-medium"
               onClick={() => refreshWorkflow(workflowState.id)}
             >
               Refresh
             </button>
          </div>

          <div className="bg-gray-50 p-4 rounded-xl font-mono text-sm overflow-auto max-h-[400px] mb-4">
            {workflowState.script ? (
               <pre>{workflowState.script}</pre>
            ) : (
               <pre>{JSON.stringify(workflowState, null, 2)}</pre>
            )}
          </div>

          {workflowState.status === "pending_confirmation" && (
            <button
              className="w-full bg-green-600 hover:bg-green-700 text-white px-6 py-3 rounded-xl shadow-sm font-medium disabled:opacity-50"
              onClick={() => confirmWorkflow(workflowState.id)}
              disabled={loading}
            >
              Approve & Run Workflow
            </button>
          )}
        </div>
      )}
    </div>
  );
}
