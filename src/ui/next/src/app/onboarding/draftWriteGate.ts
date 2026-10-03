import type { QueueOwner } from '@/lib/sync/queueIdentity';
import { readDraftAcknowledgement } from './contracts';

const live = new Set<string>();
const admitted = new Map<string, number>();
const listeners = new Set<() => void>();
const unavailable = 'Draft saving is unavailable because this browser cannot coordinate safe saves. Your local edits remain held.';
const uncertain = 'A previous draft save could not be confirmed. Local edits remain held for reconciliation.';
export function onboardingWriteFenceKey(owner: QueueOwner): string {
  return 'omnisolo_onboarding_owned_v1:' + encodeURIComponent(JSON.stringify([owner.userId, owner.tenantId])) + ':write-fence';
}
export function onboardingWriteVersion(owner: QueueOwner): string | null {
  return localStorage.getItem(onboardingWriteFenceKey(owner).replace(/write-fence$/, 'write-version'));
}
export function subscribeOnboardingWriteState(listener: () => void) { listeners.add(listener); return () => { listeners.delete(listener); }; }
function notify() { for (const listener of listeners) listener(); }
if (typeof window !== 'undefined') window.addEventListener('storage', event => {
  if (event.key?.startsWith('omnisolo_onboarding_owned_v1:') && /:(write-version|write-fence)$/.test(event.key)) notify();
});
export function hasOnboardingWriteFence(owner: QueueOwner): boolean {
  try { return localStorage.getItem(onboardingWriteFenceKey(owner)) !== null; } catch { return true; }
}
/** Includes this document's lock waiters, not crash-surviving uncertain fences. */
export function onboardingDraftWritesBusy(owner: QueueOwner | null): boolean {
  return !!owner && (admitted.get(onboardingWriteFenceKey(owner)) ?? 0) > 0;
}
export function onboardingDraftWriteProblem(owner: QueueOwner | null): string | null {
  if (!owner) return null;
  if (typeof navigator === 'undefined' || !navigator.locks?.request) return unavailable;
  return hasOnboardingWriteFence(owner) && !live.has(onboardingWriteFenceKey(owner)) ? uncertain : null;
}
/** The origin Web Lock spans dispatch and the complete acknowledgement body, including across tabs.
 * A durable fence survives crashes/lost replies. No later write guesses that an unknown write stopped. */
export async function serializeOnboardingDraftWrite(owner: QueueOwner, current: () => boolean, send: (dispatched: () => void) => Promise<Response>, acknowledged?: (version: string) => void): Promise<Response> {
  if (typeof navigator === 'undefined' || !navigator.locks?.request) throw new Error(unavailable);
  const key = onboardingWriteFenceKey(owner);
  admitted.set(key, (admitted.get(key) ?? 0) + 1); notify();
  try {
    return await navigator.locks.request(key, { mode: 'exclusive' }, async () => {
      if (!current()) throw new Error('Your session changed. This draft was not sent.');
      if (hasOnboardingWriteFence(owner)) throw new Error(uncertain);
      const marker = JSON.stringify({ operation: crypto.randomUUID(), status: 'inflight' });
      try { localStorage.setItem(key, marker); } catch { throw new Error('This device could not record a safe draft save. Your local edits remain held.'); }
      const release = () => {
        if (localStorage.getItem(key) !== marker) throw new Error(uncertain);
        localStorage.removeItem(key);
      };
      let dispatched = false;
      live.add(key); notify();
      try {
        const response = await send(() => { dispatched = true; });
        // Explicit client/authorization rejections cannot acknowledge the draft, but permit a later corrected request.
        if (response.status >= 400 && response.status < 500) { release(); return response; }
        await readDraftAcknowledgement(typeof response.clone === 'function' ? response.clone() : response);
        localStorage.setItem(key.replace(/write-fence$/, 'write-version'), marker);
        release();
        acknowledged?.(marker);
        return response;
      } catch (cause) {
        if (!dispatched) release();
        throw cause;
      } finally { live.delete(key); notify(); }
    });
  } finally {
    const remaining = (admitted.get(key) ?? 1) - 1;
    if (remaining) admitted.set(key, remaining); else admitted.delete(key);
    notify();
  }
}
