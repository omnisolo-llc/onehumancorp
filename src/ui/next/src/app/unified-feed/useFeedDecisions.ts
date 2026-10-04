'use client';
import { useCallback, useEffect, useRef, useState } from 'react';
import { hasVerifiedOfflineQueueOwner, hasPendingQueueOwnerVerification, QUEUE_IDENTITY_EPOCH_KEY, readQueueOwner, sameOwner, subscribeQueueIdentityReadiness, type QueueOwner } from '@/lib/sync/queueIdentity';
import type { ActionPayload } from '@/lib/agent-feed-types';

export type FeedItemRaw = { id: string; tenant_id: string; event_source: string; context_payload?: ActionPayload; proposed_action?: ActionPayload; lifecycle_state: string; created_at?: string | null; updated_at?: string | null };
type Decision = { state: 'APPROVED' | 'DISMISSED'; edited?: string };
type Notice = { kind: 'pending' | 'unknown' | 'rejected' | 'acknowledged'; message: string };
const unknownNotice: Notice = { kind: 'unknown', message: 'Outcome unconfirmed. The card is held to prevent duplicate actions. Refresh recorded decisions to check; no request will be resent.' };
const record = (value: unknown): Record<string, unknown> | null => value && typeof value === 'object' && !Array.isArray(value) ? value as Record<string, unknown> : null;
const validId = (value: unknown): value is string => typeof value === 'string' && /^[A-Za-z0-9._-]{1,128}$/.test(value) && !['.', '..'].includes(value);
const prefixFor = (owner: QueueOwner) => 'omnisolo_feed_decision_v1:' + encodeURIComponent(JSON.stringify([owner.userId, owner.tenantId])) + ':';
function decisionFrom(raw: string): Decision | null {
  try {
    const data = record(JSON.parse(raw));
    return data && ['APPROVED', 'DISMISSED'].includes(String(data.state)) && (data.edited === undefined || typeof data.edited === 'string')
      ? { state: data.state as Decision['state'], ...(data.edited === undefined ? {} : { edited: data.edited as string }) } : null;
  } catch { return null; }
}
function acknowledges(value: unknown, id: string, owner: QueueOwner, decision: Decision): boolean {
  const row = record(value);
  if (!row || row.error != null || ('success' in row && row.success !== true) || row.id !== id || row.tenant_id !== owner.tenantId || row.lifecycle_state !== decision.state) return false;
  if (decision.edited === undefined) return true;
  const payload = record(row.proposed_action);
  const content = payload && ['draft_reply', 'generated_response', 'summary', 'message'].filter(key => typeof payload[key] === 'string').map(key => payload[key]);
  return !!content && content.length > 0 && content.every(text => text === decision.edited);
}
const acknowledgement = (decision: Decision): Notice => ({ kind: 'acknowledged', message: decision.state === 'APPROVED'
  ? 'Approval recorded. Execution or delivery is not verified by this decision.' : 'Dismissal recorded.' });
function rowsFrom(value: unknown, owner: QueueOwner): FeedItemRaw[] {
  const data = record(value);
  if (!data || data.error != null || ('success' in data && data.success !== true) || !Array.isArray(data.items)) throw new Error('Feed unavailable');
  const rows = new Map<string, FeedItemRaw>();
  for (const value of data.items) {
    const row = record(value);
    if (!row || !validId(row.id) || row.tenant_id !== owner.tenantId || typeof row.event_source !== 'string' || typeof row.lifecycle_state !== 'string' || !row.lifecycle_state) throw new Error('Feed identity or record unavailable');
    // The existing union may include a legacy copy of the same item. Conflicting
    // lifecycle records cannot acknowledge a decision or grant another action.
    const existing = rows.get(row.id);
    if (existing && (existing.lifecycle_state !== row.lifecycle_state || JSON.stringify(existing.proposed_action ?? null) !== JSON.stringify(row.proposed_action ?? null))) throw new Error('Conflicting feed records');
    if (!rows.has(row.id)) rows.set(row.id, row as FeedItemRaw);
  }
  return [...rows.values()];
}
function ownerHeaders(owner: QueueOwner): Headers {
  const headers = new Headers({ 'x-ohc-expected-user': owner.userId, 'x-ohc-expected-tenant': owner.tenantId });
  if (headers.get('x-ohc-expected-user') !== owner.userId || headers.get('x-ohc-expected-tenant') !== owner.tenantId) throw new Error('Owner cannot be represented');
  return headers;
}

