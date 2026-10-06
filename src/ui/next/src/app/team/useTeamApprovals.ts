'use client';
import { useCallback, useEffect, useRef, useState } from 'react';
import { hasPendingQueueOwnerVerification, hasVerifiedOfflineQueueOwner, QUEUE_IDENTITY_EPOCH_KEY, queueIdentityGeneration, readQueueOwner, sameOwner, subscribeQueueIdentityReadiness, type QueueOwner } from '@/lib/sync/queueIdentity';
import { canonical, object, readApproval, readApprovalPage, readDecision, validId, type Approval, type Decision } from './approvalContract';

type Notice = { kind: 'pending' | 'unknown' | 'recorded' | 'rejected'; message: string };
const unknown: Notice = { kind: 'unknown', message: 'Decision outcome is unconfirmed. Refresh recorded decisions to reconcile; this request will not be resent.' };
const scopeKey = (owner: QueueOwner) => encodeURIComponent(JSON.stringify([owner.userId, owner.tenantId]));
const prefix = (owner: QueueOwner) => 'omnisolo_team_decision_v1:' + scopeKey(owner) + ':';
const chatKey = (owner: QueueOwner) => 'omnisolo_team_chat_v1:' + scopeKey(owner);
function heldChats(owner: QueueOwner): string[] {
  const raw = localStorage.getItem(chatKey(owner)); if (raw === null) return [];
  const data: unknown = JSON.parse(raw);
  if (!Array.isArray(data) || !data.every(value => typeof value === 'string')) throw Error('Request history unavailable');
  return data;
}
function headersFor(owner: QueueOwner, json = false) { return { ...(json ? { 'Content-Type': 'application/json' } : {}), 'x-ohc-expected-user': owner.userId, 'x-ohc-expected-tenant': owner.tenantId }; }
function storedDecision(raw: string): Decision {
  const data = object(JSON.parse(raw));
  if (!data || !['APPROVED','DISMISSED'].includes(String(data.state)) || (data.proposed_action != null && !object(data.proposed_action))) throw Error('Unrecognized pending decision');
  return data as Decision;
}
// A concurrent canonical identity read temporarily suspends the shared lease.
// Keep the finite response consumed, but never apply it until that suspension
// resolves for the same owner. Abort/retirement still wins over a late reply.
function settledVerification(valid: () => boolean, signal: AbortSignal): Promise<void> {
  return new Promise(resolve => {
    let done = false;
    let unsubscribe = () => {};
    const finish = () => {
      if (done) return;
      done = true; unsubscribe(); signal.removeEventListener('abort', finish); resolve();
    };
    const check = () => { if (!valid() || signal.aborted || !hasPendingQueueOwnerVerification()) finish(); };
    signal.addEventListener('abort', finish, { once: true });
    unsubscribe = subscribeQueueIdentityReadiness(check);
    if (done) unsubscribe();
    check();
  });
}
export function useTeamApprovals() {
  const [items, setItems] = useState<Approval[]>([]), [loading, setLoading] = useState(true), [ready, setReady] = useState(false);
  const [error, setError] = useState(''), [notices, setNotices] = useState(new Map<string, Notice>()), [busy, setBusy] = useState(false), [revision, setRevision] = useState(0);
  const [chatNotice, setChatNotice] = useState('');
  const owner = useRef<QueueOwner | null>(null), generation = useRef(0), active = useRef(false), working = useRef<number | null>(null), controllers = useRef(new Set<AbortController>());
  const retire = useCallback(() => {
    generation.current++; owner.current = null; working.current = null;
    controllers.current.forEach(controller => controller.abort()); controllers.current.clear();
    setItems([]); setNotices(new Map()); setReady(false); setLoading(false); setBusy(false); setChatNotice(''); setRevision(value => value + 1);
    setError('Your session changed. Reload to verify access to approvals.');
  }, []);
  const readiness = useCallback(() => {
    const expected = owner.current;
    setReady(!!expected && hasVerifiedOfflineQueueOwner(expected));
    if (expected && !hasPendingQueueOwnerVerification() && !hasVerifiedOfflineQueueOwner(expected)) retire();
  }, [retire]);
  const sameContext = (expected: QueueOwner, epoch: number, identity: number) => active.current && epoch === generation.current && identity === queueIdentityGeneration() && !!owner.current && sameOwner(expected, owner.current);
  const current = (expected: QueueOwner, epoch: number, identity: number) => sameContext(expected, epoch, identity) && hasVerifiedOfflineQueueOwner(expected);
  const waitCurrent = async (expected: QueueOwner, epoch: number, identity: number, signal: AbortSignal) => {
    await settledVerification(() => sameContext(expected, epoch, identity), signal);
    return !signal.aborted && current(expected, epoch, identity);
  };
  const notice = (id: string, next: Notice) => setNotices(previous => new Map(previous).set(id, next));
  const refresh = useCallback(async () => {
    if (!active.current || working.current === generation.current) return;
    const epoch = generation.current, identity = queueIdentityGeneration(); let committed = false;
    working.current = epoch; setBusy(true); setLoading(true); setError('');
    const controller = new AbortController(); controllers.current.add(controller); const timer = setTimeout(() => controller.abort(), 30000);
    try {
      const verified = await readQueueOwner(controller.signal);
      if (!active.current || epoch !== generation.current || identity !== queueIdentityGeneration()) return;
      if (owner.current && !sameOwner(owner.current, verified)) { retire(); return; }
      owner.current = verified;
      const fetched = new Map<string, Approval>(), visited = new Set<string>(); let cursor: string | null = null;
      do {
        const url = '/api/v1/agents/approvals?limit=100' + (cursor ? '&cursor=' + encodeURIComponent(cursor) : '');
        const res = await fetch(url, { headers: headersFor(verified), cache: 'no-store', credentials: 'same-origin', redirect: 'error', signal: controller.signal });
        const data: unknown = await res.json().catch(() => null);
        if (!await waitCurrent(verified, epoch, identity, controller.signal)) return;
        if ([401,403,409].includes(res.status)) { retire(); return; }
        if (res.status !== 200) throw Error('Approval list unavailable.');
        const page = readApprovalPage(data, verified.tenantId);
        if (!current(verified, epoch, identity)) return;
        for (const item of page.items) {
          if (fetched.has(item.id)) throw Error('Conflicting approval pagination.');
          fetched.set(item.id, item);
        }
        cursor = page.next;
        if (cursor) { if (visited.has(cursor)) throw Error('Repeated approval cursor.'); visited.add(cursor); }
      } while (cursor);
      const nextNotices = new Map<string, Notice>();
      for (let index = 0; index < localStorage.length; index++) {
        const key = localStorage.key(index); if (!key?.startsWith(prefix(verified))) continue;
        const id = key.slice(prefix(verified).length); if (!validId(id)) continue;
        nextNotices.set(id, unknown);
        try {
          const decision = storedDecision(localStorage.getItem(key)!);
          const res = await fetch(`/api/v1/agent-feed/${id}/decision`, { headers: headersFor(verified), cache: 'no-store', credentials: 'same-origin', redirect: 'error', signal: controller.signal });
          const data: unknown = await res.json().catch(() => null);
          if (!await waitCurrent(verified, epoch, identity, controller.signal)) return;
          if ([401,403,409].includes(res.status)) { retire(); return; }
          if (res.status !== 200) continue;
          const message = readDecision(data, id, verified.tenantId, decision);
          if (!current(verified, epoch, identity)) return;
          nextNotices.set(id, { kind: 'recorded', message }); fetched.delete(id);
        } catch { /* Failed readback leaves the original exact decision held. */ }
      }
      if (!current(verified, epoch, identity)) return;
      committed = true; setItems([...fetched.values()]); setNotices(nextNotices);
      if (heldChats(verified).length) setChatNotice('A previous request outcome is unconfirmed. Review pending approvals before sending a different request; the original request will not be resent.');
    } catch {
      if (active.current && epoch === generation.current) { setItems([]); setError('Approvals are unavailable. No empty or completed state has been inferred.'); }
    } finally {
      clearTimeout(timer); controllers.current.delete(controller);
      if (active.current && epoch === generation.current && working.current === epoch) {
        if (!committed) setError(previous => previous || 'Approvals are unavailable until this session is verified. Refresh to retry.');
        working.current = null; setBusy(false); setLoading(false); readiness();
      }
    }
  }, [readiness, retire]);
  useEffect(() => {
    active.current = true;
    const unsubscribe = subscribeQueueIdentityReadiness(readiness);
    const storage = (event: StorageEvent) => {
      if (event.key === null || event.key === QUEUE_IDENTITY_EPOCH_KEY) retire();
      else if (owner.current && event.key?.startsWith(prefix(owner.current))) {
        const id = event.key.slice(prefix(owner.current).length); if (validId(id)) notice(id, unknown);
      }
    };
    window.addEventListener('omnisolo_auth_changed', retire); window.addEventListener('pagehide', retire); window.addEventListener('storage', storage); void refresh();
    return () => { active.current = false; const retiredGeneration = generation.current++; if (working.current === retiredGeneration) working.current = null; owner.current = null; controllers.current.forEach(controller => controller.abort()); controllers.current.clear(); unsubscribe(); window.removeEventListener('omnisolo_auth_changed', retire); window.removeEventListener('pagehide', retire); window.removeEventListener('storage', storage); };
  }, [readiness, refresh, retire]);

  const mutate = async (operation: (expected: QueueOwner, guard: () => boolean, signal: AbortSignal, context: () => boolean) => Promise<boolean>): Promise<boolean> => {
    const expected = owner.current;
    if (!expected || !ready || error || working.current === generation.current || !navigator.onLine || !navigator.locks?.request) return false;
    const epoch = generation.current, identity = queueIdentityGeneration();
    working.current = epoch; setBusy(true);
    const controller = new AbortController(); controllers.current.add(controller); const timer = setTimeout(() => controller.abort(), 30000);
    const context = () => sameContext(expected, epoch, identity) && !controller.signal.aborted;
    const guard = () => context() && hasVerifiedOfflineQueueOwner(expected);
    try {
      return await navigator.locks.request('omnisolo-team:' + scopeKey(expected), { mode: 'exclusive', signal: controller.signal }, async () => {
        const verified = await readQueueOwner(controller.signal);
        if (!await waitCurrent(expected, epoch, identity, controller.signal)) return false;
        if (!sameOwner(expected, verified)) { retire(); return false; }
        return operation(expected, guard, controller.signal, context);
      });
    } catch { return false; }
    finally { clearTimeout(timer); controllers.current.delete(controller); if (active.current && epoch === generation.current && working.current === epoch) { working.current = null; setBusy(false); readiness(); } }
  };
  const decide = (id: string, state: Decision['state'], proposed_action?: Approval['payload']) => mutate(async (expected, guard, signal, context) => {
    if (!validId(id) || !items.some(item => item.id === id && item.tenant_id === expected.tenantId) || notices.has(id) && notices.get(id)?.kind !== 'rejected') return false;
    const key = prefix(expected) + id, decision: Decision = { state, ...(proposed_action ? { proposed_action } : {}) };
    if (localStorage.getItem(key) !== null) { notice(id, unknown); return false; }
    let dispatched = false;
    try {
      localStorage.setItem(key, JSON.stringify(decision)); if (!guard()) return false;
      notice(id, { kind: 'pending', message: 'Recording decision; execution or delivery is not confirmed.' }); dispatched = true;
      const res = await fetch(`/api/v1/agent-feed/${id}`, { method: 'PUT', headers: headersFor(expected, true), credentials: 'same-origin', cache: 'no-store', redirect: 'error', signal, body: JSON.stringify(decision) });
      const data: unknown = await res.json().catch(() => null);
      await settledVerification(context, signal);
      if (!guard()) return false;
      if ([401,403,409].includes(res.status)) { retire(); return false; }
      if (res.status !== 200) throw Error('Decision not confirmed');
      const message = readDecision(data, id, expected.tenantId, decision);
      if (!guard()) return false;
      notice(id, { kind: 'recorded', message }); setItems(previous => previous.filter(item => item.id !== id)); return true;
    } catch {
      if (guard()) notice(id, dispatched ? unknown : { kind: 'rejected', message: 'Decision could not be prepared safely. No request was sent.' });
      return false;
    }
  });
  const send = (message: string) => mutate(async (expected, guard, signal, context) => {
    if (!message.trim() || message.length > 16000) return false;
    const key = chatKey(expected), fingerprint = canonical({ message: message.trim() });
    if (heldChats(expected).includes(fingerprint)) { setChatNotice('Request outcome is unconfirmed. Check pending approvals; this request will not be resent.'); return false; }
    let dispatched = false;
    try {
      localStorage.setItem(key, JSON.stringify([...heldChats(expected), fingerprint])); if (!guard()) return false;
      setChatNotice('Saving your request for department review…'); dispatched = true;
      const res = await fetch('/api/v1/agents/chat', { method: 'POST', headers: headersFor(expected, true), credentials: 'same-origin', cache: 'no-store', redirect: 'error', signal, body: JSON.stringify({ message: message.trim() }) });
      const data = object(await res.json().catch(() => null));
      await settledVerification(context, signal);
      if (!guard()) return false;
      if ([401,403,409].includes(res.status)) { retire(); return false; }
      if (!guard()) return false;
      if ([400, 422, 429].includes(res.status)) {
        localStorage.setItem(key, JSON.stringify(heldChats(expected).filter(value => value !== fingerprint)));
        setChatNotice(`Request rejected (HTTP ${res.status}). ${typeof data?.error === 'string' ? data.error : 'No request was accepted.'}`); return false;
      }
      if (res.status !== 200 || data?.success !== true) throw Error('Request not confirmed');
      const item = readApproval(data.approval, expected.tenantId);
      if (item.status !== 'PENDING_APPROVAL') throw Error('Invalid chat acceptance');
      setItems(previous => [item, ...previous.filter(row => row.id !== item.id)]); localStorage.setItem(key, JSON.stringify(heldChats(expected).filter(value => value !== fingerprint)));
      setChatNotice('Request saved for department review. No execution or delivery is confirmed.'); return true;
    } catch {
      if (guard()) setChatNotice(dispatched ? 'Request outcome is unconfirmed. Check pending approvals before trying again; this request will not be resent.' : 'Request could not be saved locally. No request was sent.');
      return false;
    }
  });
  return { items, loading, ready, error, busy, notices, revision, chatNotice, refresh, decide, send,
    blocked: (id: string) => !ready || !!error || busy || (notices.has(id) && notices.get(id)?.kind !== 'rejected') };
}
