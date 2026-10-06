import { act, fireEvent, render, screen, waitFor } from '@testing-library/react';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { DashboardViralInviteWidget } from './DashboardViralInviteWidget';

const owner = { userId: 'owner-a', tenantId: 'tenant-a' };
const receipt = 'https://omnisolo.co/invite/recorded-token-123';
const key = 'omnisolo_invite_creation_v1:' + encodeURIComponent(JSON.stringify([owner.userId, owner.tenantId]));
const deferred = <T,>() => { let resolve!: (value: T) => void; let reject!: (error: unknown) => void; const promise = new Promise<T>((yes, no) => { resolve = yes; reject = no; }); return { promise, resolve, reject }; };
let inviteReply: () => Promise<Response>;
let identityReply: () => Promise<Response>;
let writeText: ReturnType<typeof vi.fn>;
const posts = () => vi.mocked(fetch).mock.calls.filter(([url, init]) => url === '/api/v1/growth/cloud-bridge/invite' && init?.method === 'POST');
beforeEach(() => {
  localStorage.clear(); localStorage.setItem('business_display_name', 'untrusted-display-name');
  inviteReply = async () => Response.json({ invite_link: receipt });
  identityReply = async () => Response.json({ ...owner, expiresAt: Date.now() + 60_000 });
  vi.stubGlobal('fetch', vi.fn(async url => {
    if (url === '/api/v1/auth/session-identity') return identityReply();
    if (url === '/api/v1/growth/cloud-bridge/invite') return inviteReply();
    return Response.json({ error: 'unsupported endpoint' }, { status: 404 });
  }));
  const locks = new Set<string>();
  Object.defineProperty(navigator, 'locks', { value: { request: async (name: string, _options: unknown, callback: (lock: object | null) => Promise<void>) => {
    if (locks.has(name)) return callback(null);
    locks.add(name); try { return await callback({}); } finally { locks.delete(name); }
  } } });
  writeText = vi.fn().mockResolvedValue(undefined);
  Object.defineProperty(navigator, 'clipboard', { configurable: true, value: { writeText } });
});
afterEach(() => { vi.unstubAllGlobals(); vi.restoreAllMocks(); });
async function create() {
  const button = screen.getByRole('button', { name: 'Get My Invite Link' });
  await waitFor(() => expect(button).toBeEnabled());
  await act(async () => { fireEvent.click(button); });
}

