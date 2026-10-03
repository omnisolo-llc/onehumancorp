'use client';

import Link from 'next/link';
import { useCallback, useEffect, useRef, useState } from 'react';
import { Card, CardContent } from '@/components/ui/card';
import { useClipboardFeedback } from '@/hooks/useClipboardFeedback';
import { currentVerifiedQueueLease, hasVerifiedOfflineQueueOwner, QUEUE_IDENTITY_EPOCH_KEY, sameOwner, subscribeQueueIdentityReadiness } from '@/lib/sync/queueIdentity';
import { builderScopeActive, openBuilderScope, type BuilderScope } from '../builder/ownedDraft';
import { publishedSitePath, readSavedPublicationOperation, refreshPublicationOperation, type PublicationChannel } from '../builder/publicationOperations';
import { subscribeOnboardingInvalidation } from '../onboarding/draftSession';

const channels: Array<{ value: PublicationChannel; label: string }> = [
  { value: 'storefront-builder', label: 'Storefront builder' },
  { value: 'website-builder', label: 'Website builder' },
  { value: 'builder', label: 'Site builder' },
  { value: 'brand-studio', label: 'Brand studio' },
  { value: 'public-bio', label: 'Public bio' },
];
function embedFor(path: string): string {
  const url = new URL(path, window.location.origin);
  if (url.origin !== window.location.origin || !/^\/api\/v1\/public\/sites\/[0-9a-f-]{36}$/.test(url.pathname) || url.search || url.hash) throw new Error('Published address unavailable');
  return `<iframe src="${url.href}" title="Published storefront" width="100%" height="600" style="border-radius:12px;border:1px solid #eaeaea;"></iframe>\n<div style="text-align:center;font-size:12px;margin-top:8px;"><a href="https://omnisolo.co" target="_blank" rel="noopener noreferrer">⚡ OmniSolo</a></div>`;
}
// Other maintained panels can be verifying the same canonical owner at mount.
// Wait for that verification, without treating its temporary pause as identity.
function awaitOwner(scope: BuilderScope, current: () => boolean, signal: AbortSignal): Promise<void> {
  return new Promise((resolve, reject) => {
    let settled = false; let unsubscribe = () => {};
    const finish = (error?: Error) => {
      if (settled) return;
      settled = true; clearTimeout(timeout); unsubscribe(); signal.removeEventListener('abort', cancel);
      if (error) reject(error); else resolve();
    };
    const cancel = () => finish(new Error('Publication view retired'));
    const timeout = setTimeout(() => finish(new Error('Current account verification unavailable')), 5_000);
    signal.addEventListener('abort', cancel, { once: true });
    unsubscribe = subscribeQueueIdentityReadiness(() => {
      if (signal.aborted || !current() || !builderScopeActive(scope)) cancel();
      else if (hasVerifiedOfflineQueueOwner(scope.owner)) finish();
      else if (hasVerifiedOfflineQueueOwner()) finish(new Error('Current account changed'));
    });
    if (settled) unsubscribe();
  });
}

