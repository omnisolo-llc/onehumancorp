import React from 'react';
import { act, cleanup, fireEvent, render, screen, waitFor, within } from '@testing-library/react';
import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import UnifiedFeed from './page';
import { notifyQueueIdentityChange } from '@/lib/sync/queueIdentity';
import { installOnboardingLocks } from '../onboarding/testLocks';

const ownerA = { userId: 'owner-a', tenantId: 'tenant-a' };
const row = { id: 'feed-one', tenant_id: ownerA.tenantId, event_source: 'Operations', context_payload: { description: 'Real pending work' }, proposed_action: { action_type: 'proposal', message: 'Original draft' }, lifecycle_state: 'PENDING_APPROVAL', created_at: '2026-10-03T12:00:00Z', updated_at: '2026-10-03T12:00:00Z' };
const originalStorage = window.localStorage;
function storage() {
  const values = new Map<string, string>();
  return { get length() { return values.size; }, key: (index: number) => [...values.keys()][index] ?? null,
    getItem: (key: string) => values.get(key) ?? null, setItem: (key: string, value: string) => { values.set(key, String(value)); },
    removeItem: (key: string) => { values.delete(key); }, clear: () => values.clear() };
}
let owner = ownerA;
let rows: typeof row[];
let mutation: () => Promise<Response>;
let identity: (() => Promise<Response>) | undefined;
let fetcher: ReturnType<typeof vi.fn<typeof fetch>>;
const writes = () => fetcher.mock.calls.filter(([, options]) => options?.method === 'PUT');
async function mount() { render(<UnifiedFeed />); await screen.findByText('Real pending work'); }
async function approve() { fireEvent.click(screen.getByRole('button', { name: 'Record approval' })); await waitFor(() => expect(writes()).toHaveLength(1)); }
beforeEach(() => {
  cleanup(); Object.defineProperty(window, 'localStorage', { value: storage(), writable: true }); localStorage.clear(); notifyQueueIdentityChange(); installOnboardingLocks(); owner = ownerA; rows = [{ ...row }]; identity = undefined;
  mutation = async () => Response.json({ ...row, lifecycle_state: 'APPROVED' });
  fetcher = vi.fn<typeof fetch>(async (input, options) => {
    const url = String(input);
    if (url === '/api/v1/auth/session-identity') return identity ? identity() : Response.json({ ...owner, expiresAt: Date.now() + 60_000 });
    if (options?.method === 'PUT') return mutation();
    if (url === '/api/v1/agent-feed') return Response.json({ items: rows });
    throw new Error('Unexpected endpoint ' + url);
  });
  vi.stubGlobal('fetch', fetcher);
});
afterEach(() => { vi.useRealTimers(); cleanup(); notifyQueueIdentityChange(); Object.defineProperty(window, 'localStorage', { value: originalStorage, writable: true }); vi.unstubAllGlobals(); });

it.each(['proposal', 'subscription_win_back'])('preserves discoverable feed cards and controls for %s', async actionType => {
  rows = [{ ...row, proposed_action: { ...row.proposed_action, action_type: actionType } }];
  await mount();
  const card = screen.getByTestId('agent-feed-card');
  expect(card).toHaveAttribute('id', `triage-card-${row.id}`);
  expect(within(card).getByText('Real pending work')).toBeVisible();
  expect(within(card).getByTestId('feed-approve-btn')).toBeEnabled();
  expect(within(card).getByTestId('feed-dismiss-btn')).toBeEnabled();
  if (actionType === 'proposal') {
    fireEvent.click(within(card).getByTestId('edit-proposal'));
    expect(within(card).getByTestId('edit-draft-textarea')).toHaveValue(JSON.stringify(rows[0].proposed_action));
    expect(within(card).getByTestId('save-edit-approve-btn')).toBeEnabled();
  }
  expect(writes()).toHaveLength(0);
});

