'use client';
import { useCallback, useEffect, useRef, useState } from 'react';
import { sameOwner } from '@/lib/sync/queueIdentity';
import {
  fetchForOwnedBusinessAction, fetchForOwnedBusinessRead, onboardingOwner,
  onboardingSessionEpoch, openOnboardingSession, readOwnedOnboardingItem,
  subscribeOnboardingInvalidation, writeOwnedOnboardingItem, type DraftOwner,
} from '../onboarding/draftSession';

type Policy = { provider: string; model: string; max_output_tokens: number };
export type AnalysisReceipt = { id: string; agent_id: string; workflow_id: string; status: 'queued' };
export type AnalysisTask = { name: string; role: string; task: string; model: string };
const REQUEST = 'agent-analysis-request';
const UNKNOWN = 'Could not confirm whether this task was accepted. It is on hold to prevent duplicate execution.';
const record = (value: unknown): Record<string, unknown> | null => value && typeof value === 'object' && !Array.isArray(value) ? value as Record<string, unknown> : null;
function policyFrom(value: unknown): Policy | null {
  const data = record(value), policy = record(data?.policy);
  if (data?.mode !== 'text_analysis' || data.workspace_access !== false || !Array.isArray(data.tools) || data.tools.length) throw new Error('Unsupported execution policy');
  if (data.available === false && data.policy === null) return null;
  if (data.available !== true || !policy || typeof policy.provider !== 'string' || !policy.provider || typeof policy.model !== 'string' || !policy.model || policy.model.length > 200 || !Number.isInteger(policy.max_output_tokens) || Number(policy.max_output_tokens) < 1 || Number(policy.max_output_tokens) > 4096) throw new Error('Invalid execution policy');
  return { provider: policy.provider, model: policy.model, max_output_tokens: Number(policy.max_output_tokens) };
}
function receiptFrom(value: unknown): AnalysisReceipt | null {
  const data = record(value);
  if (!data || typeof data.id !== 'string' || !/^agent-[a-f0-9]{32}$/.test(data.id) || data.id !== data.agent_id || typeof data.workflow_id !== 'string' || !/^[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}$/i.test(data.workflow_id) || data.status !== 'queued') return null;
  return { id: data.id, agent_id: data.id, workflow_id: data.workflow_id, status: 'queued' };
}
function untilAborted<T>(operation: Promise<T>, signal: AbortSignal): Promise<T> {
  return new Promise((resolve, reject) => {
    const abort = () => reject(new DOMException('Task request interrupted', 'AbortError'));
    if (signal.aborted) abort(); else signal.addEventListener('abort', abort, { once: true });
    operation.then(value => { signal.removeEventListener('abort', abort); resolve(value); }, error => { signal.removeEventListener('abort', abort); reject(error); });
  });
}

