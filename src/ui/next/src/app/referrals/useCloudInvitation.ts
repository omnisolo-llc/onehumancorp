import { useCallback, useEffect, useRef, useState } from 'react';
import { hasVerifiedOfflineQueueOwner, subscribeQueueIdentityReadiness } from '../../lib/sync/queueIdentity';

type Owner = { userId: string; tenantId: string; expiresAt: number };
type Marker = { version: 1; owner: Pick<Owner, 'userId' | 'tenantId'>; operation: string; state: 'pending' | 'created' };
type View = { phase: 'verifying' | 'ready' | 'requesting' | 'created' | 'held'; message: string; link: string };
const EPOCH_KEY = 'omnisolo_queue_identity_epoch_v2';
// This is the same metadata/lock namespace as the maintained dashboard bridge.
const keyFor = (owner: Owner) => 'omnisolo_invite_creation_v1:' + encodeURIComponent(JSON.stringify([owner.userId, owner.tenantId]));
// Readiness is only a contradiction check, never a replacement identity or write grant.
const canonicalOwnerChanged = (owner: Owner) => hasVerifiedOfflineQueueOwner() && !hasVerifiedOfflineQueueOwner(owner);
const sameOwner = (a: Pick<Owner, 'userId' | 'tenantId'>, b: Pick<Owner, 'userId' | 'tenantId'>) => a.userId === b.userId && a.tenantId === b.tenantId;
function markerFor(key: string, owner: Owner): Marker | null {
  const bytes = localStorage.getItem(key);
  if (bytes === null) return null;
  const marker = JSON.parse(bytes);
  if (marker?.version !== 1 || !marker.owner || !sameOwner(marker.owner, owner) || typeof marker.operation !== 'string' || !marker.operation || !['pending', 'created'].includes(marker.state)) throw new Error('Invitation history is unavailable');
  return marker;
}
async function identity(): Promise<Owner> {
  const response = await fetch('/api/v1/auth/session-identity', { credentials: 'same-origin', cache: 'no-store', redirect: 'error' });
  const value = await response.json();
  if (response.status !== 200 || value?.error != null || value?.success === false || typeof value?.userId !== 'string' || !value.userId || typeof value?.tenantId !== 'string' || !value.tenantId || !Number.isFinite(value.expiresAt) || value.expiresAt <= Date.now()) throw new Error('Verified invitation identity unavailable');
  return value;
}
function confirmedLink(value: unknown): string | null {
  if (!value || typeof value !== 'object' || Array.isArray(value)) return null;
  const body = value as Record<string, unknown>;
  if (body.error != null || body.success === false || typeof body.invite_link !== 'string') return null;
  try {
    const url = new URL(body.invite_link);
    if (url.protocol !== 'https:' || !['omnisolo.co', 'cloud.omnisolo.co'].includes(url.hostname) || url.port || url.username || url.password || url.search || url.hash || !/^\/invite\/[^/]+$/.test(url.pathname) || /\/(fallback|default)$/.test(url.pathname)) return null;
    return body.invite_link;
  } catch { return null; }
}

