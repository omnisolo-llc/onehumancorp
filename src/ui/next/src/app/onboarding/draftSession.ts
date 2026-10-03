import { serializeOnboardingDraftWrite, onboardingWriteVersion } from './draftWriteGate';
import { QUEUE_IDENTITY_EPOCH_KEY, readQueueOwner, sameOwner, type QueueOwner } from '@/lib/sync/queueIdentity';
import { readPublicationResponse } from '../builder/publicationContracts';

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
/** Read only the maintained business snapshots through the same sealed owner boundary. */
function workflowReadDestination(url: string): boolean {
  const uuid = '[a-f0-9]{8}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{12}';
  if (new RegExp(`^/api/v1/agents/workflows/(?:by-request/)?${uuid}$`).test(url)) return !url.endsWith('00000000-0000-0000-0000-000000000000');
  if (!url.startsWith('/api/v1/agents/workflows?') || url.length > 160 || url.includes('#')) return false;
  const parsed = new URL(url, 'https://owned.invalid');
  const keys = [...parsed.searchParams.keys()];
  if (keys.some(key => !['limit', 'before'].includes(key)) || new Set(keys).size !== keys.length) return false;
  const limit = parsed.searchParams.get('limit'), before = parsed.searchParams.get('before');
  return (limit === null || /^(?:[1-9]|1[0-9]|20)$/.test(limit)) &&
    (before === null || new RegExp(`^[0-9]{1,12}:${uuid}$`).test(before) && !before.endsWith('00000000-0000-0000-0000-000000000000'));
}
export async function fetchForOwnedBusinessRead(url: string, expected: DraftOwner | null): Promise<Response> {
  if (!['/api/v1/location/dashboard', '/api/v1/billing/my-plan', '/api/v1/pos/orders', '/api/v1/pos/inventory', '/api/v1/agents/execution-policy', '/api/v1/agents/workflows', '/api/v1/agents/approvals', '/api/v1/agents/approvals/activity', '/api/v1/walkthrough/store-setup'].includes(url) && !workflowReadDestination(url)) throw new Error('Invalid business read destination');
  const intended = expected ? { ...expected } : null;
  if (!intended || !owner || !sameOwner(owner, intended)) throw new Error('Your session changed. Please reopen this view.');
  return authenticatedOnboardingFetch(url, { method: 'GET' }, intended, epoch);
}
/** Existing business mutations share session authority, not draft-write queuing. */
export async function fetchForOwnedBusinessAction(url: string, options: RequestInit, expected: DraftOwner | null, onDispatch?: () => void): Promise<Response> {
  if (!['/api/v1/agent/draft-escalation', '/api/v1/location/escalate', '/api/v1/billing/create-checkout-session', '/api/v1/billing/create-billing-portal-session', '/api/v1/booking/services', '/api/v1/builder/generate', '/api/v1/builder/brand_toolbox/generate', '/api/v1/builder/geo_score', '/api/v1/builder/auto_seo', '/api/v1/builder/publish_draft', '/api/v1/agents/hire'].includes(url) && !(workflowReadDestination(url.replace(/\/cancel$/, '')) && /^\/api\/v1\/agents\/workflows\/[a-f0-9-]{36}\/cancel$/.test(url))) throw new Error('Invalid business action destination');
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
/** Publication status and version-specific revocation share the sealed owner boundary. */
export async function fetchForOwnedPublication(url: string, options: RequestInit, expected: DraftOwner | null, onDispatch?: () => void): Promise<Response> {
  const method = (options.method ?? 'GET').toUpperCase();
  const root = '/api/v1/builder/publications';
  const uuid = '[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}';
  if (!(url === root && method === 'POST')
    && !(new RegExp('^' + root + '/operations/' + uuid + '$').test(url) && method === 'GET')
    && !(new RegExp('^' + root + '/' + uuid + '$').test(url) && method === 'DELETE')) throw new Error('Invalid publication destination or method');
  const intended = expected ? { ...expected } : null;
  if (!intended || !owner || !sameOwner(owner, intended)) throw new Error('Your session changed. Please reopen this view.');
  const before = epoch;
  return authenticatedOnboardingFetch(url, { ...options, method }, intended, before, () => {
    const marker: unknown = onDispatch?.();
    if (marker && typeof (marker as PromiseLike<unknown>).then === 'function') {
      void Promise.resolve(marker).catch(() => undefined);
      throw new Error('Dispatch markers must be saved synchronously');
    }
    if (before !== epoch || !owner || !sameOwner(owner, intended)) throw new Error('Your session changed. This action was not sent.');
  }, 'publication');
}
export async function fetchForOwnedDefinition(url: string, options: RequestInit, expected: DraftOwner | null, onDispatch?: () => void): Promise<Response> {
  const method = (options.method ?? 'GET').toUpperCase();
  const parsed = new URL(url, 'https://owned.invalid');
  const root = '/api/v1/agents/definitions';
  const uuid = '[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}';
  if (!url.startsWith('/') || url.startsWith('//') || parsed.origin !== 'https://owned.invalid' || parsed.hash
    || Array.from(url).some(character => character.charCodeAt(0) <= 32 || character === '\\')) throw new Error('Invalid agent definition destination');
  const catalogue = parsed.pathname === root && method === 'GET';
  const publication = parsed.pathname === root && method === 'POST';
  const installation = new RegExp('^' + root + '/' + uuid + '/install$').test(parsed.pathname) && method === 'POST';
  const recovery = new RegExp('^' + root + '/operations/' + uuid + '$').test(parsed.pathname) && method === 'GET';
  if ((!catalogue && !publication && !installation && !recovery) || (!catalogue && parsed.search)) throw new Error('Invalid agent definition operation');
  if (catalogue) {
    for (const [key, value] of parsed.searchParams) {
      if (!['q', 'cursor', 'installation_cursor', 'limit'].includes(key) || parsed.searchParams.getAll(key).length !== 1
        || (key === 'q' && (Array.from(value).length > 256 || value.includes('\0')))
        || (key.includes('cursor') && (!value || value.length > 2048))
        || (key === 'limit' && (!/^\d+$/.test(value) || Number(value) < 1 || Number(value) > 100))) throw new Error('Invalid agent catalogue query');
    }
  }
  const intended = expected ? { ...expected } : null;
  if (!intended || !owner || !sameOwner(owner, intended)) throw new Error('Your session changed. Please reopen this view.');
  const before = epoch;
  return authenticatedOnboardingFetch(url, { ...options, method }, intended, before, () => {
    const marker: unknown = onDispatch?.();
    if (marker && typeof (marker as PromiseLike<unknown>).then === 'function') {
      void Promise.resolve(marker).catch(() => undefined);
      throw new Error('Dispatch markers must be saved synchronously');
    }
    if (before !== epoch || !owner || !sameOwner(owner, intended)) throw new Error('Your session changed. This action was not sent.');
  }, 'definition');
}
async function authenticatedOnboardingFetch(url: string, options: RequestInit, expected: DraftOwner, before: number, dispatched?: () => void, permission: false | 'definition' | 'publication' = false): Promise<Response> {
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
  const response = await fetch(url, { ...options, headers, credentials: 'same-origin', cache: 'no-store', redirect: 'error' });
  if (before !== epoch || !owner || !sameOwner(owner, expected)) throw new Error('Your session changed. This reply was not applied.');
  if (response.status === 409 || (permission && response.status === 403)) {
    const problem = await (permission === 'publication'
      ? readPublicationResponse(response.clone(), () => {
          if (before !== epoch || !owner || !sameOwner(owner, expected)) throw new Error('Your session changed. This reply was not applied.');
        })
      : response.clone().json()).catch(() => null) as Record<string, unknown> | null;
    if (before !== epoch || !owner || !sameOwner(owner, expected)) throw new Error('Your session changed. This reply was not applied.');
    const reason = problem?.error ?? (permission === 'definition' ? problem?.reason : undefined);
    if (reason === 'session_identity_changed' || reason === 'queued owner does not match the current session') invalidateOnboardingSession(false);
    if (response.status === 403 && permission === 'definition' && problem?.success === false && problem?.reason === 'owner_or_admin_required') return response;
    if (response.status === 403 && permission === 'publication' && problem?.schema_version === 1
      && problem.error === 'publication_forbidden' && problem.effect === 'none' && typeof problem.message === 'string' && problem.message.length <= 2000
      && Object.keys(problem).length === 4 && Object.keys(problem).every(key => ['schema_version', 'error', 'effect', 'message'].includes(key))) return response;
  }
  if (response.status === 401 || response.status === 403) invalidateOnboardingSession(false);
  return response;
}
