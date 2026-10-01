import { serializeOnboardingDraftWrite, onboardingWriteVersion } from './draftWriteGate';
import { QUEUE_IDENTITY_EPOCH_KEY, readQueueOwner, sameOwner, type QueueOwner } from '@/lib/sync/queueIdentity';

export type DraftOwner = QueueOwner;
const PREFIX = 'omnisolo_onboarding_owned_v1:';
let owner: DraftOwner | null = null;
let epoch = 0;
const listeners = new Set<(restart: boolean) => void>();
export function onboardingOwner(): DraftOwner | null { return owner ? { ...owner } : null; }
export function onboardingSessionEpoch(): number { return epoch; }
export function subscribeOnboardingInvalidation(listener: (restart: boolean) => void): () => void { listeners.add(listener); return () => { listeners.delete(listener); }; }
export function invalidateOnboardingSession(restart = true): void {
  owner = null; epoch += 1;
  for (const listener of listeners) listener(restart);
}
if (typeof window !== 'undefined') {
  window.addEventListener('omnisolo_auth_changed', () => invalidateOnboardingSession());
  window.addEventListener('storage', event => { if (event.key === null || event.key === QUEUE_IDENTITY_EPOCH_KEY) invalidateOnboardingSession(); });
  window.addEventListener('pagehide', () => invalidateOnboardingSession(false));
}
export async function openOnboardingSession(): Promise<DraftOwner> {
  const before = epoch;
  const verified = await readQueueOwner();
  if (before !== epoch) throw new Error('Your session changed. Please reopen setup.');
  if (owner && !sameOwner(owner, verified)) invalidateOnboardingSession();
  owner = { ...verified };
  return { ...verified };
}
export function ownedOnboardingKey(name: string): string | null {
  return owner ? PREFIX + encodeURIComponent(JSON.stringify([owner.userId, owner.tenantId])) + ':' + name : null;
}
export function readOwnedOnboardingItem(name: string): string | null {
  const key = ownedOnboardingKey(name);
  return key ? localStorage.getItem(key) : null;
}
export function writeOwnedOnboardingItem(name: string, value: string): void {
  const key = ownedOnboardingKey(name);
  if (!key) throw new Error('Verify your session before saving a draft.');
  localStorage.setItem(key, value);
}
export function hasHeldOnboardingDraft(): boolean {
  if (localStorage.getItem('onboarding-storage-v4') || localStorage.getItem('onboarding_initial_products') || localStorage.getItem('onboardingState')) return true;
  const active = ownedOnboardingKey('');
  for (let index = 0; index < localStorage.length; index++) {
    const key = localStorage.key(index);
    if (key?.startsWith(PREFIX) && (!active || !key.startsWith(active))) return true;
  }
  return false;
}
export function captureOnboardingRestoreSnapshot() {
  const currentOwner = onboardingOwner();
  if (!currentOwner) throw new Error('Verify your session before restoring a draft.');
  return { owner: currentOwner, epoch, writeVersion: onboardingWriteVersion(currentOwner), draft: readOwnedOnboardingItem('draft'), products: readOwnedOnboardingItem('products') };
}
export function assertOnboardingRestoreSnapshot(snapshot: ReturnType<typeof captureOnboardingRestoreSnapshot>): void {
  if (!owner || !sameOwner(owner, snapshot.owner) || epoch !== snapshot.epoch || onboardingWriteVersion(owner) !== snapshot.writeVersion || readOwnedOnboardingItem('draft') !== snapshot.draft || readOwnedOnboardingItem('products') !== snapshot.products) {
    throw new Error('Your draft changed in another view. Reopen setup to restore the latest local draft.');
  }
}
/** Expected identity prevents races; the authenticated server session remains authority. */
export async function fetchForOnboardingOwner(url: string, options: RequestInit, expected: DraftOwner | null, acknowledged?: (version: string) => void): Promise<Response> {
  if (!['state', 'draft', 'chat', 'intake', 'start', 'start_zero_click', 'launch'].some(path => url === '/api/v1/onboarding/' + path)) throw new Error('Invalid onboarding destination');
  if (!expected || !owner || !sameOwner(owner, expected)) throw new Error('Your session changed. Please reopen setup.');
  const before = epoch;
  if ((options.method ?? 'GET').toUpperCase() === 'POST' && ['/api/v1/onboarding/state', '/api/v1/onboarding/draft'].includes(url)) {
    return serializeOnboardingDraftWrite(expected, () => before === epoch && !!owner && sameOwner(owner, expected), dispatched => authenticatedOnboardingFetch(url, options, expected, before, dispatched), acknowledged);
  }
  return authenticatedOnboardingFetch(url, options, expected, before);
}
/** Existing business mutations share session authority, not draft-write queuing. */
export async function fetchForOwnedBusinessAction(url: string, options: RequestInit, expected: DraftOwner | null, onDispatch?: () => void): Promise<Response> {
  if (!['/api/v1/booking/services', '/api/v1/builder/generate', '/api/v1/builder/publish_draft'].includes(url)) throw new Error('Invalid business action destination');
  if ((options.method ?? 'GET').toUpperCase() !== 'POST') throw new Error('Owned business actions require POST');
  if (!expected || !owner || !sameOwner(owner, expected)) throw new Error('Your session changed. Please reopen setup.');
  const before = epoch;
  return authenticatedOnboardingFetch(url, { ...options, credentials: 'same-origin', cache: 'no-store', redirect: 'error' }, expected, before, () => {
    const result: unknown = onDispatch?.();
    if (result && typeof (result as PromiseLike<unknown>).then === 'function') {
      void Promise.resolve(result).catch(() => undefined);
      throw new Error('Dispatch markers must be saved synchronously');
    }
    if (before !== epoch || !owner || !sameOwner(owner, expected)) throw new Error('Your session changed. This action was not sent.');
  });
}
async function authenticatedOnboardingFetch(url: string, options: RequestInit, expected: DraftOwner, before: number, dispatched?: () => void): Promise<Response> {
  let verified: DraftOwner;
  try { verified = await readQueueOwner(); }
  catch (cause) { if (before === epoch) invalidateOnboardingSession(false); throw cause; }
  if (before !== epoch || !owner || !sameOwner(verified, expected) || !sameOwner(owner, expected)) {
    if (before === epoch) invalidateOnboardingSession();
    throw new Error('Your session changed. Please reopen setup.');
  }
  const headers = new Headers(options.headers);
  headers.set('x-ohc-expected-user', expected.userId);
  headers.set('x-ohc-expected-tenant', expected.tenantId);
  dispatched?.();
  const response = await fetch(url, { ...options, headers });
  if (before !== epoch || !owner || !sameOwner(owner, expected)) throw new Error('Your session changed. This reply was not applied.');
  if (response.status === 409) {
    const problem = await response.clone().json().catch(() => null);
    if (before !== epoch || !owner || !sameOwner(owner, expected)) throw new Error('Your session changed. This reply was not applied.');
    if (problem?.error === 'session_identity_changed' || problem?.error === 'queued owner does not match the current session') invalidateOnboardingSession(false);
  }
  if (response.status === 401 || response.status === 403) invalidateOnboardingSession(false);
  return response;
}
