export type QueueOwner = { userId: string; tenantId: string };
export const QUEUE_IDENTITY_EPOCH_KEY = 'omnisolo_queue_identity_epoch_v2';
let verified: { owner: QueueOwner; expiresAt: number; storageEpoch: string | null } | undefined;
let generation = 0;
let sequence = 0;
let lastResolved = 0;
let latestResolvedOwner: QueueOwner | undefined;
const readinessListeners = new Set<() => void>();
const pendingVerifications = new Set<number>();
let readinessExpiry: ReturnType<typeof setTimeout> | undefined;
/** This reflects the existing cache policy; it never supplies identity or authorizes a write. */
export function hasVerifiedOfflineQueueOwner(expected?: QueueOwner | null): boolean {
  try { return !!verified && pendingVerifications.size === 0 && verified.expiresAt > Date.now() && verified.storageEpoch === localStorage.getItem(QUEUE_IDENTITY_EPOCH_KEY) && (expected === undefined || (!!expected && sameOwner(verified.owner, expected))); }
  catch { return false; }
}
/** A copied current canonical identity for private UI leases; never reads identity from storage. */
export function currentVerifiedQueueOwner(): QueueOwner | null {
  return hasVerifiedOfflineQueueOwner() && verified ? { ...verified.owner } : null;
}
/** Readonly UI lease evidence; copying it cannot extend or replace verified authority. */
export function currentVerifiedQueueLease(): { owner: QueueOwner; expiresAt: number; storageEpoch: string | null } | null {
  return hasVerifiedOfflineQueueOwner() && verified ? { owner: { ...verified.owner }, expiresAt: verified.expiresAt, storageEpoch: verified.storageEpoch } : null;
}
export function hasPendingQueueOwnerVerification(): boolean { return pendingVerifications.size > 0; }
function publishReadiness(): void {
  clearTimeout(readinessExpiry);
  if (readinessListeners.size && verified && verified.expiresAt > Date.now()) {
    readinessExpiry = setTimeout(publishReadiness, Math.min(verified.expiresAt - Date.now(), 2_147_483_647));
  }
  for (const listener of readinessListeners) listener();
}
export function subscribeQueueIdentityReadiness(listener: () => void): () => void {
  readinessListeners.add(listener); publishReadiness();
  return () => { readinessListeners.delete(listener); if (!readinessListeners.size) clearTimeout(readinessExpiry); };
}
export function sameOwner(a: QueueOwner, b: QueueOwner): boolean { return a.userId === b.userId && a.tenantId === b.tenantId; }
export function invalidateQueueOwner(): void { verified = undefined; latestResolvedOwner = undefined; generation += 1; pendingVerifications.clear(); publishReadiness(); }
/** An opaque invalidation signal only. It never grants or supplies identity. */
export function notifyQueueIdentityChange(): void {
  invalidateQueueOwner();
  try { localStorage.setItem(QUEUE_IDENTITY_EPOCH_KEY, crypto.randomUUID()); } catch { /* Local identity is still invalidated when storage is unavailable. */ }
  if (typeof window !== 'undefined') window.dispatchEvent(new Event('omnisolo_auth_changed'));
}
if (typeof window !== 'undefined') {
  window.addEventListener('storage', event => { if (event.key === null || event.key === QUEUE_IDENTITY_EPOCH_KEY) invalidateQueueOwner(); });
  window.addEventListener('omnisolo_auth_changed', invalidateQueueOwner);
  window.addEventListener('pagehide', invalidateQueueOwner);
}

/** Cancellation retires only this verification; a late transport reply cannot repopulate its lease. */
function withIdentitySignal<T>(pending: Promise<T>, signal?: AbortSignal): Promise<T> {
  if (!signal) return pending;
  return new Promise((resolve, reject) => {
    const abort = () => reject(signal.reason ?? new DOMException('Identity verification interrupted', 'AbortError'));
    if (signal.aborted) abort(); else signal.addEventListener('abort', abort, { once: true });
    pending.then(value => { signal.removeEventListener('abort', abort); resolve(value); }, error => { signal.removeEventListener('abort', abort); reject(error); });
  });
}

/** Browser storage is never identity authority. Offline reuse is same-tab and time-bounded. */
export async function readQueueOwner(signal?: AbortSignal): Promise<QueueOwner> {
  signal?.throwIfAborted();
  if (typeof window === 'undefined') throw new Error('Verified queue identity requires a browser');
  const storageEpoch = localStorage.getItem(QUEUE_IDENTITY_EPOCH_KEY);
  if (!navigator.onLine) {
    if (verified && verified.expiresAt > Date.now() && verified.storageEpoch === storageEpoch) return { ...verified.owner };
    throw new Error('Verified queue identity unavailable while offline');
  }
  const epoch = generation;
  const requestNumber = ++sequence;
  pendingVerifications.add(requestNumber);
  verified = undefined; publishReadiness();
  try {
    const response = await withIdentitySignal(fetch('/api/v1/auth/session-identity', { credentials: 'same-origin', cache: 'no-store', redirect: 'error', ...(signal ? { signal } : {}) }), signal);
    if (response.status !== 200) throw new Error('Verified queue identity unavailable');
    const data = await withIdentitySignal(response.json(), signal) as { userId?: unknown; tenantId?: unknown; expiresAt?: unknown; error?: unknown; success?: unknown };
    signal?.throwIfAborted();
    if (!data || typeof data !== 'object' || Array.isArray(data) || data.error != null || ('success' in data && data.success !== true)) throw new Error('Verified queue identity unavailable');
    if (epoch !== generation || storageEpoch !== localStorage.getItem(QUEUE_IDENTITY_EPOCH_KEY) || typeof data.userId !== 'string' || !data.userId || typeof data.tenantId !== 'string' || !data.tenantId || typeof data.expiresAt !== 'number' || !Number.isSafeInteger(data.expiresAt) || data.expiresAt <= Date.now()) throw new Error('Verified queue identity unavailable');
    const owner = { userId: data.userId, tenantId: data.tenantId };
    if (requestNumber < lastResolved) {
      // A late same-owner read is harmless; a stale different login is not.
      if (!latestResolvedOwner || !sameOwner(latestResolvedOwner, owner)) throw new Error('Verified queue identity superseded');
    } else {
      lastResolved = requestNumber;
      latestResolvedOwner = owner;
      verified = { owner, expiresAt: data.expiresAt, storageEpoch }; publishReadiness();
    }
    return { ...owner };
  } catch (error) {
    if (epoch === generation && requestNumber >= lastResolved) { lastResolved = requestNumber; latestResolvedOwner = undefined; verified = undefined; publishReadiness(); }
    throw error;
  } finally { pendingVerifications.delete(requestNumber); publishReadiness(); }
}
