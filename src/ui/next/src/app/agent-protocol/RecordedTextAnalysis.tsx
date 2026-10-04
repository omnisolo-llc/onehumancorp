'use client';

import { useCallback, useEffect, useRef, useState } from 'react';
import { useTenantAnalysis } from '../agents/useTenantAnalysis';
import { onboardingOwner } from '../onboarding/draftSession';
import { useAuthenticatedPolling } from '@/hooks/useAuthenticatedPolling';

type Execution = ReturnType<typeof useTenantAnalysis>;
type Receipt = { id: string; tenant_id: string; actor_id: string; name: string; task: string; status: string; output: string | null; error: string | null };
const uuid = '[a-f0-9]{8}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{12}';
const active = (status: string) => status === 'queued' || status === 'running';
function receiptFrom(value: unknown, scope: 'actor' | 'tenant' = 'actor'): Receipt {
  const owner = onboardingOwner();
  if (!owner || !value || typeof value !== 'object' || Array.isArray(value)) throw new Error('Unverified receipt');
  const row = value as Record<string, unknown>;
  if (typeof row.id !== 'string' || !new RegExp(`^${uuid}$`).test(row.id) || row.id === '00000000-0000-0000-0000-000000000000'
    || typeof row.actor_id !== 'string' || !row.actor_id.trim() || row.tenant_id !== owner.tenantId
    || scope === 'actor' && row.actor_id !== owner.userId
    || !['name', 'task', 'workflow', 'provider', 'model'].every(key => typeof row[key] === 'string' && String(row[key]).trim())
    || typeof row.status !== 'string' || !['queued', 'running', 'completed', 'failed', 'cancelled', 'outcome_unknown'].includes(row.status)
    || !['output', 'error'].every(key => row[key] === null || typeof row[key] === 'string')
    || row.status === 'completed' && (typeof row.output !== 'string' || !row.output.trim() || row.error !== null)) throw new Error('Unverified receipt');
  return row as Receipt;
}

