"use client";

import Link from 'next/link';
import { useProPlan } from '../components/useProPlan';

export default function TrialExtensionPage() {
  const { currentPlan, planError, claimTrial, claimError, refreshPlan } = useProPlan();
  return <main className="min-h-screen bg-gray-50 dark:bg-gray-950 p-6 md:p-12">
    <section className="max-w-2xl mx-auto rounded-2xl bg-white dark:bg-gray-900 border p-8 space-y-4">
      <h1 className="text-3xl font-bold">Plan and Trial Availability</h1>
      <p role="status" aria-label="Current plan">{currentPlan ? `Current verified plan: ${currentPlan}.` : planError ?? 'Verifying your current plan…'}</p>
      <p>A current plan can be checked here. Sharing does not verify a new grant or a trial duration.</p>
      <button type="button" className="app-button" onClick={() => void claimTrial()}>Check trial availability</button>
      {claimError && <p role="status">{claimError}</p>}
      <button type="button" className="app-button" onClick={() => void refreshPlan()}>Refresh current plan</button>
      <div className="flex gap-4"><Link href="/pricing">Review plans</Link><Link href="/dashboard">Back to Dashboard</Link></div>
    </section>
  </main>;
}
