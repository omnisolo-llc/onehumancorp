import React from 'react';
import { act, cleanup, fireEvent, render, screen, waitFor } from '@testing-library/react';
import { afterEach, beforeEach, expect, test, vi } from 'vitest';
import TeamPage from './page';
import TeamChatPage from './chat/page';
import { invalidateQueueOwner, notifyQueueIdentityChange, readQueueOwner } from '@/lib/sync/queueIdentity';

vi.mock('../components/GrowthReferralWidget', () => ({ default: () => null }));
vi.mock('../../components/TooltipRegistry', () => ({ WithTooltip: ({ children }: { children: React.ReactNode }) => <>{children}</> }));
const id = '9e493221-eaf9-4a9c-9e88-354398e55211';
const approval = (changes = {}) => ({ id, tenant_id: 'tenant-a', department: 'CustomerSuccess', description: 'Review customer reply', status: 'PendingApproval', action_risk: 'DraftForReview', payload: { feature_type: 'ambassador_reply', original_message: 'Can I book Friday?', generated_response: 'Friday has availability.' }, ...changes });
const receipt = (changes = {}) => ({ id, tenant_id: 'tenant-a', event_source: 'customer_success', context_payload: { description: 'Review customer reply' }, proposed_action: approval().payload, lifecycle_state: 'APPROVED', decision_recorded: true, dispatch: { status: 'PENDING', job_id: 'job-1', attempted_at: null, dispatch_returned_at: null, detail: null }, ...changes });
let rows: unknown[], cursorPage: unknown[], calls: Array<[string, RequestInit | undefined]>;
let sendReply: () => Promise<Response>, writeReply: () => Promise<Response>, readReply: () => Promise<Response>;
beforeEach(() => {
  const values = new Map<string, string>();
  Object.defineProperty(window, 'localStorage', { value: { get length() { return values.size; }, key: (index: number) => [...values.keys()][index] ?? null, getItem: (key: string) => values.get(key) ?? null, setItem: (key: string, value: string) => values.set(key, String(value)), removeItem: (key: string) => values.delete(key), clear: () => values.clear() }, writable: true });
  localStorage.clear(); invalidateQueueOwner(); rows = []; cursorPage = []; calls = [];
  Object.defineProperty(navigator, 'onLine', { configurable: true, value: true });
  navigator.locks.request = vi.fn(async (_key, _options, callback) => callback!({} as Lock)) as unknown as typeof navigator.locks.request;
  sendReply = async () => Response.json({ success: true, department_assigned: 'customer_success', approval: approval() });
  writeReply = async () => Response.json(receipt());
  readReply = async () => Response.json(receipt());
  vi.stubGlobal('fetch', vi.fn(async (input, options) => {
    const url = String(input); calls.push([url, options]);
    if (url.endsWith('/session-identity')) return Response.json({ userId: 'owner-a', tenantId: 'tenant-a', expiresAt: Date.now() + 60000 });
    if (url === '/api/v1/agents/chat') return sendReply();
    if (url.endsWith('/decision')) return readReply();
    if (options?.method === 'PUT') return writeReply();
    if (url.startsWith('/api/v1/agents/approvals')) return Response.json({ pending_approvals: url.includes('cursor=') ? cursorPage : rows, next_cursor: !url.includes('cursor=') && cursorPage.length ? 'page-a' : null });
    throw new Error('Unexpected HTTP destination: ' + url);
  }));
});
afterEach(() => { cleanup(); invalidateQueueOwner(); vi.unstubAllGlobals(); });
async function chat() {
  render(<TeamChatPage />);
  await waitFor(() => expect(calls.some(([url]) => url.startsWith('/api/v1/agents/approvals'))).toBe(true));
  fireEvent.change(screen.getByTestId('team-chat-input'), { target: { value: 'Help the customer' } });
  fireEvent.click(screen.getByTestId('team-chat-send'));
  await screen.findByText('Review customer reply');
}

