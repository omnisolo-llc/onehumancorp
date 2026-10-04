import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import { currentVerifiedQueueOwner, hasPendingQueueOwnerVerification, notifyQueueIdentityChange, readQueueOwner } from './queueIdentity';
const owner = { userId: 'owner-a', tenantId: 'tenant-a', expiresAt: Date.now() + 60_000 };
beforeEach(() => { localStorage.clear(); notifyQueueIdentityChange(); });
afterEach(() => { notifyQueueIdentityChange(); vi.unstubAllGlobals(); });

it.each(['headers', 'body'])('aborts a stalled identity %s without leaving pending verification or accepting its late reply', async phase => {
  let finish!: () => void;
  if (phase === 'headers') {
    vi.stubGlobal('fetch', vi.fn(() => new Promise<Response>(resolve => { finish = () => resolve(Response.json(owner)); })));
  } else {
    const body = new ReadableStream({ start(controller) { finish = () => { controller.enqueue(new TextEncoder().encode(JSON.stringify(owner))); controller.close(); }; } });
    vi.stubGlobal('fetch', vi.fn(async () => new Response(body)));
  }
  const controller = new AbortController();
  const pending = readQueueOwner(controller.signal).then(() => 'resolved', () => 'aborted');
  await Promise.resolve(); await Promise.resolve();
  controller.abort();
  const observed = await Promise.race([pending, new Promise(resolve => setTimeout(() => resolve('still pending'), 10))]);
  const verificationStillPending = hasPendingQueueOwnerVerification();
  finish(); await pending; await Promise.resolve();
  expect(observed).toBe('aborted');
  expect(verificationStillPending).toBe(false);
  expect(currentVerifiedQueueOwner()).toBeNull();
});