describe('DashboardViralInviteWidget', () => {
  it('renders the maintained invitation action without unverified reward promises', () => {
    render(<DashboardViralInviteWidget />);
    expect(screen.getByText('Invite a Business Owner')).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Get My Invite Link' })).toBeInTheDocument();
    expect(document.body.textContent).not.toMatch(/\$50|1 month free/);
  });
  it('displays the exact owner-bound receipt and genuine share intent', async () => {
    render(<DashboardViralInviteWidget />); await create();
    expect(await screen.findByDisplayValue(receipt)).toBeInTheDocument();
    expect(posts()).toHaveLength(1);
    const options = posts()[0][1]!;
    expect(JSON.parse(String(options.body))).toEqual({ invitee_id: 'pending' });
    const headers = new Headers(options.headers);
    expect(headers.get('x-ohc-expected-user')).toBe(owner.userId);
    expect(headers.get('x-ohc-expected-tenant')).toBe(owner.tenantId);
    const share = screen.getByRole('link', { name: 'Share on X' });
    const url = new URL(share.getAttribute('href')!);
    expect(url.origin + url.pathname).toBe('https://twitter.com/intent/tweet');
    expect(url.searchParams.get('text')).toBe(`Join me on OmniSolo OneHumanCorp: ${receipt}`);
    expect(share).toHaveAttribute('target', '_blank');
    expect(share).toHaveAttribute('rel', 'noopener noreferrer');
    expect(localStorage.getItem(key)).not.toContain(receipt);
  });
  it.each(['rejected', 'network', 'malformed'])('%s generation cannot invent an invitation or allow blind retry', async failure => {
    inviteReply = async () => { if (failure === 'network') throw new Error('connection lost'); return failure === 'malformed' ? new Response('invalid JSON') : Response.json({ error: 'unavailable' }, { status: 503 }); };
    render(<DashboardViralInviteWidget />); await create();
    expect(await screen.findByRole('status', { name: 'Dashboard invitation status' })).toHaveTextContent('could not be confirmed');
    expect(document.querySelector('#dashboard-invite-link')).toBeNull();
    expect(screen.queryByRole('link', { name: 'Share on X' })).not.toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Get My Invite Link' })).toBeDisabled();
    expect(document.body.textContent).not.toContain('/invite/untrusted-display-name');
    expect(posts()).toHaveLength(1);
  });
  it('awaits the actual clipboard promise and preserves denial feedback', async () => {
    render(<DashboardViralInviteWidget />); await create(); await screen.findByDisplayValue(receipt);
    const pending = deferred<void>(); writeText.mockReturnValueOnce(pending.promise);
    fireEvent.click(screen.getByRole('button', { name: 'Copy' }));
    expect(writeText).toHaveBeenCalledWith(receipt);
    expect(screen.queryByRole('button', { name: 'Copied!' })).not.toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Copying…' })).toBeDisabled();
    await act(async () => pending.reject(new DOMException('denied', 'NotAllowedError')));
    expect(screen.getByRole('alert')).toHaveTextContent('Copy failed');
    expect(screen.queryByRole('button', { name: 'Copied!' })).not.toBeInTheDocument();
    fireEvent.click(screen.getByRole('button', { name: 'Copy' }));
    expect(await screen.findByRole('button', { name: 'Copied!' })).toBeInTheDocument();
  });
  it('keeps a single in-flight request and retains its hold after remount', async () => {
    const pending = deferred<Response>(); inviteReply = () => pending.promise;
    const first = render(<DashboardViralInviteWidget />); await create();
    fireEvent.click(screen.getByRole('button', { name: 'Generating...' }));
    expect(posts()).toHaveLength(1); first.unmount();
    render(<DashboardViralInviteWidget />);
    expect(await screen.findByRole('status', { name: 'Dashboard invitation status' })).toHaveTextContent('previous invitation request is unconfirmed');
    await act(async () => pending.resolve(Response.json({ invite_link: receipt })));
    expect(document.querySelector('#dashboard-invite-link')).toBeNull(); expect(posts()).toHaveLength(1);
  });
  it('retires a previous owner link and late copy acknowledgement', async () => {
    render(<DashboardViralInviteWidget />); await create(); await screen.findByDisplayValue(receipt);
    const pending = deferred<void>(); writeText.mockReturnValueOnce(pending.promise);
    fireEvent.click(screen.getByRole('button', { name: 'Copy' }));
    act(() => window.dispatchEvent(new Event('omnisolo_auth_changed')));
    await act(async () => pending.resolve());
    expect(document.querySelector('#dashboard-invite-link')).toBeNull();
    expect(screen.queryByRole('link', { name: 'Share on X' })).not.toBeInTheDocument();
    expect(screen.queryByRole('button', { name: 'Copied!' })).not.toBeInTheDocument();
    expect(screen.getByRole('status', { name: 'Dashboard invitation status' })).toHaveTextContent('session changed');
  });
  it('does not create when the verified identity is unavailable', async () => {
    identityReply = async () => Response.json({ error: 'unauthorized' }, { status: 401 });
    render(<DashboardViralInviteWidget />);
    expect(await screen.findByRole('status', { name: 'Dashboard invitation status' })).toHaveTextContent('No invitation was requested');
    expect(screen.getByRole('button', { name: 'Get My Invite Link' })).toBeDisabled(); expect(posts()).toHaveLength(0);
  });
});

it('preserves WhatsApp sharing only for a verified invitation receipt', async () => {
  render(<DashboardViralInviteWidget />);
  expect(screen.queryByRole('link', { name: 'Share on WhatsApp' })).toBeNull();
  await create();
  const share = await screen.findByRole('link', { name: 'Share on WhatsApp' });
  const url = new URL(share.getAttribute('href')!);
  expect(url.origin + url.pathname).toBe('https://wa.me/');
  expect(url.searchParams.get('text')).toBe(`Join me on OmniSolo OneHumanCorp: ${receipt}`);
  expect(share).toHaveAttribute('rel', 'noopener noreferrer');
});
