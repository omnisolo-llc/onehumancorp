'use client';

import Link from 'next/link';
import { useCallback, useEffect, useRef, useState } from 'react';
import { assertBuilderScope } from '../builder/ownedDraft';
import { fetchForOwnedDefinition } from '../onboarding/draftSession';
import { readCatalogue, type Definition, type Installation } from './definitions';
import { useDefinitionSession } from './useDefinitionSession';
import { OperationRecovery } from './OperationRecovery';

function mergeRecords<T extends { id: string }>(previous: T[], incoming: T[], key: (item: T) => string = item => item.id): T[] {
  const rows = new Map(previous.map(item => [key(item), item]));
  for (const item of incoming) {
    const old = rows.get(key(item));
    if (old && JSON.stringify(old) !== JSON.stringify(item)) throw new Error('The catalogue returned conflicting immutable records. Reload before continuing.');
    rows.set(key(item), item);
  }
  return [...rows.values()];
}
export default function AgentMarketplacePage() {
  const [query, setQuery] = useState(''); const [retry, setRetry] = useState(0);
  const [definitions, setDefinitions] = useState<Definition[]>([]);
  const [installations, setInstallations] = useState<Installation[]>([]);
  const [cursors, setCursors] = useState<{ public: string | null; installed: string | null }>({ public: null, installed: null });
  const [loading, setLoading] = useState(true); const [error, setError] = useState('');
  const [review, setReview] = useState<Definition | null>(null);
  const records = useRef<{ definitions: Definition[]; installations: Installation[] }>({ definitions: [], installations: [] });
  const loadEpoch = useRef(0); const controller = useRef<AbortController | null>(null); const moreBusy = useRef(false);
  const session = useDefinitionSession({ retired: () => {
    ++loadEpoch.current; controller.current?.abort(); moreBusy.current = false;
    records.current = { definitions: [], installations: [] }; setDefinitions([]); setInstallations([]); setCursors({ public: null, installed: null }); setReview(null); setQuery(''); setLoading(true); setError('');
  } });
  const load = useCallback(async (append: boolean, publicCursor?: string | null, installedCursor?: string | null) => {
    const scope = session.scope.current; if (!scope) return;
    if (append && moreBusy.current) return;
    const current = ++loadEpoch.current; controller.current?.abort(); const request = new AbortController(); controller.current = request;
    moreBusy.current = true; setLoading(true); setError('');
    if (!append) { records.current = { definitions: [], installations: [] }; setDefinitions([]); setInstallations([]); setCursors({ public: null, installed: null }); setReview(null); }
    try {
      if ([...query].length > 256) throw new Error('Search must be at most 256 characters.');
      const params = new URLSearchParams({ q: query });
      if (publicCursor) params.set('cursor', publicCursor); if (installedCursor) params.set('installation_cursor', installedCursor);
      const response = await fetchForOwnedDefinition('/api/v1/agents/definitions?' + params, { method: 'GET', signal: request.signal }, scope.owner);
      if (response.status !== 200) throw new Error('Failed to fetch agents. The catalogue is unavailable.');
      const page = await readCatalogue(await response.json()); assertBuilderScope(scope);
      if (current !== loadEpoch.current || session.scope.current !== scope) return;
      // Each cursor advances only its collection. A repeated first page from the
      // other collection must not reset that collection's completed cursor.
      const next = { definitions: append ? mergeRecords(records.current.definitions, page.definitions, item => item.id + ':' + item.version) : page.definitions, installations: append ? mergeRecords(records.current.installations, page.installations) : page.installations };
      records.current = next; setDefinitions(next.definitions); setInstallations(next.installations);
      setCursors(previous => ({ public: !append || publicCursor ? page.next_cursor : previous.public, installed: !append || installedCursor ? page.next_installation_cursor : previous.installed }));
    } catch (cause) {
      if (current === loadEpoch.current && session.scope.current === scope) setError(cause instanceof Error ? cause.message : 'Failed to fetch agents.');
    } finally {
      if (current === loadEpoch.current) { moreBusy.current = false; setLoading(false); }
    }
  }, [query, session.scope]);
  useEffect(() => {
    void load(false);
    return () => { ++loadEpoch.current; controller.current?.abort(); moreBusy.current = false; };
  }, [load, retry, session.version]);
  useEffect(() => {
    if (session.saved?.state === 'confirmed' && session.saved.receipt?.installation) {
      const installation = session.saved.receipt.installation;
      try { const next = mergeRecords(records.current.installations, [installation]); records.current.installations = next; setInstallations(next); }
      catch (cause) { setError(cause instanceof Error ? cause.message : 'The installation receipt conflicts with this view.'); }
    }
  }, [session.saved]);
  async function install() {
    if (!review || session.phase !== 'ready') return;
    const result = await session.perform({ kind: 'install', request_id: crypto.randomUUID(), definition: review });
    if (result?.status === 'installed_inactive') setReview(null);
  }
  const canInstall = session.phase === 'ready' && !loading && !error && !cursors.installed;
  return <main className="min-h-screen bg-[#f4f6f8] p-8 font-outfit">
    <div className="max-w-6xl mx-auto">
      <header className="flex justify-between gap-4 mb-6"><div><h1 className="text-4xl font-bold">Agent Marketplace</h1><p>Review public definitions and save inactive copies for your account.</p></div><Link href="/agent-marketplace/publish">Publish New Agent</Link></header>
      <p role="status" aria-label="Agent operation status">{session.message}</p>
      {(session.saved?.state === 'pending' || session.phase === 'held') && <OperationRecovery saved={session.saved} available={!!session.scope.current} busy={session.phase === 'working'} recover={() => void session.perform()} replay={() => { if (session.saved?.state === 'pending') void session.perform(session.saved.operation, true); }} />}
      {session.saved?.state === 'confirmed' && session.saved.receipt?.status === 'installed_inactive' && <p role="status">Agent installed successfully as an inactive definition. No work started. <Link href="/agents">Open Agents to review a separate Hire action</Link></p>}
      <label htmlFor="agent-search" className="sr-only">Search agents</label><input id="agent-search" placeholder="Search for agents..." value={query} onChange={event => setQuery(event.target.value)} className="w-full border rounded-xl p-4 my-6" />
      {error && <div role="alert"><p>{error}</p><button type="button" onClick={() => setRetry(value => value + 1)}>Retry marketplace</button></div>}
      {loading && <p role="status">Loading agent definitions…</p>}
      {!loading && !error && definitions.length === 0 && <p>No agents found.</p>}
      <div className="grid md:grid-cols-2 gap-6">{definitions.map(item => {
        const installed = installations.some(value => value.definition_id === item.id && value.version === item.version && value.digest === item.digest);
        return <article key={item.id + ':' + item.version} aria-label={item.name} className="rounded-xl bg-white p-6 space-y-3">
          <h2 className="text-xl font-semibold">{item.name}</h2><p>{item.description}</p><p>{item.source === 'first_party' ? 'First-party definition' : 'Community definition'} · Version {item.version}</p><p>Role: {item.role}</p>
          <button type="button" aria-pressed={installed} disabled={installed || !canInstall} onClick={() => setReview(item)}>{installed ? 'Installed' : 'Install Agent'}</button>
        </article>;
      })}</div>
      {cursors.public && <button type="button" disabled={loading} onClick={() => void load(true, cursors.public)}>Load More Definitions</button>}
      {cursors.installed && <div><p>Load the remaining private installations before installing another definition.</p><button type="button" disabled={loading} onClick={() => void load(true, null, cursors.installed)}>Load More Installations</button></div>}
      {review && <section aria-label="Review inactive installation" className="my-6 rounded-xl bg-white p-6 space-y-3">
        <h2>Review {review.name}</h2><p>{review.description}</p><p>Role: {review.role}</p><pre className="whitespace-pre-wrap break-words">{review.system_prompt}</pre><p>Version {review.version} · Digest {review.digest}</p>
        <p>This saves an inactive definition for your verified account and business. No work starts, and no tools or network permissions are granted. Hire remains a separate authorized action.</p>
        <button type="button" disabled={!canInstall} onClick={() => void install()}>Confirm Inactive Installation</button><button type="button" disabled={session.phase === 'working'} onClick={() => setReview(null)}>Cancel</button>
      </section>}
    </div>
  </main>;
}
