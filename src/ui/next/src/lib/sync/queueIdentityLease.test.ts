import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import * as identity from './queueIdentity';
beforeEach(() => { localStorage.clear(); identity.invalidateQueueOwner(); });
afterEach(() => { vi.useRealTimers(); vi.unstubAllGlobals(); });
it('exposes only a copied verified lease and cannot extend authority through the snapshot', async () => {
  expect(identity).toHaveProperty('currentVerifiedQueueLease', expect.any(Function));
  const expiresAt = Date.now() + 1000;
  localStorage.setItem(identity.QUEUE_IDENTITY_EPOCH_KEY, 'lease-epoch');
  vi.stubGlobal('fetch', vi.fn(async () => Response.json({ userId: 'lease-owner', tenantId: 'lease-tenant', expiresAt })));
  await identity.readQueueOwner();
  const snapshot = identity.currentVerifiedQueueLease()!;
  expect(snapshot).toEqual({ owner: { userId: 'lease-owner', tenantId: 'lease-tenant' }, expiresAt, storageEpoch: 'lease-epoch' });
  snapshot.owner.userId = 'forged'; snapshot.expiresAt = Number.MAX_SAFE_INTEGER;
  expect(identity.currentVerifiedQueueOwner()).toEqual({ userId: 'lease-owner', tenantId: 'lease-tenant' });
  expect(identity.currentVerifiedQueueLease()!.expiresAt).toBe(expiresAt);
  vi.useFakeTimers(); vi.setSystemTime(expiresAt + 1);
  expect(identity.currentVerifiedQueueLease()).toBeNull(); expect(identity.hasVerifiedOfflineQueueOwner()).toBe(false);
});
it('distinguishes pending revalidation from expiry without returning a usable lease while pending', async () => {
  expect(identity).toHaveProperty('hasPendingQueueOwnerVerification', expect.any(Function));
  let finish!: (response: Response) => void;
  vi.stubGlobal('fetch', vi.fn(() => new Promise<Response>(resolve => { finish = resolve; })));
  const pending = identity.readQueueOwner();
  expect(identity.hasPendingQueueOwnerVerification()).toBe(true); expect(identity.currentVerifiedQueueLease()).toBeNull();
  finish(Response.json({ userId: 'lease-owner', tenantId: 'lease-tenant', expiresAt: Date.now() + 1000 })); await pending;
  expect(identity.hasPendingQueueOwnerVerification()).toBe(false); expect(identity.currentVerifiedQueueLease()).not.toBeNull();
  identity.invalidateQueueOwner();
  expect(identity.hasPendingQueueOwnerVerification()).toBe(false); expect(identity.currentVerifiedQueueLease()).toBeNull();
});
