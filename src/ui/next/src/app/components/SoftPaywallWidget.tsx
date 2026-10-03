"use client";

import { useState } from 'react';
import { useProPlan } from './useProPlan';

export function SoftPaywallWidget() {
  const [isOpen, setIsOpen] = useState(false);
  const { currentPlan, planError, claimTrial, claimError } = useProPlan();
  return <>
    <section className="omnisolo-growth-card rounded-xl border bg-white dark:bg-gray-900 p-6 mb-6">
      <h2 className="text-xl font-bold">Advanced AI Automations</h2>
      <p>Review available agent tasks and workflow configuration in Agents.</p>
      <button type="button" className="app-button mt-3" onClick={() => setIsOpen(true)}>Review automation setup</button>
    </section>
    {isOpen && <div className="fixed inset-0 bg-black/40 z-50 flex items-center justify-center p-4">
      <section role="dialog" aria-modal="true" aria-label="Automation setup" className="bg-white dark:bg-gray-900 rounded-2xl p-8 max-w-md w-full">
        <h2 className="text-2xl font-bold">Review automation setup</h2>
        <p>{currentPlan ? `Current verified plan: ${currentPlan}.` : planError ?? 'Verifying your current plan…'}</p>
        <p>No automation is activated from this card. Open Agents to review available execution settings and authorize a task.</p>
        <div className="flex flex-col gap-3 mt-4">
          <a className="app-button" href="/agents">Open Agents</a>
          <a className="app-button" href="/pricing">Review plans</a>
          <button type="button" className="app-button" onClick={() => void claimTrial()}>Check trial availability</button>
          {claimError && <p role="status">{claimError}</p>}
          <button type="button" onClick={() => setIsOpen(false)}>Close setup</button>
        </div>
      </section>
    </div>}
  </>;
}