it('keeps a pending card and submits only once until its matching persisted decision arrives', async () => {
  let finish!: (response: Response) => void;
  mutation = () => new Promise(resolve => { finish = resolve; });
  await mount(); await approve();
  expect(screen.getByText('Real pending work')).toBeVisible();
  expect(screen.getByRole('status', { name: 'Decision status' })).toHaveTextContent('Waiting for the recorded decision');
  fireEvent.click(screen.getByRole('button', { name: 'Dismiss' }));
  expect(writes()).toHaveLength(1);
  const options = writes()[0][1]!;
  expect(new Headers(options.headers).get('x-ohc-expected-user')).toBe(ownerA.userId);
  expect(new Headers(options.headers).get('x-ohc-expected-tenant')).toBe(ownerA.tenantId);
  await act(async () => finish(Response.json({ ...row, lifecycle_state: 'APPROVED' })));
  expect(screen.queryByText('Real pending work')).toBeNull();
  expect(screen.getByRole('status', { name: 'Decision status' })).toHaveTextContent('Approval recorded');
  expect(screen.getByRole('status', { name: 'Decision status' })).toHaveTextContent('Execution or delivery is not verified');
});
it('keeps an explicitly rejected decision visible and allows a new deliberate action', async () => {
  mutation = async () => Response.json({ error: 'invalid input' }, { status: 400 });
  await mount(); await approve();
  expect(await screen.findByRole('alert')).toHaveTextContent('Request rejected (HTTP 400)');
  expect(screen.getByText('Real pending work')).toBeVisible();
  expect(screen.getByRole('button', { name: 'Dismiss' })).toBeEnabled();
});
it.each(['http-500', 'network', 'wrong-id', 'wrong-tenant', 'wrong-state', 'empty-body', 'error-body'])('holds an ambiguous %s outcome without discarding the card or retrying', async failure => {
  mutation = async () => {
    if (failure === 'network') throw new Error('Network disconnected');
    if (failure === 'http-500') return new Response('', { status: 500 });
    if (failure === 'empty-body') return new Response('');
    if (failure === 'error-body') return Response.json({ ...row, lifecycle_state: 'APPROVED', error: 'not confirmed' });
    return Response.json({ ...row, lifecycle_state: failure === 'wrong-state' ? 'DISMISSED' : 'APPROVED', id: failure === 'wrong-id' ? 'another' : row.id, tenant_id: failure === 'wrong-tenant' ? 'other-tenant' : row.tenant_id });
  };
  await mount(); await approve();
  expect(await screen.findByRole('alert')).toHaveTextContent('Outcome unconfirmed');
  expect(screen.getByText('Real pending work')).toBeVisible();
  expect(screen.getByRole('button', { name: 'Dismiss' })).toBeDisabled();
  expect(screen.getByRole('button', { name: 'Record approval' })).toBeDisabled();
  expect(writes()).toHaveLength(1);
});
it('reads back an ambiguous decision without issuing a second mutation', async () => {
  mutation = async () => { throw new Error('Network disconnected'); };
  await mount(); await approve(); await screen.findByRole('alert');
  rows = [{ ...row, lifecycle_state: 'APPROVED' }];
  fireEvent.click(screen.getByRole('button', { name: 'Refresh recorded decisions' }));
  await waitFor(() => expect(screen.queryByText('Real pending work')).toBeNull());
  expect(screen.getByRole('status', { name: 'Decision status' })).toHaveTextContent('Approval recorded');
  expect(writes()).toHaveLength(1);
});
it('does not clear an ambiguous hold when readback still reports pending or omits the row', async () => {
  mutation = async () => { throw new Error('Network disconnected'); };
  await mount(); await approve(); await screen.findByRole('alert');
  rows = [];
  fireEvent.click(screen.getByRole('button', { name: 'Refresh recorded decisions' }));
  await waitFor(() => expect(screen.getByRole('button', { name: 'Refresh recorded decisions' })).toBeEnabled());
  expect(screen.getByText('Real pending work')).toBeVisible();
  expect(screen.getByRole('button', { name: 'Record approval' })).toBeDisabled();
  expect(writes()).toHaveLength(1);
});
it('persists the unknown hold across a same-owner remount', async () => {
  mutation = async () => { throw new Error('Network disconnected'); };
  await mount(); await approve(); await screen.findByRole('alert');
  cleanup(); await mount();
  expect(screen.getByRole('button', { name: 'Record approval' })).toBeDisabled();
  expect(screen.getByRole('alert')).toHaveTextContent('Outcome unconfirmed');
  expect(writes()).toHaveLength(1);
});
it('retires visible private rows and ignores a late mutation after a session change', async () => {
  let finish!: (response: Response) => void;
  mutation = () => new Promise(resolve => { finish = resolve; });
  await mount(); await approve();
  act(() => notifyQueueIdentityChange());
  expect(screen.queryByText('Real pending work')).toBeNull();
  await act(async () => finish(Response.json({ ...row, lifecycle_state: 'APPROVED' })));
  expect(screen.queryByText(/Approval recorded/)).toBeNull();
  expect(writes()).toHaveLength(1);
});
it('does not dispatch a card loaded under a different newly verified owner', async () => {
  await mount(); owner = { userId: 'owner-b', tenantId: 'tenant-b' };
  fireEvent.click(screen.getByRole('button', { name: 'Record approval' }));
  await waitFor(() => expect(screen.queryByText('Real pending work')).toBeNull());
  expect(writes()).toHaveLength(0);
});
it('requires the acknowledged edited content and retains its draft when the outcome is unknown', async () => {
  mutation = async () => Response.json({ ...row, lifecycle_state: 'APPROVED' });
  await mount(); fireEvent.click(screen.getByRole('button', { name: 'Edit' }));
  fireEvent.change(screen.getByTestId('edit-draft-textarea'), { target: { value: 'Reviewed updated draft' } });
  fireEvent.click(screen.getByRole('button', { name: 'Save & Approve' }));
  expect(await screen.findByRole('alert')).toHaveTextContent('Outcome unconfirmed');
  expect(screen.getByTestId('edit-draft-textarea')).toHaveValue('Reviewed updated draft');
  expect(writes()).toHaveLength(1);
});

