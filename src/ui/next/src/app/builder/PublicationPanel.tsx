'use client';

import { useEffect, useRef, useState } from 'react';
import canonicalize from 'canonicalize';
import { hasVerifiedOfflineQueueOwner, sameOwner, subscribeQueueIdentityReadiness, type QueueOwner } from '@/lib/sync/queueIdentity';
import { subscribeOnboardingInvalidation } from '../onboarding/draftSession';
import { builderScopeActive, openBuilderScope, type BuilderScope } from './ownedDraft';
import { prepareSiteSnapshot } from './publicationContracts';
import { preparePublicationReview, publishedSitePath, readSavedPublicationOperation, refreshPublicationOperation, revokePublication, submitPublicationReview, type PublicationChannel, type PublicationReview, type SavedPublicationOperation } from './publicationOperations';

export type PublicationPanelProps = {
  channel: PublicationChannel;
  expectedOwner: QueueOwner | null;
  getSnapshot: () => unknown;
  isEditorCurrent: () => boolean;
  onRetired: (owner: QueueOwner, reason: string) => void;
};
function operationStatus(saved: SavedPublicationOperation | null): string {
  if (!saved) return 'Your private draft has not been published from this view.';
  if (saved.phase === 'publish_unknown') return 'Publication could not be confirmed. Check its saved status before publishing again.';
  if (saved.phase === 'revoke_unknown') return 'Revocation could not be confirmed. This version remains held until its revoked receipt is recorded.';
  if (saved.phase === 'rejected') return 'Publication was rejected before saving. Review the content and your current authority.';
  switch (saved.receipt?.status) {
    case 'pending': case 'processing': return 'Publication is queued. No public website is confirmed yet.';
    case 'published': return saved.receipt.public_path ? 'This reviewed version has a recorded publication.' : 'This version was published but is no longer the current public website.';
    case 'failed': return 'Publication failed. No public website was confirmed for this version.';
    case 'revoked': return 'This publication version is revoked.';
    default: return 'Check the saved publication status.';
  }
}

