'use client';

import { useEffect, useState } from 'react';
import { useClipboardFeedback } from '@/hooks/useClipboardFeedback';
import { hasVerifiedOfflineQueueOwner, notifyQueueIdentityChange, readQueueOwner, sameOwner, subscribeQueueIdentityReadiness } from '@/lib/sync/queueIdentity';
import type { useCloudInvitation } from '../referrals/useCloudInvitation';

type Invitation = ReturnType<typeof useCloudInvitation>;
type RecordedOrders = { user_id: string; tenant_id: string; recorded_orders: number; highest_threshold: number | null; observed_at: string };
function readRecordedOrders(value: unknown, userId: string, tenantId: string): RecordedOrders {
  if (!value || typeof value !== 'object' || Array.isArray(value)) throw new Error('Recorded orders unavailable');
  const data = value as Record<string, unknown>;
  if (data.success !== true || data.error != null || data.metric !== 'recorded_orders' || data.included_statuses !== 'all_recorded_statuses'
    || data.user_id !== userId || data.tenant_id !== tenantId || !Number.isSafeInteger(data.recorded_orders) || Number(data.recorded_orders) < 0
    || typeof data.observed_at !== 'string' || !Number.isFinite(Date.parse(data.observed_at))) throw new Error('Recorded orders unavailable');
  const reached = [1,10,50,100,1000].filter(limit => Number(data.recorded_orders) >= limit);
  if (!Array.isArray(data.reached_thresholds) || JSON.stringify(data.reached_thresholds) !== JSON.stringify(reached)
    || data.highest_threshold !== (reached.at(-1) ?? null)) throw new Error('Recorded milestone unavailable');
  return data as RecordedOrders;
}

export function RecordedOrderMilestone({ invitation, showInvitationAction = false }: { invitation: Invitation; showInvitationAction?: boolean }) {
  const userId = invitation.verifiedOwner?.userId;
  const tenantId = invitation.verifiedOwner?.tenantId;
  const [data, setData] = useState<RecordedOrders | null>(null);
  const [phase, setPhase] = useState<'loading' | 'ready' | 'error' | 'held'>('loading');
  const [identityReady, setIdentityReady] = useState(false);
  const [refresh, setRefresh] = useState(0);
  useEffect(() => {
    setData(null); setIdentityReady(false);
    if (!userId || !tenantId) { setPhase('held'); return; }
    setPhase('loading');
    const owner = { userId, tenantId }; const request = new AbortController(); let active = true;
    const retire = () => { active = false; request.abort(); setData(null); setIdentityReady(false); setPhase('held'); };
    const readiness = () => {
      if (!active) return;
      const ready = hasVerifiedOfflineQueueOwner(owner); setIdentityReady(ready);
      if (hasVerifiedOfflineQueueOwner() && !ready) retire();
    };
    const unsubscribe = subscribeQueueIdentityReadiness(readiness);
    void (async () => {
      try {
        const verified = await readQueueOwner();
        if (!active) return;
        if (!sameOwner(owner, verified)) { retire(); return; }
        const headers = new Headers({ 'x-ohc-expected-user': userId, 'x-ohc-expected-tenant': tenantId });
        if (headers.get('x-ohc-expected-user') !== userId || headers.get('x-ohc-expected-tenant') !== tenantId) throw new Error('Recorded owner unavailable');
        const response = await fetch('/api/v1/growth/milestone', { headers, credentials: 'same-origin', cache: 'no-store', redirect: 'error', signal: request.signal });
        if (!active) return;
        if (response.status === 401 || response.status === 403) { retire(); notifyQueueIdentityChange(); return; }
        const value: unknown = await response.json();
        if (!active) return;
        const reason = value && typeof value === 'object' && 'error' in value ? value.error : undefined;
        if (response.status === 409 && (reason === 'session_identity_changed' || reason === 'queued owner does not match the current session')) { retire(); notifyQueueIdentityChange(); return; }
        if (response.status !== 200) throw new Error('Recorded orders unavailable');
        const recorded = readRecordedOrders(value, userId, tenantId); readiness();
        if (!active) return;
        setData(recorded); setPhase('ready');
      } catch { if (active) { setData(null); setPhase('error'); } }
    })();
    return () => { active = false; request.abort(); unsubscribe(); };
  }, [userId, tenantId, refresh]);
  const visible = data && data.user_id === userId && data.tenant_id === tenantId && identityReady ? data : null;
  const confirmedLink = visible && invitation.phase === 'created' ? invitation.link : '';
  const shareText = visible?.highest_threshold && confirmedLink ? `We've recorded ${visible.recorded_orders} orders in OmniSolo. ${confirmedLink}` : '';
  const copy = useClipboardFeedback(shareText);
  const whatsapp = new URL('https://wa.me/'); whatsapp.searchParams.set('text', shareText);
  const twitter = new URL('https://twitter.com/intent/tweet'); twitter.searchParams.set('text', shareText);
  return <section aria-label="Recorded order milestone" className="mt-8 pt-6 border-t border-gray-200 dark:border-gray-800 space-y-3">
    <h3 className="text-xl font-bold">{phase === 'error' ? 'Order milestones unavailable' : 'Recorded order milestones'}</h3>
    <button type="button" className="app-button" disabled={!userId || !tenantId || phase === 'loading'} onClick={() => setRefresh(value => value + 1)}>Refresh recorded orders</button>
    {phase === 'error' ? <p role="status">Recorded order data is unavailable.</p> : !visible ? <p role="status">{phase === 'loading' ? 'Loading recorded orders…' : 'Verify your account to read recorded orders.'}</p> : <>
      <p>{visible.recorded_orders === 0 ? 'No recorded orders yet.' : `${visible.recorded_orders} recorded orders`}</p>
      <p className="text-sm">Count read at <time dateTime={visible.observed_at}>{new Date(visible.observed_at).toLocaleString()}</time>.</p>
      <p className="text-sm">Counts all recorded statuses, including pending and canceled records. This does not verify payment, delivery, revenue or a reward.</p>
      {visible.highest_threshold !== null && <p>Recorded-order milestone: {visible.highest_threshold}</p>}
      {visible.highest_threshold !== null && (shareText ? <>
        <label className="block">Review the aggregate share text<textarea aria-label="Milestone share preview" readOnly value={shareText} className="block w-full border rounded p-2" /></label>
        <p className="text-sm">Opening a share intent does not send a message or confirm that anyone joined.</p>
        <a href={whatsapp.href} target="_blank" rel="noopener noreferrer" className="app-button">Share to WhatsApp</a>
        <a href={twitter.href} target="_blank" rel="noopener noreferrer" className="app-button">Share on X</a>
        <button type="button" className="app-button" disabled={copy.state === 'pending'} onClick={() => void copy.copy(shareText)}>Copy milestone share text</button>
        {copy.message && <p role={copy.state === 'error' ? 'alert' : 'status'} aria-label="Milestone clipboard">{copy.message}</p>}
      </> : <>
        <p>A confirmed invitation for this account is required before a milestone share link is available.</p>
        {showInvitationAction && <button type="button" className="app-button" disabled={invitation.phase !== 'ready'} onClick={() => void invitation.create('pending-invite')}>Create milestone invitation</button>}
        {showInvitationAction && <p role="status">{invitation.message}</p>}
      </>)}
    </>}
  </section>;
}
