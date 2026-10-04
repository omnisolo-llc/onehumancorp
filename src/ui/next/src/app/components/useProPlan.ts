"use client";

import { useCallback, useEffect, useRef, useState } from 'react';
import { hasVerifiedOfflineQueueOwner, QUEUE_IDENTITY_EPOCH_KEY, readQueueOwner, sameOwner, subscribeQueueIdentityReadiness, type QueueOwner } from '@/lib/sync/queueIdentity';

type Plan = 'Free' | 'Starter' | 'Pro' | 'Business';
function readPlan(value: unknown): Plan {
  if (!value || typeof value !== 'object' || Array.isArray(value)) throw new Error('Plan unavailable');
  const data = value as Record<string, unknown>;
  if (data.error != null || ('success' in data && data.success !== true) || typeof data.current_plan !== 'string') throw new Error('Plan unavailable');
  const plan = (['Free', 'Starter', 'Pro', 'Business'] as const).find(name => name.toLowerCase() === (data.current_plan as string).toLowerCase());
  if (!plan) throw new Error('Plan unavailable');
  return plan;
}

/** Displays the signed account's current plan; it cannot grant or extend a plan. */
export function useProPlan() {
  const [plan, setPlan] = useState<Plan | null>(null);
  const [identityReady, setIdentityReady] = useState(false);
  const [planError, setPlanError] = useState<string | null>(null);
  const [claimError, setClaimError] = useState<string | null>(null);
  const owner = useRef<QueueOwner | null>(null);
  const epoch = useRef(0);
  const active = useRef(false);
  const request = useRef<AbortController | null>(null);
  const retire = useCallback(() => {
    ++epoch.current; request.current?.abort(); owner.current = null;
    setPlan(null); setIdentityReady(false); setClaimError(null);
    setPlanError('Your session changed. Refresh to verify the current plan.');
  }, []);
  const readiness = useCallback(() => {
    const intended = owner.current;
    const ready = hasVerifiedOfflineQueueOwner(intended);
    setIdentityReady(ready);
    if (intended && hasVerifiedOfflineQueueOwner() && !ready) retire();
  }, [retire]);
  const refreshPlan = useCallback(async (): Promise<boolean> => {
    if (!active.current) return false;
    const version = ++epoch.current;
    request.current?.abort(); const controller = new AbortController(); request.current = controller;
    setPlan(null); setPlanError(null); setIdentityReady(false);
    const current = () => active.current && version === epoch.current;
    try {
      const verified = await readQueueOwner();
      if (!current()) return false;
      if (owner.current && !sameOwner(owner.current, verified)) { retire(); return false; }
      owner.current = { ...verified }; readiness();
      if (!current()) return false;
      const headers = new Headers({ 'x-ohc-expected-user': verified.userId, 'x-ohc-expected-tenant': verified.tenantId });
      if (headers.get('x-ohc-expected-user') !== verified.userId || headers.get('x-ohc-expected-tenant') !== verified.tenantId) throw new Error('Plan owner unavailable');
      const response = await fetch('/api/v1/billing/my-plan', { headers, credentials: 'same-origin', cache: 'no-store', redirect: 'error', signal: controller.signal });
      if (!current()) return false;
      if (response.status === 401 || response.status === 403) { retire(); return false; }
      const data: unknown = await response.json();
      if (!current()) return false;
      if (response.status === 409 && data && typeof data === 'object' && 'error' in data && ['session_identity_changed', 'queued owner does not match the current session'].includes(typeof data.error === 'string' ? data.error : '')) { retire(); return false; }
      if (response.status !== 200) throw new Error('Plan unavailable');
      const confirmed = readPlan(data); readiness();
      if (!current()) return false;
      setPlan(confirmed);
      return hasVerifiedOfflineQueueOwner(verified) && (confirmed === 'Pro' || confirmed === 'Business');
    } catch {
      if (current()) { setPlan(null); setPlanError('Current plan data is unavailable. Refresh to check again.'); }
      return false;
    }
  }, [readiness, retire]);
  useEffect(() => {
    active.current = true;
    const unsubscribe = subscribeQueueIdentityReadiness(readiness);
    const storage = (event: StorageEvent) => { if (event.key === null || event.key === QUEUE_IDENTITY_EPOCH_KEY) retire(); };
    window.addEventListener('omnisolo_auth_changed', retire); window.addEventListener('pagehide', retire); window.addEventListener('storage', storage);
    void refreshPlan();
    return () => { active.current = false; ++epoch.current; request.current?.abort(); owner.current = null; unsubscribe(); window.removeEventListener('omnisolo_auth_changed', retire); window.removeEventListener('pagehide', retire); window.removeEventListener('storage', storage); };
  }, [readiness, refreshPlan, retire]);

  // The existing endpoint has no durable grant/expiry receipt. Keep this legacy
  // caller entry point fail-closed until that separate server contract exists.
  const claimTrial = useCallback(async () => {
    if (active.current) setClaimError('Trial activation is unavailable because a durable grant is not verified. Check your current plan or review billing.');
    return false;
  }, []);
  // Expose local retirement/refresh changes without treating pending verification as retirement.
  return { ownerRevision: epoch.current, hasPro: identityReady && (plan === 'Pro' || plan === 'Business'), currentPlan: identityReady ? plan : null, verifiedOwner: identityReady && owner.current ? { ...owner.current } : null, planError, claimError, claimTrial, refreshPlan };
}