/** Reads durable text-only receipts; no protocol, model or workspace call occurs on a read. */
export function RecordedTextAnalysis({ execution }: { execution: Execution }) {
  const { ready, revision, readSnapshot, cancelReceipt } = execution;
  const [name, setName] = useState('');
  const [task, setTask] = useState('');
  const [history, setHistory] = useState<Receipt[]>([]);
  const [cursor, setCursor] = useState<string | null>(null);
  const [nextCursor, setNextCursor] = useState<string | null>(null);
  const [historyState, setHistoryState] = useState('unverified');
  const [selectedId, setSelectedId] = useState('');
  const [selected, setSelected] = useState<Receipt | null>(null);
  const [detailError, setDetailError] = useState('');
  const [cancelNotice, setCancelNotice] = useState('');
  const [cancelHeld, setCancelHeld] = useState(false);
  const [reading, setReading] = useState(false);
  const listSequence = useRef(0), detailSequence = useRef(0), live = useRef(true), cancelling = useRef(false);
  useEffect(() => { live.current = true; return () => { live.current = false; listSequence.current++; detailSequence.current++; }; }, []);
  const loadHistory = useCallback(async () => {
    if (!ready) return;
    const sequence = ++listSequence.current; setHistoryState('loading');
    try {
      const value = await readSnapshot(cursor ? `/api/v1/agents/workflows?before=${encodeURIComponent(cursor)}` : '/api/v1/agents/workflows') as { workflows?: unknown; next_cursor?: unknown; error?: unknown; success?: unknown };
      if (!live.current || sequence !== listSequence.current) return;
      if (value?.error != null || value?.success !== undefined && value.success !== true || !Array.isArray(value?.workflows) || value.workflows.length > 20) throw new Error('Invalid history');
      const records = value.workflows.map(value => receiptFrom(value, 'tenant')), next = value.next_cursor;
      if (new Set(records.map(row => row.id)).size !== records.length || next != null && (typeof next !== 'string' || !new RegExp(`^[0-9]{1,12}:${uuid}$`).test(next) || next === cursor || records.length === 0)) throw new Error('Invalid history');
      // The existing endpoint is tenant-wide. Preserve its cursor even when
      // the whole page belongs to teammates; only this panel is personal.
      setHistory(records.filter(row => row.actor_id === onboardingOwner()?.userId)); setNextCursor(typeof next === 'string' ? next : null); setHistoryState('ready');
    } catch { if (live.current && sequence === listSequence.current) { setHistory([]); setNextCursor(null); setHistoryState('unavailable'); } }
  }, [ready, readSnapshot, cursor]);
  useEffect(() => { void loadHistory(); return () => { listSequence.current++; }; }, [loadHistory, revision]);
  const loadDetail = useCallback(async (id: string) => {
    const sequence = ++detailSequence.current; setReading(true); setDetailError('');
    try {
      const value = await readSnapshot(`/api/v1/agents/workflows/${id}`) as { workflow?: unknown; error?: unknown; success?: unknown };
      if (!live.current || sequence !== detailSequence.current) return;
      if (value?.error != null || value?.success !== undefined && value.success !== true) throw new Error('Contradictory receipt');
      const row = receiptFrom(value?.workflow);
      if (row.id !== id) throw new Error('Mismatched receipt');
      setSelected(row); setCancelHeld(false); setCancelNotice('');
    } catch { if (live.current && sequence === detailSequence.current) { setSelected(null); setCancelNotice(''); setDetailError('The recorded analysis receipt could not be verified. Refresh its status before taking action.'); } }
    finally { if (live.current && sequence === detailSequence.current) setReading(false); }
  }, [readSnapshot]);
  useEffect(() => {
    if (execution.receipt) setSelectedId(execution.receipt.workflow_id);
  }, [execution.receipt]);
  useEffect(() => {
    setSelected(null); setCancelNotice(''); setCancelHeld(false);
    if (selectedId && ready) void loadDetail(selectedId);
    return () => { detailSequence.current++; };
  }, [selectedId, ready, loadDetail]);
  useAuthenticatedPolling({ enabled: ready && !!selected && active(selected.status) && !reading && !execution.busy && !cancelHeld, onPoll: () => loadDetail(selectedId) });
  const canStart = ready && !!execution.policy && !execution.busy && !execution.held && !!name.trim() && [...name].length <= 200 && !!task.trim() && [...task].length <= 16000;
  async function submit() {
    if (!canStart) return;
    const receipt = await execution.start({ name, role: 'Text analyst', task, model: 'Auto' });
    if (!live.current || !receipt) return;
    setSelectedId(receipt.workflow_id); void loadHistory();
  }
  async function cancel() {
    if (!selected || !active(selected.status) || cancelHeld || cancelling.current || execution.busy) return;
    const id = selected.id, sequence = ++detailSequence.current;
    cancelling.current = true; setCancelHeld(true); setCancelNotice('Requesting cancellation…');
    try {
      // Never infer a terminal state from the mutation ACK. Read the durable receipt.
      await cancelReceipt(id);
      if (!live.current || sequence !== detailSequence.current) return;
      await loadDetail(id);
    } catch {
      if (live.current && sequence === detailSequence.current) setCancelNotice('Cancellation could not be confirmed. Refresh the recorded status before trying again.');
    } finally { cancelling.current = false; }
  }
  const historyPending = historyState === 'loading' || ready && historyState === 'unverified';
  return <section aria-label="Recorded text analysis" aria-busy={execution.verifying || historyPending} className="mb-10 rounded-2xl border border-sky-200 p-6">
    <h2 className="text-xl font-bold">Text analysis only</h2>
    <p>Analyze text you supply with the configured provider and your authorized usage budget. This does not run workspace tools, Git, actors, expert teams or swarm execution.</p>
    <p role="status" className="my-3">{execution.notice}</p>
    <div className="grid gap-3">
      <label>Analysis name<input className="block w-full border rounded p-2" value={name} onChange={event => setName(event.target.value)} maxLength={200} /></label>
      <label>Analysis task<textarea className="block w-full border rounded p-2" value={task} onChange={event => setTask(event.target.value)} maxLength={16000} /></label>
      <button type="button" className="rounded bg-blue-700 text-white p-2 disabled:opacity-50" disabled={!canStart} onClick={() => void submit()}>Start text analysis</button>
      {execution.receipt && <button type="button" disabled={execution.busy} onClick={() => void execution.startAnother()}>Prepare another text analysis</button>}
    </div>
    <h3 className="font-semibold mt-6">Your recorded analysis history</h3>
    <button type="button" disabled={!ready || historyState === 'loading'} onClick={() => void loadHistory()}>Refresh analysis history</button>
    {historyState === 'ready' ? <><ul>{history.map(row => <li key={row.id}><button type="button" onClick={() => setSelectedId(row.id)} aria-label={`Open analysis ${row.name}`}>{row.name}</button></li>)}</ul>{history.length === 0 && <p>No recorded text analyses on this page.</p>}</> : <p role="status">{historyState === 'unavailable' || !ready && !execution.verifying ? 'Recorded analysis history could not be verified.' : 'Checking recorded analysis history…'}</p>}
    {cursor && <button type="button" onClick={() => setCursor(null)}>Latest analyses</button>}
    {nextCursor && <button type="button" onClick={() => setCursor(nextCursor)}>Older analyses</button>}
    {selectedId && <div className="mt-4 rounded border p-4">
      <p>Receipt: {selectedId}</p>
      <button type="button" disabled={!ready || reading || execution.busy} onClick={() => void loadDetail(selectedId)}>Refresh analysis status</button>
      {detailError && <p role="alert">{detailError}</p>}
      {selected && <><p aria-label="Recorded analysis status">Last verified status: {selected.status}</p><p className="whitespace-pre-wrap">{selected.task}</p>
        {selected.status === 'completed' && <pre className="whitespace-pre-wrap" aria-label="Recorded analysis output">{selected.output}</pre>}
        {selected.error && <p role="alert">{selected.error}</p>}
        {active(selected.status) && <button type="button" disabled={cancelHeld || execution.busy || reading} onClick={() => void cancel()}>Cancel text analysis</button>}
      </>}
      {cancelNotice && <p role="status">{cancelNotice}</p>}
    </div>}
  </section>;
}
