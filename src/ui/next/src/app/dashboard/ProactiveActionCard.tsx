"use client";

import React, { useState } from "react";
import { useSWRConfig } from "swr";

export type ProactiveAction = {
  id: string;
  tenant_id: string;
  title: string;
  description: string;
  action_type: string;
  payload: any;
  status: string;
};

export default function ProactiveActionCard({ action }: { action: ProactiveAction }) {
  const [status, setStatus] = useState(action.status);
  const [loading, setLoading] = useState(false);

  const handleApprove = async () => {
    setLoading(true);
    try {
      const res = await fetch(`/api/proactive/actions/${action.id}/approve`, {
        method: "POST",
      });
      if (res.ok) {
        setStatus("approved");
      }
    } catch (e) {
      console.error(e);
    } finally {
      setLoading(false);
    }
  };

  const handleReject = async () => {
    setLoading(true);
    try {
      const res = await fetch(`/api/proactive/actions/${action.id}/reject`, {
        method: "POST",
      });
      if (res.ok) {
        setStatus("rejected");
      }
    } catch (e) {
      console.error(e);
    } finally {
      setLoading(false);
    }
  };

  if (status === "approved" || status === "rejected") {
    return (
      <div className="p-4 border rounded-lg shadow-sm bg-gray-50 flex items-center justify-between opacity-50 transition-opacity">
        <div>
          <h4 className="font-medium text-sm text-gray-500 line-through">{action.title}</h4>
        </div>
        <span className="text-xs font-semibold capitalize text-gray-500">{status}</span>
      </div>
    );
  }

  return (
    <div className="p-4 border border-blue-200 rounded-lg shadow-sm bg-blue-50/50 flex flex-col gap-3">
      <div>
        <div className="flex items-center gap-2 mb-1">
          <span className="bg-blue-100 text-blue-700 text-[10px] uppercase font-bold px-2 py-0.5 rounded-full tracking-wider">
            Proactive Action
          </span>
        </div>
        <h4 className="font-semibold text-gray-900">{action.title}</h4>
        <p className="text-sm text-gray-600 mt-1">{action.description}</p>
      </div>

      <div className="flex items-center gap-2 mt-2 w-full">
        <button
          onClick={handleApprove}
          disabled={loading}
          data-testid="proactive-action-approve"
          className="flex-1 bg-blue-600 hover:bg-blue-700 text-white font-medium py-2 px-4 rounded-md text-sm transition-colors disabled:opacity-50"
        >
          {loading ? "Approving..." : "Approve"}
        </button>
        <button
          onClick={handleReject}
          disabled={loading}
          data-testid="proactive-action-reject"
          className="flex-1 bg-white hover:bg-gray-50 text-gray-700 font-medium py-2 px-4 border border-gray-200 rounded-md text-sm transition-colors disabled:opacity-50"
        >
          Reject
        </button>
      </div>
    </div>
  );
}
