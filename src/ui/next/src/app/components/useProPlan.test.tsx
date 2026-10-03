import { act, fireEvent, render, screen, waitFor } from '@testing-library/react';
import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import { invalidateQueueOwner, readQueueOwner } from '@/lib/sync/queueIdentity';
import { useProPlan } from './useProPlan';
const a = { userId: 'plan-user-a', tenantId: 'plan-tenant-a' };
const b = { userId: 'plan-user-b', tenantId: 'plan-tenant-b' };
let current = a;
let planReply: () => Promise<Response>;
function deferred<T>() { let resolve!: (value: T) => void; const promise = new Promise<T>(yes => { resolve = yes; }); return { promise, resolve }; }
function Harness() {
  const plan = useProPlan();
  return <><output aria-label="Pro">{String(plan.hasPro)}</output><button onClick={() => void plan.refreshPlan()}>Refresh plan</button><button onClick={() => void plan.claimTrial()}>Check trial availability</button><p role="status" aria-label="Plan status">{plan.planError ?? plan.claimError ?? ''}</p></>;
}
beforeEach(() => {
  localStorage.clear(); act(() => invalidateQueueOwner()); current = a;
  Object.defineProperty(navigator, 'onLine', { configurable: true, value: true });
  planReply = async () => Response.json({ current_plan: 'Pro' });
  vi.stubGlobal('fetch', vi.fn(async input => String(input) === '/api/v1/auth/session-identity' ? Response.json({ ...current, expiresAt: Date.now() + 60000 }) : planReply()));
});
afterEach(() => { act(() => invalidateQueueOwner()); vi.unstubAllGlobals(); vi.restoreAllMocks(); });
it('uses the verified owner precondition for the actual current-plan read', async () => {
  render(<Harness />); await waitFor(() => expect(screen.getByLabelText('Pro')).toHaveTextContent('true'));
  const [,options] = vi.mocked(fetch).mock.calls.find(([input]) => input === '/api/v1/billing/my-plan')!;
  const headers = new Headers(options?.headers);
  expect(headers.get('x-ohc-expected-user')).toBe(a.userId); expect(headers.get('x-ohc-expected-tenant')).toBe(a.tenantId);
  expect(options).toMatchObject({ credentials: 'same-origin', redirect: 'error', cache: 'no-store' });
});
it.each(['Free','Starter','Pro','Business'])('reads the known current plan %s', async current_plan => {
  planReply = async () => Response.json({ current_plan }); render(<Harness />);
  await waitFor(() => expect(vi.mocked(fetch).mock.calls.some(([url]) => url === '/api/v1/billing/my-plan')).toBe(true));
  await waitFor(() => expect(screen.getByLabelText('Pro')).toHaveTextContent(String(['Pro','Business'].includes(current_plan))));
});
it.each([{ current_plan: 'Pro', success: false }, { current_plan: 'Pro', success: 0 }, { current_plan: 'Pro', error: 'unavailable' }, { current_plan: 'Unknown' }, { current_plan: ['Pro'] }, null])('does not use a contradictory or unknown plan receipt: %j', async data => {
  planReply = async () => Response.json(data); render(<Harness />);
  await waitFor(() => expect(screen.getByRole('status', { name: 'Plan status' })).toHaveTextContent(/unavailable/i)); expect(screen.getByLabelText('Pro')).toHaveTextContent('false');
});
it('does not grant access from a nonterminal202 receipt', async () => {
  planReply = async () => Response.json({ current_plan: 'Pro' }, { status: 202 }); render(<Harness />);
  await waitFor(() => expect(screen.getByRole('status', { name: 'Plan status' })).toHaveTextContent(/unavailable/i)); expect(screen.getByLabelText('Pro')).toHaveTextContent('false');
});
it.each(['omnisolo_auth_changed','pagehide','storage'])('retires the old plan immediately on %s', async event => {
  render(<Harness />); await waitFor(() => expect(screen.getByLabelText('Pro')).toHaveTextContent('true'));
  act(() => window.dispatchEvent(event === 'storage' ? new StorageEvent('storage', { key: 'omnisolo_queue_identity_epoch_v2' }) : new Event(event)));
  expect(screen.getByLabelText('Pro')).toHaveTextContent('false');
});
it('retires a no-event canonical owner replacement and rejects a late prior plan body', async () => {
  const body = deferred<unknown>(); planReply = async () => ({ status: 200, json: () => body.promise }) as Response;
  render(<Harness />); await waitFor(() => expect(vi.mocked(fetch).mock.calls.some(([url]) => url === '/api/v1/billing/my-plan')).toBe(true));
  current = b; await act(async () => { await readQueueOwner(); });
  await act(async () => body.resolve({ current_plan: 'Pro' })); expect(screen.getByLabelText('Pro')).toHaveTextContent('false');
});
it('a newer Free read cannot be overwritten by an older Pro body', async () => {
  const body = deferred<unknown>(); planReply = async () => ({ status: 200, json: () => body.promise }) as Response;
  render(<Harness />); await waitFor(() => expect(vi.mocked(fetch).mock.calls.some(([url]) => url === '/api/v1/billing/my-plan')).toBe(true));
  planReply = async () => Response.json({ current_plan: 'Free' }); await act(async () => fireEvent.click(screen.getByRole('button', { name: 'Refresh plan' })));
  await act(async () => body.resolve({ current_plan: 'Pro' })); expect(screen.getByLabelText('Pro')).toHaveTextContent('false');
});
it('a trial check cannot flip the entitlement or submit an unverified grant mutation', async () => {
  planReply = async () => Response.json({ current_plan: 'Free' }); render(<Harness />);
  await waitFor(() => expect(vi.mocked(fetch).mock.calls.some(([url]) => url === '/api/v1/billing/my-plan')).toBe(true));
  await act(async () => fireEvent.click(screen.getByRole('button', { name: 'Check trial availability' })));
  expect(screen.getByLabelText('Pro')).toHaveTextContent('false'); expect(vi.mocked(fetch).mock.calls.some(([,options]) => options?.method === 'POST')).toBe(false);
  expect(screen.getByRole('status', { name: 'Plan status' })).toHaveTextContent(/unavailable|not verified/i);
});