/** A private editor supplies content; only a complete owner-bound receipt supplies a public link. */
export function PublicationPanel(props: PublicationPanelProps) {
  const latest = useRef(props); latest.current = props;
  const scope = useRef<BuilderScope | null>(null);
  const mounted = useRef(false);
  const version = useRef(0);
  const busyRef = useRef(false);
  const [busy, setBusy] = useState(false);
  const [ready, setReady] = useState(false);
  const [loaded, setLoaded] = useState(false);
  const [held, setHeld] = useState(false);
  const [review, setReview] = useState<PublicationReview | null>(null);
  const [saved, setSaved] = useState<SavedPublicationOperation | null>(null);
  const [receiptChecked, setReceiptChecked] = useState(false);
  const [message, setMessage] = useState('Verifying publication access…');
  const expectedUser = props.expectedOwner?.userId;
  const expectedTenant = props.expectedOwner?.tenantId;

  useEffect(() => {
    mounted.current = true;
    const currentVersion = ++version.current;
    const expected = expectedUser && expectedTenant ? { userId: expectedUser, tenantId: expectedTenant } : null;
    let retired = false;
    scope.current = null; busyRef.current = false;
    setBusy(false); setReady(false); setLoaded(false); setHeld(false); setReview(null); setSaved(null); setReceiptChecked(false);
    const current = () => mounted.current && version.current === currentVersion && !retired;
    const retire = (reason: string) => {
      if (!current()) return;
      retired = true; scope.current = null; busyRef.current = false;
      setBusy(false); setReady(false); setHeld(true); setReview(null); setSaved(null); setReceiptChecked(false); setMessage(reason);
      if (expected) latest.current.onRetired(expected, reason);
    };
    const unsubscribeReadiness = subscribeQueueIdentityReadiness(() => {
      if (!current()) return;
      const verified = !!expected && hasVerifiedOfflineQueueOwner(expected);
      setReady(verified);
      if (scope.current && hasVerifiedOfflineQueueOwner() && !verified) retire('Your publication owner changed. Reopen the editor to verify access.');
    });
    const unsubscribeSession = subscribeOnboardingInvalidation(() => retire('Your publication session changed. Reopen the editor to verify access.'));
    if (!expected) { setMessage('Verify your editor owner before reviewing a public version.'); }
    else void (async () => {
      try {
        const opened = await openBuilderScope();
        if (!current()) return;
        if (!sameOwner(opened.owner, expected)) { retire('Your publication owner changed. Reopen the editor to verify access.'); return; }
        scope.current = opened;
        const stored = await readSavedPublicationOperation(opened, props.channel);
        if (!current()) return;
        setSaved(stored); setLoaded(true); setReady(hasVerifiedOfflineQueueOwner(expected));
        setMessage(stored?.receipt?.status === 'published' ? 'Check publication status to verify the saved public link.' : operationStatus(stored));
      } catch (error) {
        if (current()) { setHeld(true); setMessage(error instanceof Error ? error.message : 'Publication access could not be verified.'); }
      }
    })();
    return () => { mounted.current = false; ++version.current; scope.current = null; unsubscribeReadiness(); unsubscribeSession(); };
  }, [props.channel, expectedUser, expectedTenant]);

  const run = async (work: (ownerScope: BuilderScope, current: () => boolean) => Promise<void>) => {
    const ownerScope = scope.current;
    if (busyRef.current || !ownerScope || !builderScopeActive(ownerScope) || !hasVerifiedOfflineQueueOwner(ownerScope.owner)) return;
    const currentVersion = version.current;
    const current = () => mounted.current && version.current === currentVersion && scope.current === ownerScope && builderScopeActive(ownerScope)
      && !!latest.current.expectedOwner && sameOwner(latest.current.expectedOwner, ownerScope.owner);
    busyRef.current = true; setBusy(true);
    try { await work(ownerScope, current); }
    catch (error) {
      if (current()) {
        setReview(null); setReceiptChecked(false);
        try { const stored = await readSavedPublicationOperation(ownerScope, props.channel); if (current()) setSaved(stored); }
        catch { if (current()) setHeld(true); }
        if (current()) setMessage(error instanceof Error ? error.message : 'The publication outcome could not be confirmed. Check its saved status.');
      }
    } finally { if (current()) { busyRef.current = false; setBusy(false); } }
  };
  const prepare = () => void run(async (ownerScope, current) => {
    if (!latest.current.isEditorCurrent()) throw new Error('Your private editor changed. Reopen it and review again.');
    const approved = await preparePublicationReview(ownerScope, props.channel, latest.current.getSnapshot());
    if (!current()) return;
    if (!latest.current.isEditorCurrent()) throw new Error('Your private editor changed. Reopen it and review again.');
    setReview(approved); setMessage('Review every field before making this version public.');
  });
  const submit = () => void run(async (ownerScope, current) => {
    if (!review) return;
    if (!latest.current.isEditorCurrent()) throw new Error('Your private editor changed. Reopen it and review again.');
    const now = await prepareSiteSnapshot(latest.current.getSnapshot());
    if (!current()) return;
    if (now.snapshot_sha256 !== review.snapshot_sha256 || !latest.current.isEditorCurrent()) throw new Error('Your private draft changed. Review the public version again.');
    const result = await submitPublicationReview(ownerScope, props.channel, review, current, () => latest.current.isEditorCurrent() && canonicalize(latest.current.getSnapshot()) === now.canonical);
    if (!current()) return;
    setReview(null); setSaved(result); setReceiptChecked(true); setMessage(operationStatus(result));
  });
  const refresh = () => void run(async (ownerScope, current) => {
    const result = await refreshPublicationOperation(ownerScope, props.channel, current);
    if (current()) { setReview(null); setSaved(result); setReceiptChecked(true); setMessage(operationStatus(result)); }
  });
  const revoke = () => void run(async (ownerScope, current) => {
    const result = await revokePublication(ownerScope, props.channel, current);
    if (current()) { setReview(null); setSaved(result); setReceiptChecked(true); setMessage(operationStatus(result)); }
  });
  const blocked = saved && (['publish_unknown', 'revoke_unknown'].includes(saved.phase) || ['pending', 'processing'].includes(saved.receipt?.status ?? ''));
  const canUse = ready && loaded && !held;
  const path = canUse && receiptChecked ? publishedSitePath(saved) : null;
  return <section aria-label="Website publication" className="space-y-3 rounded-xl border border-gray-200 p-4 dark:border-white/10">
    <h2 className="font-semibold">Public website</h2>
    <p role="status">{ready || held || !loaded ? message : 'Publication access is temporarily unverified. Wait for your current session to be checked.'}</p>
    <button type="button" disabled={!canUse || busy || !!blocked} onClick={prepare}>Review public version</button>
    {canUse && review && <div className="space-y-3">
      <p>Anyone with the link can read these reviewed pages. Only this snapshot will be published; later private edits need another review.</p>
      <pre aria-label="Public website snapshot" className="max-h-96 overflow-auto whitespace-pre-wrap break-words">{JSON.stringify(review.snapshot, null, 2)}</pre>
      <button type="button" disabled={busy} onClick={submit}>Publish reviewed version</button>
      <button type="button" disabled={busy} onClick={() => { setReview(null); setMessage(operationStatus(saved)); }}>Cancel publication review</button>
    </div>}
    {canUse && saved && saved.phase !== 'rejected' && <button type="button" disabled={busy} onClick={refresh}>Check publication status</button>}
    {path && <a href={path} target="_blank" rel="noopener noreferrer">Open published website</a>}
    {canUse && saved?.phase === 'acknowledged' && saved.receipt && saved.receipt.status !== 'revoked' && <button type="button" disabled={busy} onClick={revoke}>Revoke publication version</button>}
  </section>;
}
