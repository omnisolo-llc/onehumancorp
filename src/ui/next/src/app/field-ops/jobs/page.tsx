"use client";

import React, { useCallback, useEffect, useRef, useState } from 'react';
import { useQuery } from '@powersync/react';
import { SyncManager } from '../../../lib/sync/SyncManager';
import { PowerSyncProvider } from '../../../lib/powersync/PowerSyncProvider';
import { QUEUE_IDENTITY_EPOCH_KEY, sameOwner, type QueueOwner } from '@/lib/sync/queueIdentity';
import { Appointment, AppointmentWrite, fetchFieldJob, observedTime, readAppointments, readAppointmentUpdate, readRouteProposal, readFieldOwner, terminalJob } from './fieldJobClient';

const quoteRequestKey = (owner: QueueOwner, id: string) => 'ohc_field_quote_request_v1:' + JSON.stringify([owner.userId, owner.tenantId, id]);
const quoteIdPattern = /^[a-f0-9]{8}-[a-f0-9]{4}-[1-5][a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}$/i;
const appointmentURL = '/api/v1/field-ops/appointments';
const routeURL = '/api/v1/field-ops/optimize-route';
const jsonPost = (body: unknown, key?: string): RequestInit => ({ method: 'POST', headers: { 'Content-Type': 'application/json', ...(key ? { 'Idempotency-Key': key } : {}) }, body: JSON.stringify(body) });
const message = (error: unknown) => error instanceof Error ? error.message : 'The change could not be confirmed. Your inputs have been kept.';
const time = (value: string | null) => value ? new Date(value).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' }) : 'Not scheduled';
type ProposalEntry = { original: Appointment; proposed: Appointment; key: string; error?: string };
type Proposal = { kind: 'delay' | 'route'; entries: ProposalEntry[]; observed: Appointment[]; key: string };

async function queueCompletionFollowups(jobId: string, customerId: string, notes: string, key: string, timestamp: number, owner: QueueOwner, valid: () => boolean) {
  const quoteNotes = `Follow up quote requested by field op for job ${jobId}. Notes: ${notes}`;
  const intents = [
    { id: `field-completion-${key}-invoice`, type: 'generate_invoice', payload: { job_id: jobId, customer_id: customerId }, timestamp },
    ...(notes ? [{ id: `field-completion-${key}-quote`, type: 'draft_quote', notes: quoteNotes, payload: { notes: quoteNotes }, timestamp }] : []),
  ];
  let queued = 0, unconfirmed = 0;
  for (const intent of intents) {
    if (!valid()) break;
    try { await SyncManager.getInstance().enqueue(intent, owner); queued += 1; }
    catch { unconfirmed += 1; }
  }
  return { queued, unconfirmed };
}

function CachedJobs({ owner, onRows }: { owner: QueueOwner; onRows: (rows: Appointment[]) => void }) {
  const { data } = useQuery<Appointment>('SELECT * FROM appointments WHERE tenant_id = ? ORDER BY scheduled_start_time ASC', [owner.tenantId]);
  useEffect(() => { if (data) onRows(data); }, [data, onRows]);
  return null;
}