it('preserves dismissal only after its actual row acknowledgement', async () => {
  mutation = async () => Response.json({ ...row, lifecycle_state: 'DISMISSED' });
  await mount(); fireEvent.click(screen.getByRole('button', { name: 'Dismiss' }));
  expect(await screen.findByRole('status', { name: 'Decision status' })).toHaveTextContent('Dismissal recorded');
  expect(screen.queryByText('Real pending work')).toBeNull();
});
it('keeps a prior acknowledged decision held in a stale reloaded feed without replay', async () => {
  await mount(); await approve(); await waitFor(() => expect(screen.queryByText('Real pending work')).toBeNull());
  cleanup(); await mount();
  expect(screen.getByRole('button', { name: 'Record approval' })).toBeDisabled();
  expect(writes()).toHaveLength(1);
});
it('does not offer data returned for a different tenant', async () => {
  rows = [{ ...row, tenant_id: 'foreign-tenant' }];
  render(<UnifiedFeed />);
  expect(await screen.findByRole('alert')).toHaveTextContent('could not be refreshed');
  expect(screen.queryByText('Real pending work')).toBeNull();
  expect(writes()).toHaveLength(0);
});
it('fails before sending when the origin lock is unavailable', async () => {
  Object.defineProperty(navigator, 'locks', { value: undefined });
  await mount(); fireEvent.click(screen.getByRole('button', { name: 'Record approval' }));
  expect(await screen.findByRole('alert')).toHaveTextContent('No request was sent');
  expect(screen.getByText('Real pending work')).toBeVisible();
  expect(writes()).toHaveLength(0);
});
it('does not claim an empty completed feed when an unresolved marker has no loaded row', async () => {
  mutation = async () => { throw new Error('Network disconnected'); };
  await mount(); await approve(); await screen.findByRole('alert'); cleanup(); rows = [];
  render(<UnifiedFeed />);
  expect(await screen.findByRole('alert')).toHaveTextContent('Outcome unconfirmed');
  expect(screen.queryByText('All caught up!')).toBeNull();
});
it('disables old cards if a fresh read cannot be verified', async () => {
  await mount();
  fetcher.mockImplementation(async input => String(input).endsWith('/session-identity')
    ? Response.json({ ...owner, expiresAt: Date.now() + 60_000 }) : new Response('', { status: 500 }));
  fireEvent.click(screen.getByRole('button', { name: 'Refresh recorded decisions' }));
  await screen.findByRole('alert');
  expect(screen.getByText('Real pending work')).toBeVisible();
  expect(screen.getByRole('button', { name: 'Record approval' })).toBeDisabled();
});
it('retires private cards on an empty authentication rejection during refresh', async () => {
  await mount();
  fetcher.mockImplementation(async input => String(input).endsWith('/session-identity')
    ? Response.json({ ...owner, expiresAt: Date.now() + 60_000 }) : new Response('', { status: 403 }));
  fireEvent.click(screen.getByRole('button', { name: 'Refresh recorded decisions' }));
  expect(await screen.findByRole('alert')).toHaveTextContent('Your session changed');
  expect(screen.queryByText('Real pending work')).toBeNull();
});

