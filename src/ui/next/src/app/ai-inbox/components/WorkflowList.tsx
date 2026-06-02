"use client";

import React from 'react';

export type AIaaSWorkflow = {
  workflow_id: string;
  persona_id: string;
  trigger_event: string;
  status: string;
};

type Props = {
  workflows: AIaaSWorkflow[];
};

export default function WorkflowList({ workflows }: Props) {
  if (workflows.length === 0) {
    return (
      <div className="text-center py-10 bg-white/65 backdrop-blur-[30px] rounded-[16px] border border-white/40 shadow-sm text-gray-500 text-sm">
        No active workflows.
      </div>
    );
  }

  return (
    <div className="space-y-4">
      {workflows.map((wf) => (
        <div key={wf.workflow_id} className="p-4 bg-white/80 backdrop-blur-[20px] rounded-2xl border border-gray-200 shadow-sm flex items-center justify-between">
          <div>
            <h3 className="font-semibold text-gray-900 text-sm">{wf.trigger_event}</h3>
            <p className="text-xs text-gray-500 mt-1">Persona: {wf.persona_id}</p>
          </div>
          <div className="flex items-center gap-2">
            <span className={`px-2 py-1 rounded-full text-[10px] font-bold uppercase tracking-wider ${wf.status === 'active' ? 'bg-green-100 text-green-700' : 'bg-gray-100 text-gray-600'}`}>
              {wf.status}
            </span>
          </div>
        </div>
      ))}
    </div>
  );
}
