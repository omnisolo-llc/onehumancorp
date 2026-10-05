"use client";

import { errorMessage } from '@/lib/errors';

import { useState } from "react";

type WorkflowPlan = {
  id: string; tenant_id: string; prompt: string; status: string;
  requires_confirmation: boolean; tasks: unknown[];
};

function record(value: unknown): value is Record<string, unknown> {
  return value !== null && typeof value === 'object' && !Array.isArray(value);
}

async function readPlan(response: Response, envelope: boolean, failure: string, expected?: WorkflowPlan): Promise<WorkflowPlan> {
  let payload: unknown;
  try { payload = await response.json(); } catch { payload = undefined; }
  if (!response.ok) {
    throw new Error(record(payload) && typeof payload.error === 'string' && payload.error.trim()
      ? payload.error : `${failure} (HTTP ${response.status})`);
  }
  const plan = envelope && record(payload) ? payload.plan : payload;
  const invalid = () => new Error('Backend returned an invalid workflow response');
  if (!record(plan) || typeof plan.id !== 'string' || !/^dwf-[0-9a-f]{8}(?:-[0-9a-f]{4}){3}-[0-9a-f]{12}$/.test(plan.id)
    || typeof plan.tenant_id !== 'string' || !plan.tenant_id.trim()
    || typeof plan.prompt !== 'string' || !plan.prompt.trim()
    || !['awaiting_confirmation', 'queued', 'running', 'completed', 'failed'].includes(String(plan.status))
    || typeof plan.requires_confirmation !== 'boolean' || !Array.isArray(plan.tasks)
    || (plan.status === 'awaiting_confirmation') !== plan.requires_confirmation
    || expected && (plan.id !== expected.id || plan.tenant_id !== expected.tenant_id)) throw invalid();
  if (envelope && (!record(payload) || !Number.isSafeInteger(payload.enqueued_jobs)
    || (payload.enqueued_jobs as number) < 0
    || plan.status === 'awaiting_confirmation' && payload.enqueued_jobs !== 0)) throw invalid();
  return plan as WorkflowPlan;
}

export default function DynamicWorkflowsPage() {
  const [prompt, setPrompt] = useState("");
  const [loading, setLoading] = useState(false);
  const [workflowState, setWorkflowState] = useState<WorkflowPlan | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [needsRefresh, setNeedsRefresh] = useState(false);

  const startWorkflow = async () => {
    setLoading(true);
    setError(null);
    setWorkflowState(null);
    setNeedsRefresh(false);
    try {
      const res = await fetch("/api/v1/dynamic-workflows", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ prompt })
      });
      setWorkflowState(await readPlan(res, true, 'Failed to start workflow'));
    } catch (e: unknown) {
      setError(errorMessage(e));
    } finally {
      setLoading(false);
    }
  };

  const confirmWorkflow = async (plan: WorkflowPlan) => {
    setLoading(true);
    setError(null);
    try {
      const res = await fetch(`/api/v1/dynamic-workflows/${plan.id}/confirm`, {
        method: "POST"
      });
      setWorkflowState(await readPlan(res, true, 'Failed to confirm workflow', plan));
    } catch (e: unknown) {
      setNeedsRefresh(true);
      setError(errorMessage(e));
    } finally {
      setLoading(false);
    }
  };

  const refreshWorkflow = async (plan: WorkflowPlan) => {
    setLoading(true);
    setError(null);
    try {
      const res = await fetch(`/api/v1/dynamic-workflows/${plan.id}`);
      const refreshed = await readPlan(res, false, 'Failed to fetch workflow', plan);
      setWorkflowState(refreshed);
      // An awaiting plan cannot rule out a queue commit whose acknowledgement was lost.
      if (refreshed.status !== 'awaiting_confirmation') setNeedsRefresh(false);
    } catch (e: unknown) {
      setError(errorMessage(e));
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
        <div role="alert" className="p-4 bg-red-50 text-red-700 rounded-xl mb-6 shadow-sm border border-red-200">
          {error}
        </div>
      )}

      {workflowState && (
        <div className="p-6 rounded-2xl bg-white shadow-sm">
          <div className="flex justify-between items-center mb-4">
             <h2 className="text-xl font-bold">Workflow Status: {workflowState.status}</h2>
             <button
               className="bg-gray-100 hover:bg-gray-200 text-gray-800 px-4 py-2 rounded-full font-medium"
               onClick={() => refreshWorkflow(workflowState)}
               disabled={loading}
             >
               Refresh
             </button>
          </div>

          {workflowState.status === 'awaiting_confirmation' && !needsRefresh && <p>Plan saved. No work has been queued.</p>}
          {workflowState.status === 'queued' && <p>Workflow queued. Execution and completion are not verified.</p>}
          {needsRefresh && <p>Confirmation outcome is unconfirmed. Approval remains disabled. Refresh to check its status.</p>}
          <div className="bg-gray-50 p-4 rounded-xl font-mono text-sm overflow-auto max-h-[400px] mb-4">
            <pre>{JSON.stringify(workflowState, null, 2)}</pre>
          </div>

          {workflowState.status === "awaiting_confirmation" && (
            <button
              className="w-full bg-green-600 hover:bg-green-700 text-white px-6 py-3 rounded-xl shadow-sm font-medium disabled:opacity-50"
              onClick={() => confirmWorkflow(workflowState)}
              disabled={loading || needsRefresh}
            >
              Approve & Queue Workflow
            </button>
          )}
        </div>
      )}
    </div>
  );
}