it('holds a confirmed plan during a pending same-owner verification and restores it afterward', async () => {
  render(<Harness />); await waitFor(() => expect(screen.getByLabelText('Pro')).toHaveTextContent('true'));
  const pending = deferred<Response>();
  vi.mocked(fetch).mockImplementation(async input => String(input) === '/api/v1/auth/session-identity' ? pending.promise : planReply());
  let checking!: Promise<unknown>; act(() => { checking = readQueueOwner(); });
  expect(screen.getByLabelText('Pro')).toHaveTextContent('false');
  await act(async () => { pending.resolve(Response.json({ ...a, expiresAt: Date.now() + 60000 })); await checking; });
  expect(screen.getByLabelText('Pro')).toHaveTextContent('true');
});
it('retires a denied current-plan read before consuming a stalled body', async () => {
  const json = vi.fn(() => new Promise(() => {})); planReply = async () => ({ status: 401, json }) as unknown as Response;
  render(<Harness />); await waitFor(() => expect(screen.getByRole('status', { name: 'Plan status' })).toHaveTextContent(/session changed/i));
  expect(json).not.toHaveBeenCalled(); expect(screen.getByLabelText('Pro')).toHaveTextContent('false');
});
it('an unmounted plan response cannot update the next view', async () => {
  const body = deferred<unknown>(); planReply = async () => ({ status: 200, json: () => body.promise }) as Response;
  const first = render(<Harness />); await waitFor(() => expect(vi.mocked(fetch).mock.calls.some(([url]) => url === '/api/v1/billing/my-plan')).toBe(true));
  first.unmount(); current = b; planReply = async () => Response.json({ current_plan: 'Free' }); render(<Harness />);
  await act(async () => body.resolve({ current_plan: 'Pro' }));
  expect(screen.getByLabelText('Pro')).toHaveTextContent('false');
});