/** Records decisions only. A feed row is not a receipt for dispatched business work. */
export function useFeedDecisions() {
  const [items, setItems] = useState<FeedItemRaw[]>([]);
  const [loading, setLoading] = useState(true), [refreshing, setRefreshing] = useState(false);
  const [error, setError] = useState<string | null>(null), [ready, setReady] = useState(false);
  const [processingId, setProcessingId] = useState<string | null>(null);
  const [notices, setNotices] = useState(new Map<string, Notice>());
  const [revision, setRevision] = useState(0);
  const owner = useRef<QueueOwner | null>(null), active = useRef(false), epoch = useRef(0);
  const busy = useRef(false), requests = useRef(new Set<AbortController>());
  const notice = (id: string, value: Notice) => setNotices(previous => new Map(previous).set(id, value));
  const retire = useCallback(() => {
    ++epoch.current; owner.current = null; busy.current = false;
    for (const request of requests.current) request.abort();
    requests.current.clear(); setItems([]); setNotices(new Map()); setProcessingId(null); setReady(false); setLoading(false); setRefreshing(false);
    setError('Your session changed or could not be verified. Reload to verify access to the current feed.'); setRevision(value => value + 1);
  }, []);
  const readiness = useCallback(() => {
    const expected = owner.current;
    setReady(!!expected && hasVerifiedOfflineQueueOwner(expected));
    if (expected && !hasPendingQueueOwnerVerification() && !hasVerifiedOfflineQueueOwner(expected)) retire();
  }, [retire]);
  const current = useCallback((expected: QueueOwner, generation: number) => active.current && generation === epoch.current && !!owner.current && sameOwner(owner.current, expected), []);
  const refresh = useCallback(async () => {
    if (!active.current || busy.current) return;
    busy.current = true; setRefreshing(true); setError(null);
    const generation = epoch.current;
    const controller = new AbortController(); requests.current.add(controller);
    const timer = setTimeout(() => controller.abort(), 30_000);
    try {
      const verified = await readQueueOwner(controller.signal);
      if (!active.current || generation !== epoch.current) return;
      if (owner.current && !sameOwner(owner.current, verified)) { retire(); return; }
      owner.current = { ...verified }; readiness();
      const response = await fetch('/api/v1/agent-feed', { headers: ownerHeaders(verified), credentials: 'same-origin', cache: 'no-store', redirect: 'error', signal: controller.signal });
      if ([401, 403, 409].includes(response.status)) {
        await response.text().catch(() => undefined);
        if (current(verified, generation)) retire();
        return;
      }
      const data: unknown = await response.json();
      if (!current(verified, generation)) return;
      if (response.status !== 200) throw new Error('Feed unavailable');
      const rows = rowsFrom(data, verified), prefix = prefixFor(verified);
      const nextNotices = new Map<string, Notice>();
      for (let index = 0; index < localStorage.length; index += 1) {
        const key = localStorage.key(index);
        if (!key?.startsWith(prefix)) continue;
        const id = key.slice(prefix.length), raw = localStorage.getItem(key);
        if (!validId(id) || raw === null) continue;
        const decision = decisionFrom(raw), matches = rows.filter(row => row.id === id);
        nextNotices.set(id, decision && matches.length === 1 && acknowledges(matches[0], id, verified, decision) ? acknowledgement(decision) : unknownNotice);
      }
      if (!current(verified, generation)) return;
      setNotices(nextNotices);
      setItems(previous => {
        // Missing/pending readback is not proof of failure: the list is paginated
        // and may be cached, and the original request could still complete.
        const present = new Set(rows.map(row => row.id));
        return [...rows, ...previous.filter(row => !present.has(row.id) && nextNotices.get(row.id)?.kind === 'unknown')];
      });
    } catch {
      if (active.current && generation === epoch.current) setError('Recorded decisions could not be refreshed. Existing uncertain outcomes remain held.');
    } finally {
      clearTimeout(timer); requests.current.delete(controller);
      if (active.current && generation === epoch.current) { busy.current = false; setLoading(false); setRefreshing(false); readiness(); }
    }
  }, [current, readiness, retire]);
  useEffect(() => {
    active.current = true;
    const unsubscribe = subscribeQueueIdentityReadiness(readiness);
    const storage = (event: StorageEvent) => {
      if (event.key === null || event.key === QUEUE_IDENTITY_EPOCH_KEY) retire();
      else if (owner.current && event.key?.startsWith(prefixFor(owner.current))) {
        const id = event.key.slice(prefixFor(owner.current).length);
        if (validId(id) && event.newValue) notice(id, unknownNotice);
      }
    };
    window.addEventListener('omnisolo_auth_changed', retire); window.addEventListener('pagehide', retire); window.addEventListener('storage', storage);
    void refresh();
    return () => { active.current = false; ++epoch.current; owner.current = null; busy.current = false; for (const request of requests.current) request.abort(); requests.current.clear(); unsubscribe(); window.removeEventListener('omnisolo_auth_changed', retire); window.removeEventListener('pagehide', retire); window.removeEventListener('storage', storage); };
  }, [readiness, refresh, retire]);
  const decide = async (id: string, state: Decision['state'], edited?: string): Promise<boolean> => {
    const expected = owner.current;
    if (!expected || !ready || error !== null || busy.current || !validId(id) || !items.some(row => row.id === id) || ['unknown', 'pending', 'acknowledged'].includes(notices.get(id)?.kind ?? '')) return false;
    if (!navigator.locks?.request) { notice(id, { kind: 'rejected', message: 'Safe decision submission is unavailable in this browser. No request was sent.' }); return false; }
    const generation = epoch.current, decision: Decision = { state, ...(edited === undefined ? {} : { edited }) };
    busy.current = true; setProcessingId(id); notice(id, { kind: 'pending', message: 'Waiting for the recorded decision. No execution or delivery is confirmed.' });
    const controller = new AbortController(); requests.current.add(controller);
    const timer = setTimeout(() => controller.abort(), 30_000);
    let dispatched = false;
    try {
      return await navigator.locks.request(prefixFor(expected) + id, { mode: 'exclusive', signal: controller.signal }, async () => {
        const verified = await readQueueOwner(controller.signal);
        if (!current(expected, generation)) return false;
        if (!sameOwner(verified, expected)) { retire(); return false; }
        if (controller.signal.aborted) throw new Error('Decision preparation interrupted');
        const key = prefixFor(expected) + id;
        if (localStorage.getItem(key) !== null) { notice(id, unknownNotice); return false; }
        const headers = ownerHeaders(expected); headers.set('Content-Type', 'application/json');
        // The marker survives reload/session retirement and stays after success:
        // another stale tab must read back, never repeat the business mutation.
        localStorage.setItem(key, JSON.stringify(decision));
        if (!current(expected, generation)) return false;
        dispatched = true;
        const response = await fetch(`/api/v1/agent-feed/${id}`, { method: 'PUT', headers, credentials: 'same-origin', cache: 'no-store', redirect: 'error', signal: controller.signal,
          body: JSON.stringify({ state, ...(edited === undefined ? {} : { edited_payload: edited }) }) });
        if (!current(expected, generation)) { await response.text().catch(() => undefined); return false; }
        if (response.status >= 400 && response.status < 500) {
          await response.text().catch(() => undefined);
          if (!current(expected, generation)) return false;
          localStorage.removeItem(key);
          if ([401, 403, 409].includes(response.status)) { retire(); return false; }
          notice(id, { kind: 'rejected', message: `Request rejected (HTTP ${response.status}). The card and draft are retained; no decision is confirmed.` }); return false;
        }
        const text = await response.text();
        if (!current(expected, generation)) return false;
        let receipt: unknown = null;
        try { receipt = JSON.parse(text); } catch { /* An unreadable acknowledgement stays held. */ }
        if (response.status !== 200 || !acknowledges(receipt, id, expected, decision)) { notice(id, unknownNotice); return false; }
        notice(id, acknowledgement(decision)); setItems(previous => previous.filter(row => row.id !== id)); return true;
      });
    } catch {
      if (current(expected, generation)) notice(id, dispatched ? unknownNotice : { kind: 'rejected', message: 'The decision could not be prepared safely. No request was sent.' });
      return false;
    } finally {
      clearTimeout(timer); requests.current.delete(controller);
      if (current(expected, generation)) { busy.current = false; setProcessingId(null); readiness(); }
    }
  };
  return { items, loading, refreshing, error, ready, processingId, notices, revision, refresh, decide,
    blocked: (id: string) => !ready || error !== null || refreshing || processingId !== null || ['pending', 'unknown', 'acknowledged'].includes(notices.get(id)?.kind ?? '') };
}