/** The server admits work. This hook only fences the existing owner session and its local request receipt. */
export function useTenantAnalysis(retireView: () => void) {
  const [policy, setPolicy] = useState<Policy | null>(null);
  const [ready, setReady] = useState(false);
  const [verifying, setVerifying] = useState(true);
  const [busy, setBusy] = useState(false);
  const [held, setHeld] = useState(false);
  const [receipt, setReceipt] = useState<AnalysisReceipt | null>(null);
  const [notice, setNotice] = useState('Checking your session and execution policy…');
  const [revision, setRevision] = useState(0);
  const owner = useRef<DraftOwner | null>(null), mounted = useRef(false), generation = useRef(0);
  const inFlight = useRef(false), blocked = useRef(false), request = useRef<AbortController | null>(null);
  const retire = useRef(retireView); retire.current = retireView;
  const current = useCallback((expected: DraftOwner, token: number, epoch: number) => {
    const active = onboardingOwner();
    return mounted.current && generation.current === token && onboardingSessionEpoch() === epoch && !!owner.current && !!active && sameOwner(owner.current, expected) && sameOwner(active, expected);
  }, []);
  useEffect(() => {
    mounted.current = true;
    const clear = () => {
      generation.current += 1; owner.current = null; request.current?.abort(); request.current = null;
      inFlight.current = false; blocked.current = false;
      setReady(false); setVerifying(false); setPolicy(null); setBusy(false); setHeld(false); setReceipt(null); retire.current();
    };
    const verify = async () => {
      const token = generation.current, epoch = onboardingSessionEpoch();
      setVerifying(true);
      try {
        const expected = await openOnboardingSession();
        if (!mounted.current || token !== generation.current || epoch !== onboardingSessionEpoch()) return;
        owner.current = expected;
        const response = await fetchForOwnedBusinessRead('/api/v1/agents/execution-policy', expected);
        const data: unknown = await response.json();
        if (!current(expected, token, epoch)) return;
        if (!response.ok) throw new Error('Execution policy unavailable');
        const configured = policyFrom(data);
        const pending = readOwnedOnboardingItem(REQUEST);
        setPolicy(configured); setReady(true);
        if (pending) {
          blocked.current = true; setHeld(true); setNotice(UNKNOWN);
          let stored: Record<string, unknown> | null = null;
          try { stored = record(JSON.parse(pending)); } catch { /* An unreadable marker stays held. */ }
          const acknowledged = stored?.status === 'acknowledged' ? receiptFrom(stored.receipt) : null;
          const requestId = stored?.status === 'unknown' && typeof stored.request_id === 'string' && /^[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}$/.test(stored.request_id) ? stored.request_id : null;
          if (acknowledged || requestId) {
            // Browser storage supplies only a reference. Current authenticated
            // server data must confirm it before this view acknowledges it.
            try {
              const lookup = await fetchForOwnedBusinessRead(acknowledged ? `/api/v1/agents/workflows/${acknowledged.workflow_id}` : `/api/v1/agents/workflows/by-request/${requestId}`, expected);
              const snapshot = record(await lookup.json());
              if (!current(expected, token, epoch)) return;
              const row = record(snapshot?.workflow);
              const matching = lookup.status === 200 && snapshot?.error == null && (snapshot?.success === undefined || snapshot.success === true) && row?.tenant_id === expected.tenantId && row?.actor_id === expected.userId && (acknowledged ? row.id === acknowledged.workflow_id : row.request_id === requestId) ? [row] : [];
              if (matching.length === 1 && readOwnedOnboardingItem(REQUEST) === pending && typeof matching[0]?.status === 'string' && ['queued', 'running', 'completed', 'failed', 'cancelled', 'outcome_unknown'].includes(matching[0].status)) {
                const confirmed = acknowledged ?? receiptFrom({ id: matching[0].agent_id, agent_id: matching[0].agent_id, workflow_id: matching[0].id, status: 'queued' });
                if (confirmed) {
                  if (!acknowledged) writeOwnedOnboardingItem(REQUEST, JSON.stringify({ status: 'acknowledged', receipt: confirmed }));
                  setReceipt(confirmed); setNotice(`Previously accepted text analysis: ${confirmed.workflow_id}. Current status: ${matching[0].status}.`);
                }
              }
            } catch { /* Failed readback never clears an unresolved request. */ }
          }
        }
        else setNotice(configured ? `Text analysis uses ${configured.provider} / ${configured.model}. No tools or workspace access.` : 'Text analysis is not configured. No task can be submitted.');
        setRevision(value => value + 1);
      } catch {
        if (mounted.current && token === generation.current && epoch === onboardingSessionEpoch()) setNotice('Could not verify your session or execution policy. No task can be submitted.');
      } finally {
        if (mounted.current && token === generation.current && epoch === onboardingSessionEpoch()) setVerifying(false);
      }
    };
    void verify();
    const unsubscribe = subscribeOnboardingInvalidation(restart => { clear(); setNotice('Your session changed. Verify your session before submitting work.'); if (restart) void verify(); });
    return () => { mounted.current = false; generation.current += 1; owner.current = null; request.current?.abort(); unsubscribe(); };
  }, [current]);

  const readSnapshot = useCallback(async (url: string): Promise<unknown> => {
    const expected = owner.current, token = generation.current, epoch = onboardingSessionEpoch();
    if (!expected || !current(expected, token, epoch)) throw new Error('Session unavailable');
    const response = await fetchForOwnedBusinessRead(url, expected);
    const data: unknown = await response.json();
    if (!current(expected, token, epoch) || !response.ok) throw new Error('Agent data unavailable');
    return data;
  }, [current]);

  const start = async (task: AnalysisTask): Promise<AnalysisReceipt | null> => {
    const expected = owner.current;
    if (!expected || !ready || !policy || inFlight.current || blocked.current) return null;
    if (!task.task.trim() || [...task.task].length > 16000 || !task.name.trim() || [...task.name].length > 200 || !task.role.trim() || [...task.role].length > 120 || (task.model !== 'Auto' && task.model !== policy.model)) {
      setNotice('Enter a supported text task and use the configured model.'); return null;
    }
    if (!navigator.locks?.request) { setNotice('Task submission is unavailable in this browser. No request was sent.'); return null; }
    const token = generation.current, epoch = onboardingSessionEpoch();
    const payload = { ...task, model: policy.model, providerType: 'builtin' };
    const controller = new AbortController(); request.current = controller;
    const timeout = window.setTimeout(() => controller.abort(), 30_000);
    inFlight.current = true; setBusy(true); setNotice('Submitting text analysis…');
    let dispatched = false;
    try {
      return await navigator.locks.request('omnisolo-agent-analysis:' + JSON.stringify([expected.userId, expected.tenantId]), { mode: 'exclusive', signal: controller.signal }, () => untilAborted((async () => {
        if (controller.signal.aborted || !current(expected, token, epoch)) return null;
        if (readOwnedOnboardingItem(REQUEST)) { blocked.current = true; setHeld(true); setNotice(UNKNOWN); return null; }
        const requestId = crypto.randomUUID();
        const response = await fetchForOwnedBusinessAction('/api/v1/agents/hire', { method: 'POST', headers: { 'Content-Type': 'application/json', 'Idempotency-Key': requestId }, body: JSON.stringify(payload), signal: controller.signal }, expected, () => {
          if (controller.signal.aborted || !current(expected, token, epoch)) throw new Error('Task view changed before dispatch');
          writeOwnedOnboardingItem(REQUEST, JSON.stringify({ status: 'unknown', request_id: requestId }));
          dispatched = true; blocked.current = true;
        });
        const data: unknown = await response.json();
        if (controller.signal.aborted || !current(expected, token, epoch)) return null;
        const accepted = receiptFrom(data), rejected = record(data);
        if (response.status === 201 && accepted) {
          try { writeOwnedOnboardingItem(REQUEST, JSON.stringify({ status: 'acknowledged', receipt: accepted })); } catch { /* The durable unknown hold remains; do not retry. */ }
          setReceipt(accepted); setHeld(true); setNotice(`Text analysis queued: ${accepted.workflow_id}. This receipt does not mean it has completed.`);
          return accepted;
        }
        if (response.status === 409 && rejected?.status === 'budget_rejected' && rejected.id === '' && rejected.agent_id === '' && rejected.workflow_id === '') {
          writeOwnedOnboardingItem(REQUEST, ''); blocked.current = false; setHeld(false); setNotice('Your usage budget cannot cover this request. No provider request was sent. Update the spending limit before trying again.');
        } else if (response.status === 400 && rejected?.status === 'error' && rejected.id === '' && rejected.agent_id === '' && rejected.workflow_id === '') {
          writeOwnedOnboardingItem(REQUEST, ''); blocked.current = false; setHeld(false); setNotice('The task was rejected before acceptance. Check its text and supported options.');
        } else { setHeld(true); setNotice(UNKNOWN); }
        return null;
      })(), controller.signal));
    } catch {
      if (current(expected, token, epoch)) {
        if (dispatched) { setHeld(true); setNotice(UNKNOWN); }
        else setNotice('The request could not be prepared safely. No task was sent.');
      }
      return null;
    } finally {
      window.clearTimeout(timeout);
      if (request.current === controller) request.current = null;
      if (current(expected, token, epoch)) { inFlight.current = false; setBusy(false); }
    }
  };

  const cancelReceipt = useCallback(async (id: string): Promise<unknown> => {
    const expected = owner.current, token = generation.current, epoch = onboardingSessionEpoch();
    if (!expected || !current(expected, token, epoch) || inFlight.current) throw new Error('Session or task unavailable');
    if (!/^[a-f0-9]{8}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{12}$/.test(id) || id === '00000000-0000-0000-0000-000000000000') throw new Error('Invalid receipt');
    const controller = new AbortController(); request.current = controller;
    const timeout = window.setTimeout(() => controller.abort(), 30_000);
    inFlight.current = true; setBusy(true);
    try {
      const response = await untilAborted(fetchForOwnedBusinessAction(`/api/v1/agents/workflows/${id}/cancel`, { method: 'POST', signal: controller.signal }, expected, () => {
        if (controller.signal.aborted || !current(expected, token, epoch)) throw new Error('Task view changed before cancellation');
      }), controller.signal);
      const data: unknown = await untilAborted(response.json(), controller.signal);
      if (!current(expected, token, epoch) || controller.signal.aborted || !response.ok) throw new Error('Cancellation unconfirmed');
      return data;
    } finally {
      window.clearTimeout(timeout);
      if (request.current === controller) request.current = null;
      if (current(expected, token, epoch)) { inFlight.current = false; setBusy(false); }
    }
  }, [current]);

  const startAnother = async () => {
    const expected = owner.current, accepted = receipt;
    if (!expected || !accepted || inFlight.current || !navigator.locks?.request) return;
    const token = generation.current, epoch = onboardingSessionEpoch();
    inFlight.current = true; setBusy(true);
    try {
      await navigator.locks.request('omnisolo-agent-analysis:' + JSON.stringify([expected.userId, expected.tenantId]), { mode: 'exclusive' }, async () => {
        if (!current(expected, token, epoch)) return;
        if (readOwnedOnboardingItem(REQUEST) !== JSON.stringify({ status: 'acknowledged', receipt: accepted })) { setNotice(UNKNOWN); return; }
        writeOwnedOnboardingItem(REQUEST, ''); blocked.current = false; setReceipt(null); setHeld(false); setNotice('Ready for a new explicit text task.');
      });
    } catch { if (current(expected, token, epoch)) setNotice('The acknowledged request could not be retired locally. No new task was sent.'); }
    finally { if (current(expected, token, epoch)) { inFlight.current = false; setBusy(false); } }
  };
  return { policy, ready, verifying, busy, held, receipt, notice, revision, readSnapshot, start, startAnother, cancelReceipt };
}