export function useCloudInvitation(onRetire: () => void) {
  const [view, setView] = useState<View>({ phase: 'verifying', message: 'Verifying invitation access…', link: '' });
  const owner = useRef<Owner | null>(null);
  const epoch = useRef(0);
  const active = useRef(false);
  const busy = useRef(false);
  const expires = useRef<ReturnType<typeof setTimeout> | undefined>(undefined);
  const phase = useRef<View['phase']>('verifying');
  const show = useCallback((next: View) => { phase.current = next.phase; if (active.current) setView(next); }, []);
  const retire = useCallback(() => {
    epoch.current += 1; owner.current = null; busy.current = false; clearTimeout(expires.current);
    onRetire();
    show({ phase: 'held', message: 'Your session changed. Reload to verify invitation access.', link: '' });
  }, [show, onRetire]);
  useEffect(() => {
    active.current = true;
    const expected = ++epoch.current;
    phase.current = 'verifying';
    const changed = (event: StorageEvent) => {
      if (event.key === null || event.key === EPOCH_KEY) retire();
      else if (owner.current && event.key === keyFor(owner.current) && !busy.current && phase.current === 'ready') {
        show({ phase: 'held', message: 'An invitation request changed in another view. Review existing invitations before retrying.', link: '' });
      }
    };
    const unsubscribeReadiness = subscribeQueueIdentityReadiness(() => {
      if (active.current && owner.current && canonicalOwnerChanged(owner.current)) retire();
    });
    window.addEventListener('omnisolo_auth_changed', retire);
    window.addEventListener('storage', changed);
    window.addEventListener('pagehide', retire);
    void (async () => {
      try {
        const verified = await identity();
        if (!active.current || expected !== epoch.current) return;
        if (canonicalOwnerChanged(verified)) { retire(); return; }
        owner.current = verified;
        expires.current = setTimeout(retire, Math.min(verified.expiresAt - Date.now(), 2_147_483_647));
        if (!navigator.locks?.request) throw new Error('Coordinated invitation requests unavailable');
        const marker = markerFor(keyFor(verified), verified);
        if (marker) {
          show({ phase: 'held', message: marker.state === 'created' ? 'An invitation was already created in this browser. Review existing invitations before creating another.' : 'A previous invitation request is unconfirmed. Review existing invitations before trying again.', link: '' });
        } else show({ phase: 'ready', message: 'Create one invitation link for this verified account.', link: '' });
      } catch {
        if (active.current && expected === epoch.current) show({ phase: 'held', message: 'Invitation access or local request history is unavailable. No invitation was requested.', link: '' });
      }
    })();
    return () => {
      active.current = false; epoch.current += 1; owner.current = null; clearTimeout(expires.current); unsubscribeReadiness();
      window.removeEventListener('omnisolo_auth_changed', retire); window.removeEventListener('storage', changed); window.removeEventListener('pagehide', retire);
    };
  }, [retire, show]);

  const create = async (invitee: string) => {
    if (!active.current || phase.current !== 'ready' || busy.current || !owner.current || owner.current.expiresAt <= Date.now()) return;
    if (canonicalOwnerChanged(owner.current)) { retire(); return; }
    const intended = { ...owner.current }; const expected = epoch.current; const key = keyFor(intended); const inviteeId = invitee.trim();
    if (!inviteeId) { show({ phase: 'ready', message: 'Enter the team member or client email before creating an invitation.', link: '' }); return; }
    busy.current = true; show({ phase: 'requesting', message: 'Requesting an invitation…', link: '' });
    let dispatched = false;
    try {
      await navigator.locks.request(key, { mode: 'exclusive', ifAvailable: true }, async lock => {
        if (!lock) throw new Error('Another invitation request is active');
        let verified: Owner;
        try { verified = await identity(); }
        catch (cause) {
          if (active.current && expected === epoch.current) retire();
          throw cause;
        }
        if (!active.current || expected !== epoch.current) return;
        if (!sameOwner(verified, intended) || canonicalOwnerChanged(intended)) { retire(); return; }
        if (markerFor(key, intended)) throw new Error('Previous invitation outcome must be reviewed');
        const headers = new Headers({ 'content-type': 'application/json', 'x-ohc-expected-user': intended.userId, 'x-ohc-expected-tenant': intended.tenantId });
        if (headers.get('x-ohc-expected-user') !== intended.userId || headers.get('x-ohc-expected-tenant') !== intended.tenantId) throw new Error('Identity cannot be bound');
        const marker: Marker = { version: 1, owner: { userId: intended.userId, tenantId: intended.tenantId }, operation: crypto.randomUUID(), state: 'pending' };
        localStorage.setItem(key, JSON.stringify(marker));
        if (!active.current || expected !== epoch.current) return;
        dispatched = true;
        const response = await fetch('/api/v1/growth/cloud-bridge/invite', { method: 'POST', headers, body: JSON.stringify({ invitee_id: inviteeId }), credentials: 'same-origin', cache: 'no-store', redirect: 'error' });
        const body = await response.json(); const link = response.status === 200 ? confirmedLink(body) : null;
        if (!link) throw new Error('Invitation result unconfirmed');
        const current = markerFor(key, intended);
        if (current?.operation !== marker.operation || current.state !== 'pending') throw new Error('Invitation history changed');
        localStorage.setItem(key, JSON.stringify({ ...marker, state: 'created' }));
        if (!active.current || expected !== epoch.current || !owner.current || !sameOwner(owner.current, intended)) return;
        show({ phase: 'created', message: `Cloud Invite generated: ${link}. Sending or joining is not verified here.`, link });
      });
    } catch {
      if (active.current && expected === epoch.current) show({ phase: 'held', message: dispatched ? 'The invitation result could not be confirmed or saved. Review existing invitations before trying again.' : 'Invitation creation is held. Verify your session and review any earlier request before trying again.', link: '' });
    } finally { if (expected === epoch.current) busy.current = false; }
  };
  // A copy lets read-only consumers share this verified lifetime without
  // changing the authority retained for invitation creation.
  return { ...view, create, verifiedOwner: owner.current ? { ...owner.current } : null };
}