function FieldOpsJobsPageContent({ localDatabase = false }: { localDatabase?: boolean }) {
  const [isOffline, setIsOffline] = useState(false);
  const [jobs, setJobs] = useState<Appointment[]>([]);
  const jobsRef = useRef(jobs); jobsRef.current = jobs;
  const [owner, setOwner] = useState<QueueOwner | null>(null);
  const ownerRef = useRef<QueueOwner | null>(null);
  const epoch = useRef(0);
  const mounted = useRef(true);
  const notesDrafts = useRef(new Map<string, string>());
  const expectedNotes = useRef(new Map<string, string | null>());
  const pendingJobs = useRef(new Set<string>());
  const pendingStatuses = useRef(new Map<string, string>());
  const [pending, setPending] = useState<string[]>([]);
  const [offlinePending, setOfflinePending] = useState<string[]>([]);
  const offlinePendingRef = useRef(new Map<string, { status: string; notes: string; updated_at: string }>());
  const statusAttempts = useRef(new Map<string, { body: string; key: string }>());
  const [loading, setLoading] = useState(true);
  const [loadError, setLoadError] = useState<string | null>(null);
  const [notice, setNotice] = useState<string | null>(null);
  const proposalSequence = useRef(0);
  const [calculating, setCalculating] = useState(false);
  const [proposal, setProposal] = useState<Proposal | null>(null);
  const savingProposalRef = useRef(false);
  const [savingProposal, setSavingProposal] = useState(false);
  const quoteSequence = useRef(0);
  const quoteBusy = useRef(false);
  const [voiceQuoteJobId, setVoiceQuoteJobId] = useState<string | null>(null);
  const [voiceTranscript, setVoiceTranscript] = useState('');
  const [draftingQuote, setDraftingQuote] = useState(false);
  const [quoteHeld, setQuoteHeld] = useState(false);
  const [quoteError, setQuoteError] = useState<string | null>(null);
  const [draftQuoteResult, setDraftQuoteResult] = useState<{ id: string } | null>(null);

  const invalidate = useCallback(() => {
    epoch.current += 1; proposalSequence.current += 1; quoteSequence.current += 1;
    ownerRef.current = null; notesDrafts.current.clear(); expectedNotes.current.clear(); statusAttempts.current.clear();
    pendingJobs.current.clear(); pendingStatuses.current.clear(); offlinePendingRef.current.clear(); savingProposalRef.current = false; quoteBusy.current = false;
    setOwner(null); setJobs([]); setPending([]); setOfflinePending([]); setProposal(null); setCalculating(false); setSavingProposal(false);
    setVoiceQuoteJobId(null); setVoiceTranscript(''); setDraftQuoteResult(null); setDraftingQuote(false); setQuoteError(null); setQuoteHeld(false); setNotice(null); setLoading(false);
    setLoadError('Your session changed. Reload the schedule to verify your business.');
  }, []);
  const current = (generation: number, expected: QueueOwner) => mounted.current && epoch.current === generation && !!ownerRef.current && sameOwner(ownerRef.current, expected);
  const reportError = (error: unknown) => {
    if (/session/i.test(message(error))) invalidate();
    else setLoadError(message(error));
  };
  const cancelProposal = () => { proposalSequence.current += 1; setProposal(null); setCalculating(false); };

  const loadSchedule = useCallback(async () => {
    const generation = ++epoch.current;
    proposalSequence.current += 1; setProposal(null); setCalculating(false); setLoading(true); setLoadError(null); setNotice(null);
    try {
      const verified = await readFieldOwner();
      if (!mounted.current || generation !== epoch.current) return;
      if (ownerRef.current && !sameOwner(ownerRef.current, verified)) { invalidate(); return; }
      ownerRef.current = verified; setOwner(verified);
      const valid = () => mounted.current && generation === epoch.current && !!ownerRef.current && sameOwner(ownerRef.current, verified);
      if (!navigator.onLine) return;
      const data = await fetchFieldJob(appointmentURL, {}, verified, valid);
      const rows = readAppointments(data);
      if (!valid()) return;
      for (const row of rows) expectedNotes.current.set(row.id, row.notes ?? null);
      setJobs(rows.map(row => notesDrafts.current.has(row.id) ? { ...row, notes: notesDrafts.current.get(row.id)! } : row));
      // Pending offline entries remain held until their saved fields are observed again.
      for (const [id, queued] of offlinePendingRef.current) {
        const saved = rows.find(job => job.id === id);
        if (saved && saved.status === queued.status && (saved.notes ?? '') === queued.notes && saved.updated_at !== queued.updated_at) {
          offlinePendingRef.current.delete(id);
          if (notesDrafts.current.get(id) === queued.notes) notesDrafts.current.delete(id);
        }
      }
      setOfflinePending([...offlinePendingRef.current.keys()]);
      if (localDatabase) {
        try {
          const { getPowerSyncDB } = await import('../../../lib/powersync/db');
          const db = await getPowerSyncDB();
          if (!valid()) return;
          await db.writeTransaction(async tx => {
            if (!valid()) throw new Error('Session changed');
            await tx.execute('DELETE FROM appointments WHERE tenant_id = ?', [verified.tenantId]);
            for (const row of rows) {
              if (!valid()) throw new Error('Session changed');
              await tx.execute('INSERT INTO appointments (id, tenant_id, customer_id, customer_name, job_template_id, job_name, status, scheduled_start_time, scheduled_end_time, location_address, notes, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)', [row.id, verified.tenantId, row.customer_id, row.customer_name, row.job_template_id, row.job_name, row.status, row.scheduled_start_time, row.scheduled_end_time, row.location_address ?? null, row.notes ?? null, row.updated_at ?? null]);
            }
            if (!valid()) throw new Error('Session changed');
          });
        } catch { if (valid()) setNotice('Schedule loaded. The offline copy could not be saved in this browser.'); }
      }
    } catch (error) {
      if (mounted.current && generation === epoch.current) {
        if (/session/i.test(message(error))) invalidate();
        else setLoadError("We couldn't load today's schedule. Reload to try again.");
      }
    } finally { if (mounted.current && generation === epoch.current) setLoading(false); }
  }, [invalidate, localDatabase]);

  useEffect(() => {
    mounted.current = true; setIsOffline(!navigator.onLine); void loadSchedule();
    const online = () => setIsOffline(false), offline = () => setIsOffline(true);
    const storage = (event: StorageEvent) => { if (event.key === null || event.key === QUEUE_IDENTITY_EPOCH_KEY) invalidate(); };
    window.addEventListener('online', online); window.addEventListener('offline', offline);
    window.addEventListener('omnisolo_auth_changed', invalidate); window.addEventListener('storage', storage); window.addEventListener('pagehide', invalidate);
    return () => { mounted.current = false; epoch.current += 1; proposalSequence.current += 1; quoteSequence.current += 1; window.removeEventListener('online', online); window.removeEventListener('offline', offline); window.removeEventListener('omnisolo_auth_changed', invalidate); window.removeEventListener('storage', storage); window.removeEventListener('pagehide', invalidate); };
  }, [invalidate, loadSchedule]);
  const showCachedJobs = useCallback((rows: Appointment[]) => {
    if (!ownerRef.current || !mounted.current) return;
    const owned = rows.filter(row => row.tenant_id === ownerRef.current?.tenantId);
    if (!owned.length) return;
    try {
      const parsed = readAppointments({ appointments: owned });
      setJobs(previous => previous.length ? previous : parsed);
      for (const row of parsed) if (!expectedNotes.current.has(row.id)) expectedNotes.current.set(row.id, row.notes ?? null);
    } catch { setLoadError('The offline schedule could not be verified. Reconnect and reload it.'); }
  }, []);

  const previewRoute = async (observed: Appointment[], expected: QueueOwner, generation: number) => {
    const sequence = ++proposalSequence.current; setCalculating(true); setProposal(null);
    const valid = () => current(generation, expected) && sequence === proposalSequence.current;
    try {
      const data = await fetchFieldJob(routeURL, jsonPost({ appointments: observed, commit: false }), expected, valid);
      const proposed = readRouteProposal(data, observed, false);
      if (valid()) setProposal({ kind: 'route', observed, key: crypto.randomUUID(), entries: proposed.map(row => ({ original: observed.find(job => job.id === row.id)!, proposed: row, key: crypto.randomUUID() })) });
    } catch (error) { if (valid()) reportError(error); }
    finally { if (valid()) setCalculating(false); }
  };
  const handleStatusChange = async (jobId: string, newStatus: string) => {
    const job = jobsRef.current.find(row => row.id === jobId), expected = ownerRef.current;
    if (!job || !expected || pendingJobs.current.has(jobId) || savingProposalRef.current || offlinePendingRef.current.has(jobId)) return;
    const generation = epoch.current, valid = () => current(generation, expected);
    pendingJobs.current.add(jobId); pendingStatuses.current.set(jobId, newStatus); setPending([...pendingJobs.current]); setLoadError(null); setNotice(null); cancelProposal();
    try {
      const body: AppointmentWrite = { id: job.id, status: newStatus, expected_updated_at: observedTime(job), notes: job.notes ?? '' };
      if (isOffline) {
        const eventId = crypto.randomUUID(), timestamp = Date.now();
        await SyncManager.getInstance().enqueue({ id: eventId, type: 'sync_event', ...(newStatus === 'Completed' ? { field_completion: { job_id: job.id, customer_id: job.customer_id, notes: body.notes ?? '' } } : {}), payload: { id: eventId, entity_type: 'appointment', entity_id: job.id, action_type: 'UpdateStatus', base_version: job.base_version ?? job.version ?? 0, payload: { status: newStatus, expected_status: job.status, expected_notes: expectedNotes.current.get(jobId), expected_updated_at: body.expected_updated_at, notes: body.notes, scheduled_start_time: job.scheduled_start_time, scheduled_end_time: job.scheduled_end_time } }, timestamp }, expected);
        if (!valid()) return;
        offlinePendingRef.current.set(jobId, { status: newStatus, notes: body.notes!, updated_at: body.expected_updated_at }); setOfflinePending([...offlinePendingRef.current.keys()]);
        setJobs(rows => rows.map(row => row.id === jobId ? { ...row, status: newStatus } : row));
        return;
      }
      const serialized = JSON.stringify(body), previous = statusAttempts.current.get(jobId);
      const key = previous?.body === serialized ? previous.key : crypto.randomUUID();
      statusAttempts.current.set(jobId, { body: serialized, key });
      const data = await fetchFieldJob(appointmentURL, jsonPost(body, key), expected, valid);
      const saved = readAppointmentUpdate(data, job, body);
      if (!valid()) return;
      expectedNotes.current.set(jobId, saved.notes ?? null); statusAttempts.current.delete(jobId);
      if (notesDrafts.current.get(jobId) === body.notes) notesDrafts.current.delete(jobId);
      const apply = (row: Appointment) => row.id === jobId ? { ...saved, notes: notesDrafts.current.get(jobId) ?? saved.notes } : row;
      setJobs(rows => rows.map(apply));
      if (newStatus === 'Completed') {
        const { queued, unconfirmed } = await queueCompletionFollowups(jobId, job.customer_id, body.notes ?? '', key, Date.parse(saved.updated_at!), expected, valid);
        if (!valid()) return;
        setNotice(`Job completion saved. ${queued} follow-up requests queued, pending processing and review.`);
        if (unconfirmed) setLoadError(`${unconfirmed} follow-up queue writes could not be confirmed. Check the offline queue before retrying; the job completion is saved.`);
        await previewRoute(jobsRef.current.map(apply), expected, generation);
      }
    } catch (error) { if (valid()) reportError(error); }
    finally { if (valid()) { pendingJobs.current.delete(jobId); pendingStatuses.current.delete(jobId); setPending([...pendingJobs.current]); } }
  };
  const handleRunningLate = async (jobId: string) => {
    const expected = ownerRef.current;
    if (!expected || isOffline || savingProposalRef.current || pendingJobs.current.size || offlinePendingRef.current.size) return;
    const generation = epoch.current, sequence = ++proposalSequence.current, observed = jobsRef.current.map(job => ({ ...job }));
    const valid = () => current(generation, expected) && sequence === proposalSequence.current;
    setCalculating(true); setProposal(null); setLoadError(null); setNotice(null);
    try {
      observed.forEach(observedTime);
      const data = await fetchFieldJob('/api/v1/field-ops/running-late', jsonPost({ appointments: observed, delayJobId: jobId }), expected, valid);
      const proposed = readRouteProposal(data, observed, false);
      const entries = proposed.flatMap(row => {
        const original = observed.find(job => job.id === row.id)!;
        if (row.scheduled_start_time === original.scheduled_start_time && row.scheduled_end_time === original.scheduled_end_time) return [];
        if (terminalJob(original)) throw new Error('The delay proposal tried to change a closed job. Reload and review it again.');
        return [{ original, proposed: row, key: crypto.randomUUID() }];
      });
      if (valid()) { if (entries.length) setProposal({ kind: 'delay', entries, observed, key: crypto.randomUUID() }); else setNotice('There are no later appointments to move.'); }
    } catch (error) { if (valid()) reportError(error); }
    finally { if (valid()) setCalculating(false); }
  };
  const handleApproveProposal = async () => {
    const expected = ownerRef.current, selected = proposal;
    if (!expected || !selected || savingProposalRef.current || pendingJobs.current.size || isOffline) return;
    const generation = epoch.current, sequence = proposalSequence.current;
    const valid = () => current(generation, expected) && sequence === proposalSequence.current;
    savingProposalRef.current = true; setSavingProposal(true); setLoadError(null); setNotice(null);
    try {
      if (selected.kind === 'route') {
        const data = await fetchFieldJob(routeURL, jsonPost({ appointments: selected.observed, commit: true }, selected.key), expected, valid);
        const saved = readRouteProposal(data, selected.observed, true);
        if (valid()) {
          setJobs(previous => saved.map(row => ({ ...row, notes: notesDrafts.current.get(row.id) ?? previous.find(job => job.id === row.id)?.notes ?? row.notes })));
          setProposal(null); setNotice('Route saved. No customer notifications were sent.');
        }
      } else {
        const remaining: ProposalEntry[] = [];
        for (const entry of selected.entries) {
          if (!valid()) return;
          try {
            const body: AppointmentWrite = { id: entry.original.id, status: entry.original.status, expected_updated_at: observedTime(entry.original), scheduled_start_time: entry.proposed.scheduled_start_time, scheduled_end_time: entry.proposed.scheduled_end_time };
            const data = await fetchFieldJob(appointmentURL, jsonPost(body, entry.key), expected, valid);
            const saved = readAppointmentUpdate(data, entry.original, body);
            if (!valid()) return;
            expectedNotes.current.set(saved.id, saved.notes ?? null);
            setJobs(previous => previous.map(row => row.id === saved.id ? { ...saved, notes: notesDrafts.current.get(row.id) ?? saved.notes } : row));
          } catch (error) {
            if (!valid()) return;
            if (/session/i.test(message(error))) { invalidate(); return; }
            remaining.push({ ...entry, error: message(error) });
          }
        }
        if (valid()) { setProposal(remaining.length ? { ...selected, entries: remaining } : null); setNotice(remaining.length ? `${selected.entries.length - remaining.length} schedule changes saved; ${remaining.length} still need attention.` : 'Schedule saved. No customer notifications were sent.'); }
      }
    } catch (error) { if (valid()) reportError(error); }
    finally { if (valid()) { savingProposalRef.current = false; setSavingProposal(false); } }
  };
  const closeQuote = () => { quoteSequence.current += 1; quoteBusy.current = false; setVoiceQuoteJobId(null); setDraftQuoteResult(null); setVoiceTranscript(''); setQuoteError(null); setQuoteHeld(false); setDraftingQuote(false); };
  const openQuote = (id: string) => {
    closeQuote(); setVoiceQuoteJobId(id);
    const expected = ownerRef.current;
    if (!expected) return;
    try {
      const existing = localStorage.getItem(quoteRequestKey(expected, id));
      if (!existing) return;
      const record: { transcript?: unknown; id?: unknown } = JSON.parse(existing);
      setQuoteHeld(true);
      if (typeof record.transcript === 'string') setVoiceTranscript(record.transcript);
      if (typeof record.id === 'string' && quoteIdPattern.test(record.id)) setDraftQuoteResult({ id: record.id });
      else setQuoteError('An earlier quote request needs reconciliation. Check existing quotes before requesting another draft. Your notes have been kept.');
    } catch { setQuoteHeld(true); setQuoteError('The earlier quote request could not be verified. Check existing quotes before requesting another draft.'); }
  };
  const handleDraftVoiceQuote = async () => {
    const job = jobsRef.current.find(row => row.id === voiceQuoteJobId), expected = ownerRef.current;
    if (!job || !job.customer_id || !expected || !voiceTranscript.trim() || quoteBusy.current || quoteHeld || isOffline) return;
    const generation = epoch.current, sequence = ++quoteSequence.current;
    const valid = () => current(generation, expected) && sequence === quoteSequence.current;
    quoteBusy.current = true; setDraftingQuote(true); setQuoteError(null);
    try {
      if (!navigator.locks) throw new Error('This browser cannot safely coordinate quote requests. Use the quote workspace.');
      const data = await navigator.locks.request(quoteRequestKey(expected, job.id), () => fetchFieldJob('/api/v1/quotes/draft_agent', jsonPost({ inquiry: voiceTranscript, customer_id: job.customer_id }), expected, valid, () => {
        // The cross-tab lock covers the marker check and dispatch. Unknown outcomes stay held.
        if (localStorage.getItem(quoteRequestKey(expected, job.id))) { setQuoteHeld(true); throw new Error('An earlier quote request needs reconciliation.'); }
        localStorage.setItem(quoteRequestKey(expected, job.id), JSON.stringify({ transcript: voiceTranscript }));
        setQuoteHeld(true);
      })) as { id?: unknown; success?: boolean; error?: unknown };
      if (!data || data.success === false || data.error != null || typeof data.id !== 'string' || !quoteIdPattern.test(data.id)) throw new Error('The quote request could not be confirmed. Your notes have been kept. Check existing quotes before requesting another draft.');
      if (valid()) {
        setDraftQuoteResult({ id: data.id });
        try { localStorage.setItem(quoteRequestKey(expected, job.id), JSON.stringify({ transcript: voiceTranscript, id: data.id })); }
        catch { setQuoteError('The quote request was accepted, but its local receipt could not be saved. Keep the review link before leaving this page.'); }
      }
    } catch (error) { if (valid()) { if (/session/i.test(message(error))) invalidate(); else setQuoteError(`${message(error)} Check existing quotes before requesting another draft.`); } }
    finally { if (valid()) { quoteBusy.current = false; setDraftingQuote(false); } }
  };

  if (loading) return <div className="p-4 bg-gray-50 min-h-screen flex items-center justify-center">Loading schedule...</div>;
  return <div className="p-4 bg-gray-50 min-h-screen">
    {localDatabase && isOffline && owner && <CachedJobs owner={owner} onRows={showCachedJobs} />}
    <div className="flex justify-between items-center mb-6 gap-3 flex-wrap">
      <h1 className="text-2xl font-bold font-outfit text-gray-900">Today's Route</h1>
      <button className="px-3 py-2 rounded-lg border border-gray-300 text-sm" disabled={pending.length > 0 || savingProposal || draftingQuote} onClick={() => void loadSchedule()}>Reload schedule</button>
      {isOffline && <div className="bg-white border border-gray-200 px-3 py-2 rounded-full text-sm">☁️ Offline Mode · Changes await server confirmation</div>}
    </div>
    {loadError && <div role="alert" className="mb-6 rounded-xl border border-red-200 bg-red-50 p-4 text-sm text-red-800">{loadError}</div>}
    {notice && <div role="status" className="mb-4 p-3 rounded-xl bg-blue-50 text-sm text-blue-800">{notice}</div>}
    {calculating && <div className="mb-4 p-4 rounded-xl bg-blue-50">Calculating a schedule proposal… <button onClick={cancelProposal} className="underline ml-3">Cancel calculation</button></div>}
    {proposal && <section aria-label="Schedule proposal" className="mb-6 p-4 bg-orange-50 border border-orange-200 rounded-xl shadow-sm">
      <h2 className="font-semibold text-gray-900">{proposal.kind === 'delay' ? 'Review 30-minute schedule changes' : 'Review route order'}</h2>
      <p className="text-sm text-gray-700 my-2">No customer notifications are sent by this feature. Contact clients separately if needed.</p>
      <ul className="space-y-2 mb-3">{proposal.entries.map(entry => <li key={entry.original.id} className="text-sm">
        <span className="font-medium">{entry.proposed.customer_name}</span>: {proposal.kind === 'delay' && <>{time(entry.original.scheduled_start_time)} → </>}{time(entry.proposed.scheduled_start_time)} – {time(entry.proposed.scheduled_end_time)}
        {entry.error && <p role="alert" className="text-red-800 mt-1">{entry.error}</p>}
      </li>)}</ul>
      <div className="flex gap-2">
        <button disabled={savingProposal || isOffline} onClick={() => void handleApproveProposal()} className="px-3 py-2 bg-orange-600 text-white text-sm font-semibold rounded-lg disabled:opacity-50">{savingProposal ? 'Saving…' : proposal.kind === 'delay' ? 'Save schedule' : 'Save route'}</button>
        <button disabled={savingProposal} onClick={cancelProposal} className="px-3 py-2 bg-white border border-gray-300 text-sm rounded-lg disabled:opacity-50">Cancel</button>
      </div>
    </section>}
    {!jobs.length && !loadError && <p className="text-gray-600">{isOffline ? 'No verified offline schedule is available. Reconnect and reload your schedule.' : 'No appointments are scheduled.'}</p>}
    <div className="space-y-4">{jobs.map(job => {
      const busy = pending.includes(job.id) || savingProposal || offlinePending.includes(job.id);
      const nextStatus = ['Requested', 'Pending', 'Scheduled', 'Confirmed'].includes(job.status) ? 'En-Route' : job.status === 'En-Route' ? 'In-Progress' : 'Completed';
      return <div key={job.id} data-testid={`job-card-${job.id}`} className="bg-white/65 backdrop-blur-[30px] backdrop-saturate-[2.1] shadow-sm border border-white/40 overflow-hidden rounded-[16px]">
        <div className="p-5 border-b border-gray-100 bg-gray-50/50">
          <div className="flex justify-between items-start mb-2"><h3 className="font-bold text-lg text-gray-900">{job.customer_name}</h3><span className={`px-2 py-1 text-xs font-semibold rounded-full ${job.status === 'Completed' ? 'bg-green-100 text-green-700' : job.status === 'In-Progress' ? 'bg-yellow-100 text-yellow-700' : job.status === 'En-Route' ? 'bg-purple-100 text-purple-700' : 'bg-blue-100 text-blue-700'}`}>{job.status.toUpperCase()}</span></div>
          <p className="text-gray-600 text-sm mb-1">📍 {job.location_address || 'Address not provided'}</p><p className="text-gray-600 text-sm">🔧 {job.job_name}</p>
          <p className="text-gray-500 text-xs mt-2 font-medium">⏱ {time(job.scheduled_start_time)} – {time(job.scheduled_end_time)}</p>
          {offlinePending.includes(job.id) && <p className="text-sm text-orange-800 mt-2">Saved locally, awaiting server confirmation. Reconnect and reload before making another change.</p>}
          {pending.includes(job.id) && <p className="text-sm text-gray-600 mt-2">Saving change…</p>}
        </div>
        {!terminalJob(job) ? <div className="p-5">
          <div className="flex justify-between items-center mb-2"><label htmlFor={`notes-${job.id}`} className="block text-sm font-medium text-gray-700">Service Notes & Potential Follow-ups</label><button onClick={() => openQuote(job.id)} className="w-10 h-10 rounded-full bg-[#0066FF]/10 text-[#0066FF]" title="Voice-to-Quote" data-testid={`voice-quote-btn-${job.id}`}>🎤</button></div>
          <textarea id={`notes-${job.id}`} className="w-full border border-gray-300 rounded-lg p-3 text-sm min-h-[80px]" placeholder="E.g., Needs a replacement quote." value={job.notes ?? ''} disabled={pendingStatuses.current.get(job.id) === 'Completed'} onChange={event => { const notes = event.target.value; notesDrafts.current.set(job.id, notes); setJobs(rows => rows.map(row => row.id === job.id ? { ...row, notes } : row)); }} />
          <div className="mt-4 flex gap-2 flex-col sm:flex-row">
            <button disabled={busy} className="flex-1 bg-[#0071E3] text-white font-semibold py-3 rounded-xl min-h-[44px] disabled:opacity-50" onClick={() => void handleStatusChange(job.id, nextStatus)}>{nextStatus === 'En-Route' ? 'Heading to Job' : nextStatus === 'In-Progress' ? 'Start Work' : 'Job Done'}</button>
            <button disabled={isOffline || busy || pending.length > 0 || offlinePending.length > 0} className="flex-1 bg-red-50 text-red-600 font-semibold py-3 rounded-xl min-h-[44px] disabled:opacity-50" onClick={() => void handleRunningLate(job.id)}>Running Late</button>
          </div>
        </div> : job.notes && <div className="p-5 bg-blue-50/50"><p className="text-sm font-medium text-gray-800">{offlinePending.includes(job.id) || notesDrafts.current.has(job.id) ? 'Local Notes:' : 'Saved Notes:'}</p><p className="text-sm text-gray-600 italic">“{job.notes}”</p><button onClick={() => openQuote(job.id)} data-testid={`voice-quote-btn-${job.id}`} className="text-sm text-[#0071E3] font-semibold mt-2">Review follow-up quote</button></div>}
      </div>;
    })}</div>
    {voiceQuoteJobId && <div role="dialog" aria-label="Quote draft" className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/40 backdrop-blur-sm"><div className="bg-white rounded-[24px] shadow-2xl p-6 w-full max-w-md">
      <div className="flex justify-between items-center mb-4"><h2 className="text-lg font-bold font-outfit">🎤 Voice-to-Quote</h2><button aria-label="Close quote draft" onClick={closeQuote}>✕</button></div>
      <p className="text-sm text-gray-600 mb-3">Type service notes to request a quote draft. This does not send a quote to the customer.</p>
      {quoteError && <p role="alert" className="text-red-800 text-sm mb-3">{quoteError}</p>}
      {!draftQuoteResult ? <><textarea aria-label="Quote notes" data-testid="voice-transcript-input" className="w-full border border-gray-300 rounded-lg p-3 text-sm min-h-[100px] mb-4" placeholder="Describe the work and any confirmed amounts…" value={voiceTranscript} onChange={event => setVoiceTranscript(event.target.value)} disabled={draftingQuote} /><button data-testid="generate-quote-btn" onClick={() => void handleDraftVoiceQuote()} disabled={draftingQuote || quoteHeld || !voiceTranscript.trim() || isOffline} className="w-full bg-[#0066FF] text-white font-semibold py-3 disabled:opacity-50 rounded-lg">{draftingQuote ? 'Drafting…' : 'Generate Draft Quote'}</button></> : <div data-testid="draft-quote-result" className="flex flex-col gap-3">
        <p className="bg-blue-100 text-blue-800 p-3 rounded-xl text-sm">Draft requested. Preparation is pending; no quote has been sent.</p>
        <a href={`/quoting?id=${encodeURIComponent(draftQuoteResult.id)}`} className="text-[#0066FF] underline">Review requested quote</a><button onClick={closeQuote} className="bg-[#0066FF] text-white font-semibold py-3 rounded-lg">Close draft</button>
      </div>}
    </div></div>}
  </div>;
}

export default function FieldOpsJobsPage() {
  return <PowerSyncProvider unsupportedFallback={<FieldOpsJobsPageContent />}><FieldOpsJobsPageContent localDatabase /></PowerSyncProvider>;
}
