/* @vitest-environment jsdom */
import { act, cleanup, fireEvent, render, screen, waitFor, within } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import '@testing-library/jest-dom/vitest';
import { notifyQueueIdentityChange } from '@/lib/sync/queueIdentity';
import NewServicePage from './page';
import { installOnboardingLocks } from '../../onboarding/testLocks';
import { readOwnedOnboardingItem } from '../../onboarding/draftSession';

const push = vi.hoisted(() => vi.fn());
vi.mock('next/navigation', () => ({ useRouter: () => ({ push }) }));
const originalOwner = { userId: 'service-owner-a', tenantId: 'service-tenant-a' };
let currentOwner = { ...originalOwner };
const id = 'aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa';
const receipt = { success: true, service_id: id, error: null };
let create: () => Promise<Response>;
const mutations = () => vi.mocked(fetch).mock.calls.filter(([url]) => url === '/api/v1/booking/services');
async function openPage() {
  const view = render(<NewServicePage />);
  await waitFor(() => expect(screen.getByRole('button', { name: 'Save Service' })).toBeEnabled());
  return view;
}
async function enterService(price = '12.34') {
  fireEvent.change(screen.getByPlaceholderText('e.g. Weekly Music Tutoring'), { target: { value: '  Piano lesson  ' } });
  fireEvent.change(screen.getByPlaceholderText('Describe the service...'), { target: { value: 'A one-hour lesson.' } });
  fireEvent.change(screen.getByPlaceholderText('0.00'), { target: { value: price } });
}
beforeEach(() => {
  installOnboardingLocks(); localStorage.clear(); currentOwner = { ...originalOwner }; notifyQueueIdentityChange();
  push.mockClear(); create = async () => Response.json(receipt);
  vi.stubGlobal('fetch', vi.fn(async (url: string) => {
    if (url.endsWith('/session-identity')) return Response.json({ ...currentOwner, expiresAt: Date.now() + 60_000 });
    if (url === '/api/v1/booking/services') return create();
    return Response.json({ error: 'unexpected route' }, { status: 404 });
  }));
});
afterEach(() => { cleanup(); vi.unstubAllGlobals(); vi.restoreAllMocks(); });

