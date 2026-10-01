import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import { invalidateQueueOwner, readQueueOwner } from './queueIdentity';
beforeEach(() => { invalidateQueueOwner(); vi.spyOn(navigator, 'onLine', 'get').mockReturnValue(true); });
afterEach(() => { vi.restoreAllMocks(); vi.unstubAllGlobals(); });
it('uses only the verified session endpoint and permits a verified same-tab offline identity', async () => {
  localStorage.setItem('userId', 'forged');
  const fetchMock = vi.fn().mockResolvedValue(Response.json({ userId: 'a', tenantId: 't', expiresAt: Date.now() + 100000 }));
  vi.stubGlobal('fetch', fetchMock);
  expect(await readQueueOwner()).toEqual({ userId: 'a', tenantId: 't' });
  expect(fetchMock.mock.calls[0][0]).toBe('/api/v1/auth/session-identity');
  vi.spyOn(navigator, 'onLine', 'get').mockReturnValue(false);
  expect(await readQueueOwner()).toEqual({ userId: 'a', tenantId: 't' });
  invalidateQueueOwner();
  await expect(readQueueOwner()).rejects.toThrow('identity');
});
it('does not reuse an old verified owner after failed online verification or a new login', async () => {
  vi.stubGlobal('fetch', vi.fn().mockResolvedValueOnce(Response.json({ userId: 'a', tenantId: 't', expiresAt: Date.now() + 100000 })).mockResolvedValueOnce(new Response('{}', { status: 401 })).mockResolvedValueOnce(Response.json({ userId: 'b', tenantId: 'u', expiresAt: Date.now() + 100000 })));
  await readQueueOwner();
  await expect(readQueueOwner()).rejects.toThrow('identity');
  vi.spyOn(navigator, 'onLine', 'get').mockReturnValue(false);
  await expect(readQueueOwner()).rejects.toThrow('identity');
  vi.spyOn(navigator, 'onLine', 'get').mockReturnValue(true);
  expect(await readQueueOwner()).toEqual({ userId: 'b', tenantId: 'u' });
});
it('invalidates across tabs and does not trust expired identity data', async () => {
  vi.stubGlobal('fetch', vi.fn().mockResolvedValue(Response.json({ userId: 'a', tenantId: 't', expiresAt: Date.now() - 1 })));
  await expect(readQueueOwner()).rejects.toThrow('identity');
});
it('cannot replace a newer verified identity with a delayed old session response', async () => {
  let oldResolve!: (response: Response) => void;
  vi.stubGlobal('fetch', vi.fn().mockReturnValueOnce(new Promise(resolve => { oldResolve = resolve; })).mockResolvedValueOnce(Response.json({ userId: 'new', tenantId: 't', expiresAt: Date.now() + 100000 })));
  const oldRequest = readQueueOwner();
  expect(await readQueueOwner()).toEqual({ userId: 'new', tenantId: 't' });
  oldResolve(Response.json({ userId: 'old', tenantId: 't', expiresAt: Date.now() + 100000 }));
  await expect(oldRequest).rejects.toThrow('identity');
  vi.spyOn(navigator, 'onLine', 'get').mockReturnValue(false);
  expect(await readQueueOwner()).toEqual({ userId: 'new', tenantId: 't' });
});
it('allows simultaneous same-owner verifications to finish without invalidating one another', async () => {
  const replies: Array<(response: Response) => void> = [];
  vi.stubGlobal('fetch', vi.fn(() => new Promise<Response>(resolve => { replies.push(resolve); })));
  const first = readQueueOwner(); const second = readQueueOwner();
  replies[0](Response.json({ userId: 'a', tenantId: 't', expiresAt: Date.now() + 100000 }));
  expect(await first).toEqual({ userId: 'a', tenantId: 't' });
  replies[1](Response.json({ userId: 'a', tenantId: 't', expiresAt: Date.now() + 100000 }));
  expect(await second).toEqual({ userId: 'a', tenantId: 't' });
});
it('does not reject an older same-owner response when a third verification is still pending', async () => {
  const replies: Array<(response: Response) => void> = [];
  vi.stubGlobal('fetch', vi.fn(() => new Promise<Response>(resolve => { replies.push(resolve); })));
  const first = readQueueOwner(); const second = readQueueOwner();
  const answer = () => Response.json({ userId: 'a', tenantId: 't', expiresAt: Date.now() + 100000 });
  replies[1](answer()); await second;
  const third = readQueueOwner();
  replies[0](answer()); expect(await first).toEqual({ userId: 'a', tenantId: 't' });
  replies[2](answer()); expect(await third).toEqual({ userId: 'a', tenantId: 't' });
});
it('invalidates offline reuse when another tab changes the opaque auth epoch even before event delivery', async () => {
  vi.stubGlobal('fetch', vi.fn().mockResolvedValue(Response.json({ userId: 'a', tenantId: 't', expiresAt: Date.now() + 100000 })));
  await readQueueOwner();
  localStorage.setItem('omnisolo_queue_identity_epoch_v2', 'new-login');
  vi.spyOn(navigator, 'onLine', 'get').mockReturnValue(false);
  await expect(readQueueOwner()).rejects.toThrow('identity');
});
it('rejects a verification that finishes after a logout epoch event', async () => {
  let resolve!: (response: Response) => void;
  vi.stubGlobal('fetch', vi.fn(() => new Promise<Response>(done => { resolve = done; })));
  const pending = readQueueOwner();
  window.dispatchEvent(new Event('omnisolo_auth_changed'));
  resolve(Response.json({ userId: 'a', tenantId: 't', expiresAt: Date.now() + 100000 }));
  await expect(pending).rejects.toThrow('identity');
});