test('renders actual Rust department DTOs across every pending page using structured payload', async () => {
  rows = [approval({ id: 'first', department: 'Operations' })];
  cursorPage = [approval()]; render(<TeamPage />);
  const card = await screen.findByRole('button', { name: /The Ambassador.*1 item/ });
  fireEvent.click(card);
  expect(await screen.findByText('Friday has availability.')).toBeVisible();
  expect(screen.queryByText('Active and running')).toBeNull();
  expect(calls.some(([url]) => url.includes('cursor=page-a'))).toBe(true);
});

test('chat submits persisted approval ID to canonical durable decision route and describes dispatch honestly', async () => {
  await chat(); fireEvent.click(screen.getByRole('button', { name: 'Record approval' }));
  expect(await screen.findByText(/Approval recorded.*queued.*not verified/i)).toBeVisible();
  const writes = calls.filter(([, options]) => options?.method === 'PUT');
  expect(writes).toHaveLength(1); expect(writes[0][0]).toBe(`/api/v1/agent-feed/${id}`);
  expect(JSON.parse(String(writes[0][1]?.body))).toEqual({ state: 'APPROVED' });
  expect(new Headers(writes[0][1]?.headers).get('x-ohc-expected-tenant')).toBe('tenant-a');
  expect(screen.queryByText(/Approve & Execute|All departments online/)).toBeNull();
});

test.each([{}, { success: true, department_assigned: 'sales' }, { success: true, approval: approval({ tenant_id: 'foreign' }) }, { success: true, approval: approval({ department: 'Unknown' }) }])('malformed or foreign chat DTO never creates actionable authority: %j', async payload => {
  sendReply = async () => Response.json(payload); render(<TeamChatPage />);
  await waitFor(() => expect(calls.some(([url]) => url.startsWith('/api/v1/agents/approvals'))).toBe(true));
  fireEvent.change(screen.getByTestId('team-chat-input'), { target: { value: 'Request' } }); fireEvent.click(screen.getByTestId('team-chat-send'));
  expect(await screen.findByText(/Request outcome is unconfirmed/)).toBeVisible();
  expect(screen.queryByRole('button', { name: 'Record approval' })).toBeNull();
});

test('unknown mutation is held through reload and reconciles by GET without replay', async () => {
  rows = [approval()]; writeReply = async () => { throw Error('response lost'); };
  const first = render(<TeamChatPage />); fireEvent.click(await screen.findByRole('button', { name: 'Record approval' }));
  expect(await screen.findByText(/Decision outcome is unconfirmed/)).toBeVisible(); first.unmount();
  rows = []; readReply = async () => Response.json(receipt({ dispatch: { status: 'DISPATCH_RETURNED', job_id: 'job-1', attempted_at: '2026-10-06T00:00:00Z', dispatch_returned_at: '2026-10-06T00:01:00Z', detail: null } }));
  render(<TeamChatPage />);
  expect(await screen.findByText(/Dispatch returned.*delivery is not verified/)).toBeVisible();
  expect(calls.filter(([, options]) => options?.method === 'PUT')).toHaveLength(1);
});

test('malformed successful decision cannot hide pending card or claim approval', async () => {
  rows = [approval()]; writeReply = async () => Response.json({ success: true }); render(<TeamChatPage />);
  fireEvent.click(await screen.findByRole('button', { name: 'Record approval' }));
  expect(await screen.findByText(/Decision outcome is unconfirmed/)).toBeVisible();
  expect(screen.getByText('Review customer reply')).toBeVisible();
  expect(screen.getByRole('button', { name: 'Record approval' })).toBeDisabled();
});

test('authority invalidation during decision retires private cards and rejects late success', async () => {
  rows = [approval()]; let finish!: (value: Response) => void;
  writeReply = () => new Promise(resolve => { finish = resolve; }); render(<TeamChatPage />);
  fireEvent.click(await screen.findByRole('button', { name: 'Record approval' }));
  await waitFor(() => expect(finish).toBeTypeOf('function'));
  act(() => notifyQueueIdentityChange()); await act(async () => finish(Response.json(receipt())));
  expect(screen.queryByText('Review customer reply')).toBeNull();
  expect(screen.queryByText(/Approval recorded/)).toBeNull();
});