describe('Acknowledged service creation', () => {
  it('sends only supported fields and waits for the actual receipt before claiming success', async () => {
    let acknowledge!: (response: Response) => void;
    create = () => new Promise(resolve => { acknowledge = resolve; });
    await openPage(); await enterService();
    fireEvent.click(screen.getByRole('button', { name: 'Save Service' }));
    await waitFor(() => expect(mutations()).toHaveLength(1));
    expect(screen.queryByText('Service Saved!')).not.toBeInTheDocument();
    expect(push).not.toHaveBeenCalled();
    const options = mutations()[0][1]!;
    expect(JSON.parse(String(options.body))).toEqual({ title: 'Piano lesson', description: 'A one-hour lesson.', price_cents: 1234 });
    expect(new Headers(options.headers).get('x-ohc-expected-user')).toBe(originalOwner.userId);
    fireEvent.click(screen.getByRole('button', { name: /Saving/ }));
    expect(mutations()).toHaveLength(1);
    await act(async () => acknowledge(Response.json(receipt)));
    expect(await screen.findByText('Service Saved!')).toBeVisible();
    expect(screen.getByText(id)).toBeVisible();
    expect(push).toHaveBeenCalledWith('/dashboard');
  });

  it('validates an empty title before a request', async () => {
    await openPage(); fireEvent.click(screen.getByRole('button', { name: 'Save Service' }));
    expect(await screen.findByText('Enter a service title before saving.')).toBeVisible();
    expect(mutations()).toHaveLength(0);
  });
  it.each(['-1', '1.001', '1e3', 'NaN', '10000000.01'])('rejects invalid price %s without sending it', async price => {
    await openPage(); await enterService(price); fireEvent.click(screen.getByRole('button', { name: 'Save Service' }));
    expect(await screen.findByText(/Enter a price from/)).toBeVisible();
    expect(mutations()).toHaveLength(0);
  });
  it('supports a free service without inventing a recurring payment', async () => {
    await openPage(); await enterService('0');
    expect(screen.getByRole('checkbox', { name: 'Recurring payment' })).toBeDisabled();
    expect(screen.getByText(/Recurring payments are not supported/)).toBeVisible();
    fireEvent.click(screen.getByRole('button', { name: 'Save Service' }));
    await screen.findByText('Service Saved!');
    expect(JSON.parse(String(mutations()[0][1]?.body)).price_cents).toBe(0);
    expect(JSON.parse(String(mutations()[0][1]?.body))).not.toHaveProperty('frequency');
  });
  it('copies only the supplied title instead of inventing a tutoring description', async () => {
    const user = userEvent.setup(); await openPage();
    fireEvent.change(screen.getByLabelText('Service Title'), { target: { value: 'Garden consultation' } });
    await user.click(screen.getByRole('button', { name: 'Copy title' }));
    expect(screen.getByLabelText('Description')).toHaveValue('Garden consultation');
    expect(mutations()).toHaveLength(0);
  });
  it('keeps a definite validation rejection editable and does not claim success', async () => {
    create = async () => Response.json({ success: false, service_id: null, error: 'invalid service fields' }, { status: 400 });
    await openPage(); await enterService(); fireEvent.click(screen.getByRole('button', { name: 'Save Service' }));
    expect(await screen.findByText(/Service was not saved/)).toBeVisible();
    expect(screen.getByRole('button', { name: 'Save Service' })).toBeEnabled();
    expect(push).not.toHaveBeenCalled(); expect(mutations()).toHaveLength(1);
  });
  it.each(['network', 'invalid-json', 'malformed-receipt', 'server-error'])('holds an ambiguous %s result across reloads without retry', async failure => {
    create = async () => {
      if (failure === 'network') throw new TypeError('connection ended');
      if (failure === 'invalid-json') return new Response('not json', { status: 200 });
      if (failure === 'malformed-receipt') return Response.json({ success: true });
      return Response.json({ error: 'internal error' }, { status: 500 });
    };
    const view = await openPage(); await enterService(); fireEvent.click(screen.getByRole('button', { name: 'Save Service' }));
    expect(await screen.findByText(/Could not confirm whether the service was saved/)).toBeVisible();
    expect(screen.queryByText('Service Saved!')).not.toBeInTheDocument(); expect(push).not.toHaveBeenCalled();
    view.unmount(); render(<NewServicePage />);
    expect(await screen.findByText(/Could not confirm whether the service was saved/)).toBeVisible();
    expect(screen.getByRole('button', { name: 'Save Service' })).toBeDisabled();
    expect(screen.getByLabelText('Service Title')).toHaveValue('  Piano lesson  ');
    expect(mutations()).toHaveLength(1);
  });
  it('does not send when the durable pending marker cannot be written', async () => {
    await openPage(); await enterService();
    const write = localStorage.setItem;
    localStorage.setItem = (key, value) => {
      if (key.endsWith(':service-create-request')) throw new DOMException('full', 'QuotaExceededError');
      return write.call(localStorage, key, value);
    };
    try {
      fireEvent.click(screen.getByRole('button', { name: 'Save Service' }));
      expect(await screen.findByText(/Could not record this request locally/)).toBeVisible();
      expect(mutations()).toHaveLength(0); expect(push).not.toHaveBeenCalled();
    } finally { localStorage.setItem = write; }
  });
  it('clears an old owner view and fences a late response body after an account switch', async () => {
    let finish!: () => void;
    create = async () => new Response(new ReadableStream({ start(controller) {
      finish = () => { controller.enqueue(new TextEncoder().encode(JSON.stringify(receipt))); controller.close(); };
    } }), { headers: { 'content-type': 'application/json' } });
    await openPage(); await enterService(); fireEvent.click(screen.getByRole('button', { name: 'Save Service' }));
    await waitFor(() => expect(finish).toBeDefined());
    await act(async () => { currentOwner = { userId: 'service-owner-b', tenantId: 'service-tenant-b' }; notifyQueueIdentityChange(); });
    await waitFor(() => expect(screen.getByLabelText('Service Title')).toHaveValue(''));
    await act(async () => finish());
    expect(screen.queryByText('Service Saved!')).not.toBeInTheDocument(); expect(push).not.toHaveBeenCalled();
    await act(async () => { currentOwner = { ...originalOwner }; notifyQueueIdentityChange(); });
    expect(await screen.findByText(/Could not confirm whether the service was saved/)).toBeVisible();
    expect(screen.getByRole('button', { name: 'Save Service' })).toBeDisabled();
    expect(mutations()).toHaveLength(1);
  });
  it('serializes separate views so they cannot dispatch the same unresolved service twice', async () => {
    const replies: Array<(response: Response) => void> = [];
    create = () => new Promise(resolve => replies.push(resolve));
    const first = await openPage(); await enterService();
    const second = render(<NewServicePage />);
    await waitFor(() => expect(within(second.container).getByRole('button', { name: 'Save Service' })).toBeEnabled());
    fireEvent.click(within(first.container).getByRole('button', { name: 'Save Service' }));
    fireEvent.click(within(second.container).getByRole('button', { name: 'Save Service' }));
    await waitFor(() => expect(mutations().length).toBeGreaterThan(0));
    await act(async () => { await new Promise(resolve => setTimeout(resolve, 10)); });
    try {
      expect(mutations()).toHaveLength(1);
      expect(readOwnedOnboardingItem('service-create-request')).toContain('unknown');
    } finally {
      await act(async () => { replies.forEach(resolve => resolve(Response.json(receipt))); });
    }
    await waitFor(() => expect(mutations()).toHaveLength(1));
  });

  it('does not let a stale saved view clear another unresolved request', async () => {
    const first = await openPage(); await enterService();
    fireEvent.click(within(first.container).getByRole('button', { name: 'Save Service' }));
    await within(first.container).findByText('Service Saved!');
    const second = render(<NewServicePage />);
    await within(second.container).findByText('Previously acknowledged service');
    fireEvent.click(within(first.container).getByRole('button', { name: 'Add another service' }));
    await waitFor(() => expect(within(first.container).getByRole('button', { name: 'Save Service' })).toBeEnabled());
    await enterService();
    let acknowledge!: (value: Response) => void;
    create = () => new Promise(resolve => { acknowledge = resolve; });
    fireEvent.click(within(first.container).getByRole('button', { name: 'Save Service' }));
    await waitFor(() => expect(mutations()).toHaveLength(2));
    fireEvent.click(within(second.container).getByRole('button', { name: 'Add another service' }));
    await act(async () => { await new Promise(resolve => setTimeout(resolve, 10)); });
    expect(readOwnedOnboardingItem('service-create-request')).toContain('unknown');
    const nextId = 'bbbbbbbb-bbbb-4bbb-8bbb-bbbbbbbbbbbb';
    await act(async () => acknowledge(Response.json({ ...receipt, service_id: nextId })));
    await waitFor(() => expect(readOwnedOnboardingItem('service-create-request')).toContain(nextId));
    expect(mutations()).toHaveLength(2);
  });

  it('releases the request lock on timeout and rejects a late owner-read dispatch', async () => {
    await openPage(); await enterService();
    let resolveIdentity!: (value: Response) => void;
    vi.mocked(fetch).mockImplementation(async url => {
      if (String(url).endsWith('/session-identity')) return new Promise(resolve => { resolveIdentity = resolve; });
      return Response.json(receipt);
    });
    let expire!: () => void;
    const schedule = window.setTimeout.bind(window);
    vi.spyOn(window, 'setTimeout').mockImplementation((handler, delay, ...args) => {
      if (delay === 30_000 && typeof handler === 'function') expire = () => handler(...args);
      return schedule(handler, delay, ...args);
    });
    fireEvent.click(screen.getByRole('button', { name: 'Save Service' }));
    await waitFor(() => expect(resolveIdentity).toBeDefined());
    await act(async () => expire());
    try {
      expect(await screen.findByText(/timed out before it was sent/)).toBeVisible();
    } finally {
      await act(async () => resolveIdentity(Response.json({ ...currentOwner, expiresAt: Date.now() + 60_000 })));
    }
    expect(mutations()).toHaveLength(0);
    expect(readOwnedOnboardingItem('service-create-request')).toBeNull();
  });

  it.each([
    ['Service Title', 'x'.repeat(201), '200 characters'],
    ['Description', 'x'.repeat(10001), '10,000 characters'],
  ])('keeps oversized %s out of the request', async (label, value, limit) => {
    await openPage(); await enterService();
    fireEvent.change(screen.getByLabelText(label), { target: { value } });
    fireEvent.click(screen.getByRole('button', { name: 'Save Service' }));
    expect(await screen.findByText(new RegExp(limit))).toBeVisible();
    expect(mutations()).toHaveLength(0);
  });
  it('matches server character counting and the maximum integer-cent boundary', async () => {
    await openPage(); await enterService('10000000.00');
    fireEvent.change(screen.getByLabelText('Service Title'), { target: { value: '🎹'.repeat(200) } });
    fireEvent.click(screen.getByRole('button', { name: 'Save Service' }));
    await screen.findByText('Service Saved!');
    const body = JSON.parse(String(mutations()[0][1]?.body));
    expect([...body.title]).toHaveLength(200);
    expect(body.price_cents).toBe(1_000_000_000);
  });

  it('retains a confirmed service receipt if navigation fails after acceptance', async () => {
    push.mockImplementationOnce(() => { throw new Error('Navigation unavailable'); });
    const view = await openPage(); await enterService();
    fireEvent.click(screen.getByRole('button', { name: 'Save Service' }));
    expect(await screen.findByText('Service Saved!')).toBeVisible();
    expect(await screen.findByText(/service was saved.*navigation failed/i)).toBeVisible();
    expect(screen.queryByText(/Could not confirm whether/)).not.toBeInTheDocument();
    expect(readOwnedOnboardingItem('service-create-request')).toContain(id);
    view.unmount(); render(<NewServicePage />);
    await screen.findByText('Previously acknowledged service');
    expect(mutations()).toHaveLength(1);
  });

});