it('serializes two stale views and never repeats an already acknowledged mutation', async () => {
  let finish!: (response: Response) => void;
  mutation = () => new Promise(resolve => { finish = resolve; });
  render(<><section aria-label="First view"><UnifiedFeed /></section><section aria-label="Second view"><UnifiedFeed /></section></>);
  const first = within(screen.getByRole('region', { name: 'First view' }));
  const second = within(screen.getByRole('region', { name: 'Second view' }));
  await first.findByText('Real pending work'); await second.findByText('Real pending work');
  fireEvent.click(first.getByRole('button', { name: 'Record approval' }));
  await waitFor(() => expect(writes()).toHaveLength(1));
  fireEvent.click(second.getByRole('button', { name: 'Record approval' }));
  await act(async () => finish(Response.json({ ...row, lifecycle_state: 'APPROVED' })));
  expect(await second.findByRole('alert')).toHaveTextContent('Outcome unconfirmed');
  expect(second.getByRole('button', { name: 'Record approval' })).toBeDisabled();
  expect(writes()).toHaveLength(1);
});
it('clears private rows when the verified identity lease expires', async () => {
  vi.useFakeTimers();
  await act(async () => { render(<UnifiedFeed />); });
  expect(screen.getByText('Real pending work')).toBeVisible();
  await act(async () => { await vi.advanceTimersByTimeAsync(60_001); });
  expect(screen.queryByText('Real pending work')).toBeNull();
  expect(screen.getByRole('alert')).toHaveTextContent('Your session changed');
  expect(writes()).toHaveLength(0);
});
it('accepts a recorded edited decision only when the actual receipt preserves its content', async () => {
  mutation = async () => Response.json({ ...row, lifecycle_state: 'APPROVED', proposed_action: { message: 'Reviewed updated draft' } });
  await mount(); fireEvent.click(screen.getByRole('button', { name: 'Edit' }));
  fireEvent.change(screen.getByTestId('edit-draft-textarea'), { target: { value: 'Reviewed updated draft' } });
  fireEvent.click(screen.getByRole('button', { name: 'Save & Approve' }));
  await waitFor(() => expect(screen.queryByText('Real pending work')).toBeNull());
  expect(screen.getByRole('status', { name: 'Decision status' })).toHaveTextContent('Approval recorded');
  expect(JSON.parse(String(writes()[0][1]!.body))).toEqual({ state: 'APPROVED', edited_payload: 'Reviewed updated draft' });
});
it('reloads its owned feed after StrictMode retires the initial effect', async () => {
  render(<React.StrictMode><UnifiedFeed /></React.StrictMode>);
  expect(await screen.findByText('Real pending work')).toBeVisible();
  expect(screen.getByRole('button', { name: 'Record approval' })).toBeEnabled();
  expect(writes()).toHaveLength(0);
});
it('rejects edited acknowledgements with conflicting displayed and secondary fields', async () => {
  mutation = async () => Response.json({ ...row, lifecycle_state: 'APPROVED', proposed_action: { draft_reply: 'Old draft', message: 'Reviewed updated draft' } });
  await mount(); fireEvent.click(screen.getByRole('button', { name: 'Edit' }));
  fireEvent.change(screen.getByTestId('edit-draft-textarea'), { target: { value: 'Reviewed updated draft' } });
  fireEvent.click(screen.getByRole('button', { name: 'Save & Approve' }));
  expect(await screen.findByRole('alert')).toHaveTextContent('Outcome unconfirmed');
  expect(screen.getByTestId('edit-draft-textarea')).toHaveValue('Reviewed updated draft');
});
it('does not select a convenient acknowledgement from conflicting duplicate readback records', async () => {
  mutation = async () => { throw new Error('Network disconnected'); };
  await mount(); fireEvent.click(screen.getByRole('button', { name: 'Edit' }));
  fireEvent.change(screen.getByTestId('edit-draft-textarea'), { target: { value: 'Reviewed updated draft' } });
  fireEvent.click(screen.getByRole('button', { name: 'Save & Approve' }));
  await screen.findByRole('alert');
  rows = [{ ...row, lifecycle_state: 'APPROVED', proposed_action: { action_type: 'proposal', message: 'Reviewed updated draft' } }, { ...row, lifecycle_state: 'APPROVED' }];
  fireEvent.click(screen.getByRole('button', { name: 'Refresh recorded decisions' }));
  await screen.findByText('Recorded decisions could not be refreshed. Existing uncertain outcomes remain held.');
  expect(screen.getByText('Real pending work')).toBeVisible();
  expect(screen.getByTestId('edit-draft-textarea')).toHaveValue('Reviewed updated draft');
  expect(writes()).toHaveLength(1);
});
it('bounds a stalled initial identity read and releases its loading state', async () => {
  vi.useFakeTimers(); identity = () => new Promise(() => {});
  await act(async () => { render(<UnifiedFeed />); });
  expect(screen.getByText('Loading feed...')).toBeVisible();
  await act(async () => { await vi.advanceTimersByTimeAsync(30_001); });
  expect(screen.queryByText('Loading feed...')).toBeNull();
  expect(screen.getByRole('alert')).toBeVisible();
  expect(screen.getByRole('button', { name: 'Refresh recorded decisions' })).toBeEnabled();
  expect(writes()).toHaveLength(0);
});
it('bounds stalled pre-dispatch identity verification without sending or retaining a pending lock', async () => {
  await mount(); vi.useFakeTimers(); identity = () => new Promise(() => {});
  await act(async () => { fireEvent.click(screen.getByRole('button', { name: 'Record approval' })); });
  await act(async () => { await vi.advanceTimersByTimeAsync(30_001); });
  expect(screen.getByRole('alert')).toBeVisible();
  expect(screen.getByRole('button', { name: 'Refresh recorded decisions' })).toBeEnabled();
  expect(writes()).toHaveLength(0);
  identity = undefined;
  await act(async () => { fireEvent.click(screen.getByRole('button', { name: 'Refresh recorded decisions' })); });
  expect(screen.getByText('Real pending work')).toBeVisible();
  expect(screen.getByRole('button', { name: 'Record approval' })).toBeEnabled();
});

it.each(['proposal', 'subscription_win_back'])('labels %s approval as a recorded decision while its receipt is pending', async actionType => {
  rows = [{ ...row, proposed_action: { ...row.proposed_action, action_type: actionType } }];
  let finish!: (response: Response) => void;
  mutation = () => new Promise(resolve => { finish = resolve; });
  await mount();
  expect(screen.queryByRole('button', { name: /send/i })).toBeNull();
  fireEvent.click(screen.getByRole('button', { name: /^Record approval$/ }));
  await waitFor(() => expect(writes()).toHaveLength(1));
  expect(screen.getByRole('button', { name: /^Recording approval\.\.\.$/ })).toBeDisabled();
  expect(screen.getByRole('status', { name: 'Decision status' })).toHaveTextContent('No execution or delivery is confirmed');
  await act(async () => finish(Response.json({ ...rows[0], lifecycle_state: 'APPROVED' })));
  expect(screen.getByRole('status', { name: 'Decision status' })).toHaveTextContent('Approval recorded. Execution or delivery is not verified');
});