test('foreign or malformed approval list is unavailable rather than empty', async () => {
  rows = [approval({ tenant_id: 'foreign' })]; render(<TeamPage />);
  expect(await screen.findByRole('alert')).toHaveTextContent(/unavailable/i);
  expect(screen.queryByText(/All Caught Up|Active and running/)).toBeNull();
});

test('repeated clicks while pending issue one decision and preserve authority headers', async () => {
  rows = [approval()]; let finish!: (value: Response) => void;
  writeReply = () => new Promise(resolve => { finish = resolve; }); render(<TeamChatPage />);
  const button = await screen.findByRole('button', { name: 'Record approval' });
  fireEvent.click(button); fireEvent.click(button);
  await waitFor(() => expect(finish).toBeTypeOf('function'));
  expect(calls.filter(([, options]) => options?.method === 'PUT')).toHaveLength(1);
  await act(async () => finish(Response.json(receipt())));
});

test('an HTTP failure remains held until exact tenant and payload readback is verified', async () => {
  rows = [approval()]; writeReply = async () => Response.json({ error: 'unavailable' }, { status: 503 });
  readReply = async () => Response.json(receipt({ tenant_id: 'foreign' })); render(<TeamChatPage />);
  fireEvent.click(await screen.findByRole('button', { name: 'Record approval' }));
  expect(await screen.findByText(/Decision outcome is unconfirmed/)).toBeVisible();
  fireEvent.click(screen.getByRole('button', { name: 'Refresh recorded decisions' }));
  await waitFor(() => expect(calls.some(([url]) => url.endsWith('/decision'))).toBe(true));
  expect(screen.queryByText(/Approval recorded/)).toBeNull();
  expect(calls.filter(([, options]) => options?.method === 'PUT')).toHaveLength(1);
});

test('department edit submits the structured payload and keeps the edited draft on malformed receipt', async () => {
  rows = [approval()]; writeReply = async () => Response.json(receipt()); render(<TeamPage />);
  fireEvent.click(await screen.findByRole('button', { name: /The Ambassador.*1 item/ }));
  fireEvent.click(screen.getByRole('button', { name: 'Edit' }));
  fireEvent.change(screen.getByTestId('edit-ambassador-draft'), { target: { value: 'Owner edited reply' } });
  fireEvent.click(screen.getByTestId('modal-approve-btn'));
  expect(await screen.findByText(/Decision outcome is unconfirmed/)).toBeVisible();
  const write = calls.find(([, options]) => options?.method === 'PUT')!;
  const body = JSON.parse(String(write[1]?.body));
  expect(body.proposed_action.original_message).toBe('Can I book Friday?');
  expect(body.proposed_action.generated_response).toBe('Owner edited reply');
  expect(screen.getByTestId('edit-ambassador-draft')).toHaveValue('Owner edited reply');
});

test('unknown chat creation is not replayed after another failed request or reload', async () => {
  sendReply = async () => { throw Error('lost receipt'); };
  const first = render(<TeamChatPage />);
  await waitFor(() => expect(screen.getByTestId('team-chat-input')).toBeEnabled());
  for (const message of ['First request', 'Second request', 'First request']) {
    fireEvent.change(screen.getByTestId('team-chat-input'), { target: { value: message } }); fireEvent.click(screen.getByTestId('team-chat-send'));
    await screen.findByText(/Request outcome is unconfirmed/); await waitFor(() => expect(screen.getByTestId('team-chat-send')).toBeEnabled());
  }
  expect(calls.filter(([url]) => url === '/api/v1/agents/chat')).toHaveLength(2);
  first.unmount(); render(<TeamChatPage />);
  await waitFor(() => expect(screen.getByTestId('team-chat-input')).toBeEnabled());
  fireEvent.change(screen.getByTestId('team-chat-input'), { target: { value: 'First request' } }); fireEvent.click(screen.getByTestId('team-chat-send'));
  await screen.findByText(/Request outcome is unconfirmed/);
  expect(calls.filter(([url]) => url === '/api/v1/agents/chat')).toHaveLength(2);
});

