"use client";

import { useEffect, useState } from 'react';
import { notifyQueueIdentityChange } from '@/lib/sync/queueIdentity';
import { useProPlan } from './useProPlan';
type Savings = { hours_saved: number; inquiries_handled?: number; appointments_scheduled?: number };
function readSavings(value: unknown): Savings {
  if (!value || typeof value !== 'object' || Array.isArray(value)) throw new Error('Savings unavailable');
  const data = value as Record<string, unknown>;
  if (data.error != null || ('success' in data && data.success !== true) || typeof data.hours_saved !== 'number' || !Number.isFinite(data.hours_saved) || data.hours_saved < 0) throw new Error('Savings unavailable');
  for (const key of ['inquiries_handled', 'appointments_scheduled']) if (data[key] !== undefined && (!Number.isSafeInteger(data[key]) || Number(data[key]) < 0)) throw new Error('Savings unavailable');
  return data as Savings;
}
export default function AiTimeSavingsWidget() {
  const { claimTrial, claimError, verifiedOwner } = useProPlan();
  const userId = verifiedOwner?.userId; const tenantId = verifiedOwner?.tenantId;
  const [savings, setSavings] = useState<Savings | null>(null);
  const [error, setError] = useState(''); const [checking, setChecking] = useState(false);
  useEffect(() => {
    setSavings(null); setError('');
    if (!userId || !tenantId) return;
    const request = new AbortController(); let active = true;
    void (async () => {
      try {
        const headers = new Headers({ 'x-ohc-expected-user': userId, 'x-ohc-expected-tenant': tenantId });
        const response = await fetch('/api/v1/growth/time-savings', { headers, credentials: 'same-origin', cache: 'no-store', redirect: 'error', signal: request.signal });
        if (!active) return;
        if (response.status === 401 || response.status === 403) { notifyQueueIdentityChange(); return; }
        const value: unknown = await response.json();
        if (!active) return;
        const reason = value && typeof value === 'object' && 'error' in value ? value.error : undefined;
        if (response.status === 409 && (reason === 'session_identity_changed' || reason === 'queued owner does not match the current session')) { notifyQueueIdentityChange(); return; }
        if (response.status !== 200) throw new Error('Savings unavailable');
        setSavings(readSavings(value));
      } catch { if (active) setError('Recorded time-savings data is unavailable.'); }
    })();
    return () => { active = false; request.abort(); };
  }, [userId, tenantId]);
  const checkTrial = async () => {
    setChecking(true);
    try { await claimTrial(); }
    catch { setError('Trial activation is unavailable.'); }
    finally { setChecking(false); }
  };
  return <section className="rounded-xl border bg-white dark:bg-gray-900 p-6 mb-8" aria-label="Recorded time savings">
    <h2 className="text-xl font-bold">Recorded time savings</h2>
    {!verifiedOwner ? <p role="status">Verify your account to read time-savings data.</p> : error ? <p role="status">{error}</p> : !savings ? <p role="status">Loading recorded time-savings data…</p> : <>
      <p>Recorded estimate: {savings.hours_saved} hours saved.</p>
      <p>Customer inquiries handled: {savings.inquiries_handled ?? 'not reported'}. Appointments scheduled: {savings.appointments_scheduled ?? 'not reported'}.</p>
    </>}
    <p>Sharing does not verify a trial grant or its duration.</p>
    <button type="button" disabled={checking} onClick={() => void checkTrial()} className="app-button">{checking ? 'Checking…' : 'Check trial availability'}</button>
    {claimError && <p role="status">{claimError}</p>}
  </section>;
}
