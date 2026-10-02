import { act, fireEvent, render, screen, waitFor, within } from '@testing-library/react';
import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import ReferralsPage from './page';
vi.mock('../components/PoweredByOmniSolo', () => ({ PoweredByOmniSolo: () => null }));
vi.mock('../components/GrowthReferralWidget', () => ({ default: () => null }));
const owner = { userId: 'owner-a', tenantId: 'tenant-a' };
const receipt = 'https://omnisolo.co/invite/actual-record-123';
let inviteReply: () => Promise<Response>;
let identityReply: () => Promise<Response>;
const key = 'omnisolo_invite_creation_v1:' + encodeURIComponent(JSON.stringify([owner.userId, owner.tenantId]));
const deferred = <T,>() => { let resolve!: (value: T) => void; const promise = new Promise<T>(yes => { resolve = yes; }); return { promise, resolve }; };
const posts = () => vi.mocked(fetch).mock.calls.filter(([url, init]) => url === '/api/v1/growth/cloud-bridge/invite' && init?.method === 'POST');
beforeEach(() => {
  localStorage.clear();
  inviteReply = async () => Response.json({ invite_link: receipt });
  identityReply = async () => Response.json({ ...owner, expiresAt: Date.now() + 60_000 });
  const locks = new Set<string>();
  Object.defineProperty(navigator, 'locks', { value: { request: async (name: string, _options: unknown, callback: (lock: object | null) => Promise<void>) => {
    if (locks.has(name)) return callback(null);
    locks.add(name); try { return await callback({}); } finally { locks.delete(name); }
  } } });
  vi.stubGlobal('fetch', vi.fn(async url => {
    if (url === '/api/v1/auth/session-identity') return identityReply();
    if (url === '/api/v1/growth/cloud-bridge/invite') return inviteReply();
    return Response.json({ referral_link: 'https://example.test/onboarding?ref=recorded' });
  }));
});
afterEach(() => { vi.unstubAllGlobals(); vi.restoreAllMocks(); });
async function create() {
  const button = screen.getByRole('button', { name: 'Generate Cloud Invite' });
  await waitFor(() => expect(button).toBeEnabled());
  fireEvent.change(screen.getByPlaceholderText('team-member@example.com'), { target: { value: 'member@example.test' } });
  await act(async () => { fireEvent.click(button); });
}
it.each([undefined, true])('displays the actual owner-bound invitation receipt with success=%j', async success => {
  inviteReply = async () => Response.json({ invite_link: receipt, ...(success === undefined ? {} : { success }) });
  render(<ReferralsPage />); await create();
  expect(posts()).toHaveLength(1);
  const options = posts()[0][1]!; const headers = new Headers(options.headers);
  expect(headers.get('x-ohc-expected-user')).toBe(owner.userId); expect(headers.get('x-ohc-expected-tenant')).toBe(owner.tenantId);
  expect(JSON.parse(options.body as string)).toEqual({ invitee_id: 'member@example.test' });
  expect(await screen.findByRole('status', { name: 'Cloud invitation status' })).toHaveTextContent(receipt);
  expect(document.body.textContent).not.toContain('/invite/member%40example.test');
});
it('a rejected creation never fabricates an invitation URL', async () => {
  inviteReply = async () => Response.json({ error: 'unavailable' }, { status: 503 });
  render(<ReferralsPage />); await create();
  expect(document.body.textContent).not.toContain('/invite/member%40example.test');
  expect(screen.getByRole('status', { name: 'Cloud invitation status' })).toHaveTextContent(/not be confirmed|unconfirmed/i);
  expect(screen.getByRole('button', { name: 'Generate Cloud Invite' })).toBeDisabled();
  expect(posts()).toHaveLength(1);
});
it('repeated clicks do not duplicate an in-flight request and no capability URL is persisted', async () => {
  const pending = deferred<Response>(); inviteReply = () => pending.promise;
  render(<ReferralsPage />); await create();
  fireEvent.click(screen.getByRole('button', { name: 'Generate Cloud Invite' }));
  expect(posts()).toHaveLength(1); expect(JSON.parse(localStorage.getItem(key)!)).toMatchObject({ state: 'pending', owner });
  await act(async () => pending.resolve(Response.json({ invite_link: receipt })));
  expect(screen.getByRole('status', { name: 'Cloud invitation status' })).toHaveTextContent(receipt);
  const bytes = localStorage.getItem(key)!; expect(JSON.parse(bytes).state).toBe('created'); expect(bytes).not.toContain(receipt); expect(bytes).not.toContain('member@example.test');
});
it.each(['pending', 'created'])('a previous %s marker remains held across a fresh view', async state => {
  const bytes = JSON.stringify({ version: 1, owner, operation: 'earlier-operation', state }); localStorage.setItem(key, bytes);
  render(<ReferralsPage />); await screen.findByText(/previous invitation request is unconfirmed|already created in this browser/);
  expect(screen.getByRole('button', { name: 'Generate Cloud Invite' })).toBeDisabled(); expect(posts()).toHaveLength(0); expect(localStorage.getItem(key)).toBe(bytes);
});
it.each(['auth', 'storage', 'null-storage', 'pagehide'])('%s retires a late receipt without exposing the previous owner link', async event => {
  const pending = deferred<Response>(); inviteReply = () => pending.promise;
  render(<ReferralsPage />); await create();
  act(() => window.dispatchEvent(event === 'auth' ? new Event('omnisolo_auth_changed') : event === 'pagehide' ? new Event('pagehide') : new StorageEvent('storage', { key: event === 'storage' ? 'omnisolo_queue_identity_epoch_v2' : null })));
  await act(async () => pending.resolve(Response.json({ invite_link: receipt })));
  expect(screen.getByRole('status', { name: 'Cloud invitation status' })).not.toHaveTextContent(receipt);
  expect(screen.getByRole('button', { name: 'Generate Cloud Invite' })).toBeDisabled(); expect(JSON.parse(localStorage.getItem(key)!).state).toBe('created');
  expect(screen.getByPlaceholderText('team-member@example.com')).toHaveValue('');
});
it('unmount prevents a late response from updating the replacement view', async () => {
  const pending = deferred<Response>(); inviteReply = () => pending.promise;
  const first = render(<ReferralsPage />); await create(); first.unmount();
  render(<ReferralsPage />); await screen.findByText(/previous invitation request is unconfirmed/);
  await act(async () => pending.resolve(Response.json({ invite_link: receipt })));
  expect(screen.getByRole('status', { name: 'Cloud invitation status' })).not.toHaveTextContent(receipt); expect(posts()).toHaveLength(1);
});
it.each([
  { status: 200, body: { invite_link: receipt, error: 'denied' } },
  { status: 200, body: { invite_link: receipt, success: 0 } },
  { status: 200, body: { invite_link: receipt, success: 'false' } },
  { status: 200, body: { invite_link: receipt, success: 'true' } },
  { status: 200, body: { invite_link: receipt, success: null } },
  { status: 202, body: { invite_link: receipt } },
  { status: 200, body: { invite_link: 'https://untrusted.test/invite/record' } },
  { status: 200, body: { invite_link: 'https://omnisolo.co/invite/fallback' } },
  { status: 200, body: { invite_url: receipt } },
])('unconfirmed receipt stays held without enabling retry: %j', async fixture => {
  inviteReply = async () => Response.json(fixture.body, { status: fixture.status });
  render(<ReferralsPage />); await create();
  expect(screen.getByRole('status', { name: 'Cloud invitation status' })).toHaveTextContent(/not be confirmed/);
  expect(screen.getByRole('button', { name: 'Generate Cloud Invite' })).toBeDisabled(); expect(JSON.parse(localStorage.getItem(key)!).state).toBe('pending');
});
it('missing origin locks cannot dispatch creation', async () => {
  Object.defineProperty(navigator, 'locks', { value: undefined });
  render(<ReferralsPage />); await screen.findByText(/Invitation access or local request history is unavailable/);
  expect(screen.getByRole('button', { name: 'Generate Cloud Invite' })).toBeDisabled(); expect(posts()).toHaveLength(0);
});
it('marker persistence failure holds the action before POST', async () => {
  vi.mocked(localStorage.setItem).mockImplementationOnce(() => { throw new Error('quota'); });
  render(<ReferralsPage />); await create();
  expect(posts()).toHaveLength(0); expect(screen.getByRole('status', { name: 'Cloud invitation status' })).toHaveTextContent(/creation is held/);
});
it('a changed verified owner cannot dispatch under the previous view', async () => {
  let calls = 0; identityReply = async () => Response.json({ ...owner, userId: ++calls === 1 ? owner.userId : 'owner-b', expiresAt: Date.now() + 60_000 });
  render(<ReferralsPage />); await create(); expect(posts()).toHaveLength(0);
  expect(screen.getByRole('status', { name: 'Cloud invitation status' })).toHaveTextContent(/session changed/);
});
it('an unsafe header identity holds before recording or dispatch', async () => {
  identityReply = async () => Response.json({ ...owner, userId: 'owner-雪', expiresAt: Date.now() + 60_000 });
  render(<ReferralsPage />); await create(); expect(posts()).toHaveLength(0);
  const unsafeKey = 'omnisolo_invite_creation_v1:' + encodeURIComponent(JSON.stringify(['owner-雪', owner.tenantId]));
  expect(localStorage.getItem(unsafeKey)).toBeNull();
});
it('corrupt own history and another owner marker remain intact', async () => {
  const otherKey = 'omnisolo_invite_creation_v1:other-owner'; localStorage.setItem(otherKey, 'foreign bytes'); localStorage.setItem(key, 'corrupt original');
  render(<ReferralsPage />); await screen.findByText(/Invitation access or local request history is unavailable/);
  expect(localStorage.getItem(key)).toBe('corrupt original'); expect(localStorage.getItem(otherKey)).toBe('foreign bytes'); expect(posts()).toHaveLength(0);
});
it.each(['network', 'malformed'])('%s outcome remains pending across a new view', async outcome => {
  inviteReply = async () => { if (outcome === 'network') throw new Error('connection lost'); return new Response('not JSON'); };
  const first = render(<ReferralsPage />); await create(); expect(posts()).toHaveLength(1); first.unmount();
  render(<ReferralsPage />); await screen.findByText(/previous invitation request is unconfirmed/);
  expect(screen.getByRole('button', { name: 'Generate Cloud Invite' })).toBeDisabled(); expect(posts()).toHaveLength(1);
});
it('separate mounted views cannot dispatch before the first request writes its marker', async () => {
  const pending = deferred<Response>(); let identities = 0;
  identityReply = async () => ++identities === 3 ? pending.promise : Response.json({ ...owner, expiresAt: Date.now() + 60_000 });
  const first = render(<ReferralsPage />); const second = render(<ReferralsPage />);
  const a = within(first.container); const b = within(second.container);
  await waitFor(() => { expect(a.getByRole('button', { name: 'Generate Cloud Invite' })).toBeEnabled(); expect(b.getByRole('button', { name: 'Generate Cloud Invite' })).toBeEnabled(); });
  fireEvent.change(a.getByPlaceholderText('team-member@example.com'), { target: { value: 'first@example.test' } });
  fireEvent.change(b.getByPlaceholderText('team-member@example.com'), { target: { value: 'second@example.test' } });
  await act(async () => { fireEvent.click(a.getByRole('button', { name: 'Generate Cloud Invite' })); });
  expect(localStorage.getItem(key)).toBeNull();
  await act(async () => { fireEvent.click(b.getByRole('button', { name: 'Generate Cloud Invite' })); });
  expect(posts()).toHaveLength(0); expect(b.getByRole('status', { name: 'Cloud invitation status' })).toHaveTextContent(/creation is held/);
  await act(async () => pending.resolve(Response.json({ ...owner, expiresAt: Date.now() + 60_000 })));
  expect(posts()).toHaveLength(1); expect(JSON.parse(posts()[0][1]!.body as string)).toEqual({ invitee_id: 'first@example.test' });
});

it('account retirement clears an unsubmitted private invitee draft', async () => {
  render(<ReferralsPage />); await waitFor(() => expect(screen.getByRole('button', { name: 'Generate Cloud Invite' })).toBeEnabled());
  fireEvent.change(screen.getByPlaceholderText('team-member@example.com'), { target: { value: 'private-person@example.test' } });
  act(() => window.dispatchEvent(new Event('omnisolo_auth_changed')));
  expect(screen.getByPlaceholderText('team-member@example.com')).toHaveValue(''); expect(posts()).toHaveLength(0);
});
it('a rejected pre-dispatch identity retires the private invitee draft', async () => {
  let calls = 0;
  identityReply = async () => ++calls === 1 ? Response.json({ ...owner, expiresAt: Date.now() + 60_000 }) : Response.json({ error: 'authentication required' }, { status: 401 });
  render(<ReferralsPage />); await create();
  expect(posts()).toHaveLength(0); expect(screen.getByPlaceholderText('team-member@example.com')).toHaveValue('');
  expect(screen.getByRole('status', { name: 'Cloud invitation status' })).toHaveTextContent(/session changed/);
});