test.each(['legal_compliance', 'global_localization', 'ai_geo', 'social_calendar'])('department %s cards show returned evidence instead of sample facts', async feature_type => {
  rows = [approval({ department: 'Legal', payload: { feature_type, description: 'Exact returned evidence' } })]; render(<TeamPage />);
  fireEvent.click(await screen.findByRole('button', { name: /The Protector.*1 item/ }));
  expect(await screen.findByText(/Exact returned evidence/)).toBeVisible();
  expect(screen.queryByText(/Sales are approaching|New flavor drop|Search queries|Updated Privacy Policy/)).toBeNull();
  expect(screen.getByRole('button', { name: 'Department policy unavailable' })).toBeDisabled();
});

test('independent owner verification suspends private card rendering while identity is unresolved', async () => {
  rows = [approval()]; render(<TeamChatPage />); await screen.findByText('Review customer reply');
  const { readQueueOwner } = await import('@/lib/sync/queueIdentity');
  vi.mocked(fetch).mockImplementationOnce(() => new Promise(() => {}));
  act(() => { void readQueueOwner(); });
  expect(screen.getByText('Review customer reply')).not.toBeVisible();
});

test('an open department inbox never reports caught up when refresh fails', async () => {
  rows = [approval()]; render(<TeamPage />);
  fireEvent.click(await screen.findByRole('button', { name: /The Ambassador.*1 item/ }));
  expect(screen.getByText('Friday has availability.')).toBeVisible();
  const original = vi.mocked(fetch).getMockImplementation()!;
  vi.mocked(fetch).mockImplementation((input, options) => String(input).startsWith('/api/v1/agents/approvals')
    ? Promise.resolve(new Response(null, { status: 503 })) : original(input, options));
  fireEvent.click(screen.getByRole('button', { name: 'Refresh recorded decisions' }));
  expect(await screen.findByRole('alert')).toHaveTextContent('Approvals are unavailable');
  expect(screen.queryByText(/All Caught Up|There are no pending actions/)).toBeNull();
});

test.each([401, 403, 409])('empty-body chat HTTP %s retires private state before JSON parsing and never replays', async status => {
  rows = [approval()]; sendReply = async () => new Response(null, { status }); render(<TeamChatPage />);
  await screen.findByText('Review customer reply');
  fireEvent.change(screen.getByTestId('team-chat-input'), { target: { value: 'Private unsent request' } });
  fireEvent.click(screen.getByTestId('team-chat-send'));
  expect(await screen.findByRole('alert')).toHaveTextContent('Your session changed');
  expect(screen.queryByText('Review customer reply')).toBeNull();
  expect(screen.getByTestId('team-chat-input')).toHaveValue('');
  expect(screen.getByTestId('team-chat-send')).toBeDisabled();
  expect(calls.filter(([url]) => url === '/api/v1/agents/chat')).toHaveLength(1);
  fireEvent.click(screen.getByTestId('team-chat-send'));
  expect(calls.filter(([url]) => url === '/api/v1/agents/chat')).toHaveLength(1);
});

test('StrictMode effect replay completes the verified list and enables the composer', async () => {
  rows = [approval()]; render(<React.StrictMode><TeamChatPage /></React.StrictMode>);
  expect(await screen.findByText('Review customer reply')).toBeVisible();
  await waitFor(() => expect(screen.getByTestId('team-chat-input')).toBeEnabled());
  expect(screen.queryByText('Loading recorded approvals…')).toBeNull();
  fireEvent.change(screen.getByTestId('team-chat-input'), { target: { value: 'After effect replay' } });
  expect(screen.getByTestId('team-chat-send')).toBeEnabled();
});

