"use client";

import { useProPlan } from './useProPlan';

export function ViralTrialExtensionWidget() {
  const { currentPlan, planError, claimTrial, claimError } = useProPlan();
  return <section className="mt-4 p-4 bg-indigo-50/50 rounded-xl border border-indigo-100" aria-label="Plan and trial availability">
    <h2 className="font-bold text-sm mb-2">Plan and trial availability</h2>
    <p className="text-xs mb-3">{currentPlan ? `Current verified plan: ${currentPlan}.` : planError ?? 'Verifying your current plan…'}</p>
    <p className="text-xs mb-3">Sharing does not confirm a trial grant or its duration.</p>
    <button type="button" className="app-button" onClick={() => void claimTrial()}>Check trial availability</button>
    {claimError && <p role="status" className="mt-2 text-xs">{claimError}</p>}
    <a href="/pricing" className="block mt-3 text-sm">Review plans</a>
  </section>;
}
