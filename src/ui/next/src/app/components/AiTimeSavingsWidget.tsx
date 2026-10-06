"use client";

import { useEffect, useRef, useState } from 'react';
import { currentVerifiedQueueLease, hasPendingQueueOwnerVerification, hasVerifiedOfflineQueueOwner, notifyQueueIdentityChange, QUEUE_IDENTITY_EPOCH_KEY, queueIdentityGeneration, sameOwner, subscribeQueueIdentityReadiness } from '@/lib/sync/queueIdentity';
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
  const { claimTrial, claimError, verifiedOwner, ownerRevision } = useProPlan();
  const userId = verifiedOwner?.userId; const tenantId = verifiedOwner?.tenantId;
  const [savings, setSavings] = useState<Savings | null>(null);
  const [error, setError] = useState(''); const [checking, setChecking] = useState(false);
  const [, setReadVersion] = useState(0);
  const latestRevision = useRef(ownerRevision);
  latestRevision.current = ownerRevision;
  const visibleOwner = useRef(verifiedOwner);
  visibleOwner.current = verifiedOwner;
  // Readiness pauses hide private data without truncating an already-started read.
  const pendingRead = useRef<{ userId: string; tenantId: string; revision: number; cancel: () => void; reconcile: () => void; canDisplay: () => boolean; isActive: () => boolean } | null>(null);
  useEffect(() => () => pendingRead.current?.cancel(), []);
  useEffect(() => {
    const previous = pendingRead.current;
    if (previous && (previous.revision !== ownerRevision || userId && (previous.userId !== userId || previous.tenantId !== tenantId))) { previous.cancel(); pendingRead.current = null; }
    if (!userId || !tenantId) { pendingRead.current?.reconcile(); return; }
    // This effect runs only for an actual owner/readiness or local revision
    // transition. A fresh verified transition may recover with a new read;
    // retiring a scope alone never retries or revives its old response.
    if (pendingRead.current && !pendingRead.current.isActive()) pendingRead.current = null;
    if (pendingRead.current) { pendingRead.current.reconcile(); return; }
    const owner = { userId, tenantId };
    const lease = currentVerifiedQueueLease();
    if (!lease || !sameOwner(lease.owner, owner)) return;
    const generation = queueIdentityGeneration();
    const request = new AbortController();
    let active = true, consumed = false, started = false;
    let receipt: { status: number; body: unknown } | { failed: true } | undefined;
    let expiry: ReturnType<typeof setTimeout> | undefined;
    let unsubscribe = () => {};
    setSavings(null); setError('');
    const cancel = () => {
      if (!active) return;
      // Keep a retired scope until the next verified owner transition. Its
      // original response remains retired even if a later read is permitted.
      active = false; receipt = undefined; consumed = true;
      clearTimeout(expiry); unsubscribe(); request.abort();
      setReadVersion(value => value + 1);
      setSavings(null); setError('Recorded time-savings data is unavailable.');
    };
    const originalLeaseValid = () => {
      try { return ownerRevision === latestRevision.current && generation === queueIdentityGeneration() && lease.expiresAt > Date.now()
        && lease.storageEpoch === localStorage.getItem(QUEUE_IDENTITY_EPOCH_KEY); }
      catch { return false; }
    };
    const reconcile = () => {
      if (!active) return;
      if (!originalLeaseValid()) { cancel(); return; }
      if (!hasVerifiedOfflineQueueOwner(owner)) {
        if (!hasPendingQueueOwnerVerification()) cancel();
        return;
      }
      if (!visibleOwner.current || !sameOwner(visibleOwner.current, owner)) return;
      if (!started) { started = true; void dispatch(); }
      if (!receipt || consumed) return;
      consumed = true;
      try {
        if ('failed' in receipt) throw new Error('Savings unavailable');
        const value = receipt.body;
        if (receipt.status !== 200) throw new Error('Savings unavailable');
        setSavings(readSavings(value));
      } catch { if (active) setError('Recorded time-savings data is unavailable.'); }
    };
    const canDisplay = () => active && originalLeaseValid() && hasVerifiedOfflineQueueOwner(owner)
      && !!visibleOwner.current && sameOwner(visibleOwner.current, owner);
    pendingRead.current = { userId, tenantId, revision: ownerRevision, cancel, reconcile, canDisplay, isActive: () => active };
    setReadVersion(value => value + 1);
    const armExpiry = () => {
      expiry = setTimeout(() => {
        if (!active) return;
        if (!originalLeaseValid()) cancel(); else armExpiry();
      }, Math.min(Math.max(1, lease.expiresAt - Date.now()), 2_147_483_647));
    };
    const dispatch = async () => {
      try {
        const headers = new Headers({ 'x-ohc-expected-user': userId, 'x-ohc-expected-tenant': tenantId });
        const response = await fetch('/api/v1/growth/time-savings', { headers, credentials: 'same-origin', cache: 'no-store', redirect: 'error', signal: request.signal });
        if (!active) return;
        if (!originalLeaseValid()) { cancel(); return; }
        if (response.status === 401 || response.status === 403) { notifyQueueIdentityChange(); return; }
        const body: unknown = await response.json();
        if (!active) return;
        if (!originalLeaseValid()) { cancel(); return; }
        const reason = body && typeof body === 'object' && 'error' in body ? body.error : undefined;
        if (response.status === 409 && (reason === 'session_identity_changed' || reason === 'queued owner does not match the current session')) { notifyQueueIdentityChange(); return; }
        receipt = { status: response.status, body };
      } catch { if (active) receipt = { failed: true }; }
      reconcile();
    };
    armExpiry();
    unsubscribe = subscribeQueueIdentityReadiness(reconcile);
    if (!active) unsubscribe();
  }, [userId, tenantId, ownerRevision]);
  const checkTrial = async () => {
    setChecking(true);
    try { await claimTrial(); }
    catch { setError('Trial activation is unavailable.'); }
    finally { setChecking(false); }
  };
  const canDisplay = !!verifiedOwner && !!pendingRead.current && pendingRead.current.revision === ownerRevision && pendingRead.current.canDisplay();
  return <section className="rounded-xl border bg-white dark:bg-gray-900 p-6 mb-8" aria-label="Recorded time savings">
    <h2 className="text-xl font-bold">Recorded time savings</h2>
    {!canDisplay ? <p role="status">Verify your account to read time-savings data.</p> : error ? <p role="status">{error}</p> : !savings ? <p role="status">Loading recorded time-savings data…</p> : <>
      <p>Recorded estimate: {savings.hours_saved} hours saved.</p>
      <p>Customer inquiries handled: {savings.inquiries_handled ?? 'not reported'}. Appointments scheduled: {savings.appointments_scheduled ?? 'not reported'}.</p>
      <a
        className="app-button"
        href={`https://twitter.com/intent/tweet?text=${encodeURIComponent(`Recorded estimate: ${savings.hours_saved} hours saved using OmniSolo OneHumanCorp.`)}`}
        target="_blank"
        rel="noopener noreferrer"
        onClick={event => { if (!pendingRead.current?.canDisplay()) event.preventDefault(); }}
      >Share recorded savings on X</a>
    </>}
    <p>Sharing does not verify a trial grant or its duration.</p>
    <button type="button" disabled={checking} onClick={() => void checkTrial()} className="app-button">{checking ? 'Checking…' : 'Check trial availability'}</button>
    {claimError && <p role="status">{claimError}</p>}
  </section>;
}