test('previously verified empty inbox suspends its empty claim during refresh', async () => {
  rows = [approval()]; render(<TeamPage />);
  fireEvent.click(await screen.findByRole('button', { name: /The Ambassador.*1 item/ }));
  fireEvent.click(screen.getByRole('button', { name: 'Record approval' }));
  expect(await screen.findByText('All Caught Up!')).toBeVisible();
  const original = vi.mocked(fetch).getMockImplementation()!;
  let finish!: (value: Response) => void;
  vi.mocked(fetch).mockImplementation((input, options) => String(input).startsWith('/api/v1/agents/approvals')
    ? new Promise(resolve => { finish = resolve; }) : original(input, options));
  fireEvent.click(screen.getByRole('button', { name: 'Refresh recorded decisions' }));
  await waitFor(() => expect(finish).toBeTypeOf('function'));
  expect(screen.queryByText('All Caught Up!')).toBeNull();
  await act(async () => finish(Response.json({ pending_approvals: [], next_cursor: null })));
  expect(await screen.findByText('All Caught Up!')).toBeVisible();
});

test('bodyless approval and decision reads carry owner preconditions without a JSON body type', async () => {
  await chat();
  fireEvent.click(screen.getByRole('button', { name: 'Record approval' }));
  await screen.findByText(/Approval recorded.*queued.*not verified/i);
  cleanup(); rows = [];
  render(<TeamPage />);
  await screen.findByText(/Approval recorded.*queued.*not verified/i);
  const reads = calls.filter(([url, options]) => !['POST', 'PUT'].includes(options?.method ?? 'GET')
    && (url.startsWith('/api/v1/agents/approvals') || url.endsWith('/decision')));
  expect(reads.some(([url]) => url.endsWith('/decision'))).toBe(true);
  for (const [, options] of reads) {
    const headers = new Headers(options?.headers);
    expect(headers.get('content-type')).toBeNull();
    expect(headers.get('x-ohc-expected-user')).toBe('owner-a');
    expect(headers.get('x-ohc-expected-tenant')).toBe('tenant-a');
  }
  const writes = calls.filter(([, options]) => ['POST', 'PUT'].includes(options?.method ?? 'GET'));
  expect(writes).toHaveLength(2);
  for (const [, options] of writes) expect(new Headers(options?.headers).get('content-type')).toBe('application/json');
});

test.each([true, false])('a finite approval body is consumed while overlapping verification settles; same owner=%s', async sameOwner => {
  const original = vi.mocked(fetch).getMockImplementation()!;
  let finishList!: (response: Response) => void, finishIdentity!: (response: Response) => void;
  let overlap = false;
  const finite = Response.json({ pending_approvals: [approval()], next_cursor: null });
  vi.mocked(fetch).mockImplementation((input, options) => {
    const url = String(input);
    if (url.startsWith('/api/v1/agents/approvals')) return new Promise(resolve => { finishList = resolve; });
    if (url.endsWith('/session-identity') && overlap) return new Promise(resolve => { finishIdentity = resolve; });
    return original(input, options);
  });
  render(<TeamPage />);
  await waitFor(() => expect(finishList).toBeTypeOf('function'));
  overlap = true;
  const verification = readQueueOwner();
  await waitFor(() => expect(finishIdentity).toBeTypeOf('function'));
  await act(async () => finishList(finite));
  await waitFor(() => expect(finite.bodyUsed).toBe(true));
  expect(screen.queryByText('No pending approvals')).toBeNull();
  await act(async () => {
    finishIdentity(Response.json({ userId: sameOwner ? 'owner-a' : 'owner-b', tenantId: sameOwner ? 'tenant-a' : 'tenant-b', expiresAt: Date.now() + 60000 }));
    await verification;
  });
  if (sameOwner) {
    expect(await screen.findByRole('button', { name: /The Ambassador.*1 item/ })).toBeEnabled();
    expect(screen.queryByRole('alert')).toBeNull();
  } else {
    expect(await screen.findByRole('alert')).toHaveTextContent('Your session changed');
    expect(screen.queryByRole('button', { name: /The Ambassador.*1 item/ })).toBeNull();
    expect(screen.queryByText('Review customer reply')).toBeNull();
  }
});
