export type QueueOwner = { userId: string; tenantId: string };
export const QUEUE_IDENTITY_EPOCH_KEY = 'omnisolo_queue_identity_epoch_v2';
let verified: { owner: QueueOwner; expiresAt: number; storageEpoch: string | null } | undefined;
let generation = 0;
let sequence = 0;
let lastResolved = 0;
let latestResolvedOwner: QueueOwner | undefined;
export function sameOwner(a: QueueOwner, b: QueueOwner): boolean { return a.userId === b.userId && a.tenantId === b.tenantId; }
export function invalidateQueueOwner(): void { verified = undefined; latestResolvedOwner = undefined; generation += 1; }
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

/** Browser storage is never identity authority. Offline reuse is same-tab and time-bounded. */
export async function readQueueOwner(): Promise<QueueOwner> {
  if (typeof window === 'undefined') throw new Error('Verified queue identity requires a browser');
  const storageEpoch = localStorage.getItem(QUEUE_IDENTITY_EPOCH_KEY);
  if (!navigator.onLine) {
    if (verified && verified.expiresAt > Date.now() && verified.storageEpoch === storageEpoch) return { ...verified.owner };
    throw new Error('Verified queue identity unavailable while offline');
  }
  const epoch = generation;
  const requestNumber = ++sequence;
  verified = undefined;
  try {
    const response = await fetch('/api/v1/auth/session-identity', { credentials: 'same-origin', cache: 'no-store', redirect: 'error' });
    if (!response.ok) throw new Error('Verified queue identity unavailable');
    const data = await response.json() as { userId?: unknown; tenantId?: unknown; expiresAt?: unknown };
    if (epoch !== generation || storageEpoch !== localStorage.getItem(QUEUE_IDENTITY_EPOCH_KEY) || typeof data.userId !== 'string' || !data.userId || typeof data.tenantId !== 'string' || !data.tenantId || typeof data.expiresAt !== 'number' || data.expiresAt <= Date.now()) throw new Error('Verified queue identity unavailable');
    const owner = { userId: data.userId, tenantId: data.tenantId };
    if (requestNumber < lastResolved) {
      // A late same-owner read is harmless; a stale different login is not.
      if (!latestResolvedOwner || !sameOwner(latestResolvedOwner, owner)) throw new Error('Verified queue identity superseded');
    } else {
      lastResolved = requestNumber;
      latestResolvedOwner = owner;
      verified = { owner, expiresAt: data.expiresAt, storageEpoch };
    }
    return { ...owner };
  } catch (error) {
    if (epoch === generation && requestNumber >= lastResolved) { lastResolved = requestNumber; latestResolvedOwner = undefined; verified = undefined; }
    throw error;
  }
}