/** Saved operation IDs are hints; only a refreshed server receipt supplies a public URL. */
export function PublishedStorefrontEmbed() {
  const scope = useRef<BuilderScope | null>(null);
  const generation = useRef(0);
  const mounted = useRef(false);
  const busy = useRef(false);
  const scanRequest = useRef<AbortController | null>(null);
  const lease = useRef<ReturnType<typeof currentVerifiedQueueLease>>(null);
  const leaseExpiry = useRef<ReturnType<typeof setTimeout> | undefined>(undefined);
  const selected = useRef<PublicationChannel | null>(null);
  const [available, setAvailable] = useState<PublicationChannel[]>([]);
  const [channel, setChannel] = useState<PublicationChannel | null>(null);
  const [ready, setReady] = useState(false);
  const [pending, setPending] = useState(false);
  const [code, setCode] = useState('');
  const [context, setContext] = useState(0);
  const [message, setMessage] = useState('Checking saved publications…');
  const clipboard = useClipboardFeedback(String(context));
  const retire = useCallback(() => {
    scanRequest.current?.abort();
    clearTimeout(leaseExpiry.current); lease.current = null;
    generation.current++; scope.current = null; selected.current = null; busy.current = false;
    setAvailable([]); setChannel(null); setReady(false); setPending(false); setCode(''); setContext(value => value + 1);
    setMessage('Your session changed. Check saved publications to verify this account again.');
  }, []);
  const leaseCurrent = useCallback(() => {
    const original = lease.current; const current = currentVerifiedQueueLease();
    return !!original && original.expiresAt > Date.now() && !!current
      && sameOwner(original.owner, current.owner) && original.storageEpoch === current.storageEpoch;
  }, []);
  const captureLease = useCallback((intended: BuilderScope) => {
    const current = currentVerifiedQueueLease();
    if (!current || !sameOwner(current.owner, intended.owner)) throw new Error('Current account lease unavailable');
    lease.current = current; clearTimeout(leaseExpiry.current);
    leaseExpiry.current = setTimeout(retire, Math.min(current.expiresAt - Date.now(), 2_147_483_647));
  }, [retire]);
  const scan = useCallback(async () => {
    if (!mounted.current || busy.current) return;
    const version = ++generation.current; busy.current = true;
    scanRequest.current?.abort(); const request = new AbortController(); scanRequest.current = request;
    setPending(true); setReady(false); setCode(''); setContext(value => value + 1); setAvailable([]); setChannel(null); selected.current = null;
    const current = () => mounted.current && version === generation.current;
    try {
      const verified = await openBuilderScope(); if (!current()) return;
      scope.current = verified;
      await awaitOwner(verified, current, request.signal); captureLease(verified);
      const found: PublicationChannel[] = []; let unreadable = false;
      for (const candidate of channels) {
        await awaitOwner(verified, current, request.signal);
        try { if (await readSavedPublicationOperation(verified, candidate.value)) found.push(candidate.value); }
        catch { unreadable = true; }
        if (!current()) return;
      }
      await awaitOwner(verified, current, request.signal);
      if (!current() || !leaseCurrent()) { retire(); return; }
      setAvailable(found); selected.current = found[0] ?? null; setChannel(selected.current); setReady(true);
      setMessage(unreadable ? 'Some saved publication records could not be read. Recheck after verifying your account.' : found.length ? 'Select a saved publication and check its current server status.' : 'No saved publication exists for this account on this device. Publish a reviewed site in the builder first.');
    } catch {
      if (current()) { scope.current = null; setMessage('Saved publications could not be verified. Check your account and try again.'); }
    } finally { if (current()) { busy.current = false; setPending(false); } }
  }, [captureLease, leaseCurrent, retire]);
  useEffect(() => {
    mounted.current = true;
    const invalidation = subscribeOnboardingInvalidation(retire);
    const readiness = subscribeQueueIdentityReadiness(() => {
      if (!mounted.current) return;
      const current = scope.current;
      setReady(!!current && builderScopeActive(current) && leaseCurrent());
      if (lease.current && (lease.current.expiresAt <= Date.now()
        || hasVerifiedOfflineQueueOwner() && !leaseCurrent())) { retire(); return; }
      if (current && hasVerifiedOfflineQueueOwner() && !hasVerifiedOfflineQueueOwner(current.owner)) retire();
    });
    const storage = (event: StorageEvent) => {
      if (event.key === null || event.key === QUEUE_IDENTITY_EPOCH_KEY || event.key.startsWith('omnisolo_onboarding_owned_v1:')) retire();
    };
    window.addEventListener('storage', storage);
    void scan();
    return () => { mounted.current = false; generation.current++; scanRequest.current?.abort(); clearTimeout(leaseExpiry.current); lease.current = null; scope.current = null; busy.current = false; invalidation(); readiness(); window.removeEventListener('storage', storage); };
  }, [leaseCurrent, retire, scan]);

  const check = async (copy: boolean) => {
    if (busy.current) return;
    const intended = scope.current; const target = selected.current;
    if (!intended || !target || !builderScopeActive(intended) || !leaseCurrent()) { retire(); return; }
    const version = generation.current; busy.current = true; setPending(true);
    const current = () => mounted.current && generation.current === version && scope.current === intended && selected.current === target;
    try {
      const saved = await refreshPublicationOperation(intended, target, current);
      if (!current() || !builderScopeActive(intended)) return;
      if (!leaseCurrent()) { retire(); return; }
      captureLease(intended);
      const path = publishedSitePath(saved);
      if (!path) { setCode(''); setContext(value => value + 1); setMessage(`This publication is ${saved?.receipt?.status ?? 'unconfirmed'}. A published server receipt is required before embedding.`); return; }
      const verifiedCode = embedFor(path);
      if (copy && code === verifiedCode) {
        // The operation just refreshed through the canonical owner transport.
        // Retiring this view also retires clipboard completion feedback.
        await clipboard.copy(verifiedCode);
      } else {
        setCode(verifiedCode); setContext(value => value + 1);
        setMessage(copy ? 'The publication changed. Review the refreshed code before copying it.' : 'Published site verified. Copying checks its status again. Referral rewards are not created by this embed.');
      }
    } catch {
      if (current()) { setCode(''); setContext(value => value + 1); setMessage('The publication could not be verified. Its saved operation is preserved; check again to retry.'); }
    } finally { if (current()) { busy.current = false; setPending(false); } }
  };
  const active = ready && builderScopeActive(scope.current) && leaseCurrent();
  return <section aria-label="Published storefront embed">
    <Card className="border-white/20 dark:border-white/10 shadow-xl overflow-hidden backdrop-blur-[30px] bg-white/30 dark:bg-black/30">
      <CardContent className="p-6 space-y-4">
        <h2 className="text-2xl font-bold">Embed Your Business</h2>
        <p>Embed an existing published site for your verified account. This does not create a publication, referral link, discount, or reward.</p>
        <button type="button" className="app-button" disabled={pending} onClick={() => void scan()}>Check saved publications</button>
        {available.length > 0 && <label className="block">Saved publication
          <select aria-label="Saved publication" className="block border rounded p-2" disabled={!active || pending} value={channel ?? ''} onChange={event => {
            if (!leaseCurrent()) { retire(); return; }
            const next = channels.find(candidate => candidate.value === event.target.value)?.value;
            if (!next || !available.includes(next)) return;
            generation.current++; selected.current = next; setChannel(next); setCode(''); setContext(value => value + 1); setMessage('Check this publication before copying its embed.');
          }}>{channels.filter(candidate => available.includes(candidate.value)).map(candidate => <option key={candidate.value} value={candidate.value}>{candidate.label}</option>)}</select>
        </label>}
        <div className="flex flex-wrap gap-3">
          <button type="button" className="app-button" disabled={!active || !channel || pending} onClick={() => void check(false)}>Check publication</button>
          <button type="button" className="app-button" disabled={!active || !code || pending || clipboard.state === 'pending'} onClick={() => void check(true)}>Copy Embed Code</button>
          <Link className="app-button" href="/storefront-builder">Open storefront builder</Link>
        </div>
        {active && code && <textarea aria-label="Published embed code" readOnly value={code} className="w-full h-32 border rounded p-3 font-mono text-xs" />}
        <p role={clipboard.state === 'error' && active ? 'alert' : 'status'}>{pending ? clipboard.state === 'pending' ? 'Copying…' : 'Checking current publication status…' : !active && available.length > 0 ? 'Verify the current account before using this publication.' : active && code && clipboard.message ? clipboard.message : message}</p>
      </CardContent>
    </Card>
  </section>;
}
