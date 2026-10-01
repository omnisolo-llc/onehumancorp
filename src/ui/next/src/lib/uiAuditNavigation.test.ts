import { describe, expect, it, vi } from 'vitest';
import type { Page } from '@playwright/test';
import { createAuditNavigation } from '../../../../e2e/support/ui_audit_navigation';

function fixture() {
  const listeners = new Map<string, ((event: unknown) => void)[]>();
  const context = { on: vi.fn((name: string, fn: (event: unknown) => void) => { listeners.set(name, [...listeners.get(name) || [], fn]); }) };
  let current = 'about:blank';
  const request = { get: vi.fn().mockResolvedValue({ status: () => 200 }) };
  const goto = vi.fn(async (url: string) => { current = url; return { status: (): number => 200 }; });
  const fill = vi.fn();
  const waitForFunction = vi.fn().mockResolvedValue({ dispose: vi.fn() });
  const page = { context: () => context, request, goto, url: () => current,
    waitForFunction, waitForLoadState: vi.fn().mockResolvedValue(undefined), waitForTimeout: vi.fn().mockResolvedValue(undefined), evaluate: fill } as unknown as Page;
  const authenticate = vi.fn().mockResolvedValue(undefined);
  const navigate = createAuditNavigation('https://fixture.test', authenticate);
  const emit = (name: string, event: unknown) => listeners.get(name)?.forEach(fn => fn(event));
  return { page, context, request, goto, fill, waitForFunction, authenticate, navigate, emit,
    setUrl: (url: string) => { current = url; } };
}

const response = (url: string, status: number) => ({ url: () => url, status: () => status });
const request = (url: string, method: string) => ({ url: () => url, method: () => method });

describe('read navigation in the isolated audit session', () => {
  it('authenticates once without a separate API probe for every reset', async () => {
    const f = fixture();
    await f.navigate(f.page, '/dashboard');
    await f.navigate(f.page, '/dashboard');
    expect(f.authenticate).toHaveBeenCalledTimes(1);
    expect(f.request.get).not.toHaveBeenCalled();
    expect(f.goto).toHaveBeenCalledTimes(2);
    expect(f.fill).not.toHaveBeenCalled();
  });

  it('renews after an observed same-origin logout before another route is read', async () => {
    const f = fixture();
    await f.navigate(f.page, '/dashboard');
    f.emit('request', request('https://fixture.test/api/v1/auth/logout', 'POST'));
    await f.navigate(f.page, '/dashboard');
    expect(f.authenticate).toHaveBeenCalledTimes(2);
  });

  it('renews an observed same-origin401 but ignores another origin', async () => {
    const f = fixture();
    await f.navigate(f.page, '/dashboard');
    f.emit('response', response('https://outside.test/api/status', 401));
    await f.navigate(f.page, '/dashboard');
    expect(f.authenticate).toHaveBeenCalledTimes(1);
    f.emit('response', response('https://fixture.test/api/v1/agent-feed', 401));
    await f.navigate(f.page, '/dashboard');
    expect(f.authenticate).toHaveBeenCalledTimes(2);
  });

  it('repeats only the read navigation once after a401 document response', async () => {
    const f = fixture();
    f.goto.mockResolvedValueOnce({ status: () => 401 });
    await f.navigate(f.page, '/dashboard');
    expect(f.authenticate).toHaveBeenCalledTimes(2);
    expect(f.goto).toHaveBeenCalledTimes(2);
    expect(f.fill).not.toHaveBeenCalled();
  });

  it('fails visibly after persistent denial instead of auditing an unauthenticated page', async () => {
    const f = fixture();
    f.goto.mockResolvedValue({ status: () => 401 });
    await expect(f.navigate(f.page, '/dashboard')).rejects.toThrow(/authentication/i);
    expect(f.authenticate).toHaveBeenCalledTimes(2);
    expect(f.goto).toHaveBeenCalledTimes(2);
    expect(f.fill).not.toHaveBeenCalled();
  });

  it('recovers a login redirect before describing controls', async () => {
    const f = fixture();
    f.goto.mockImplementationOnce(async () => { f.setUrl('https://fixture.test/login?next=%2Fdashboard'); return { status: () => 200 }; });
    await f.navigate(f.page, '/dashboard');
    expect(f.authenticate).toHaveBeenCalledTimes(2);
    expect(f.goto).toHaveBeenCalledTimes(2);
    expect(f.fill).not.toHaveBeenCalled();
  });

  it('renews after a late logout response without trusting another-origin logout', async () => {
    const f = fixture();
    await f.navigate(f.page, '/dashboard');
    f.emit('request', request('https://outside.test/api/v1/auth/logout', 'POST'));
    await f.navigate(f.page, '/dashboard');
    expect(f.authenticate).toHaveBeenCalledTimes(1);
    f.emit('response', response('https://fixture.test/api/v1/auth/logout', 200));
    await f.navigate(f.page, '/dashboard');
    expect(f.authenticate).toHaveBeenCalledTimes(2);
  });

  it('does not mistake an intentionally audited login page for lost authentication', async () => {
    const f = fixture();
    await f.navigate(f.page, '/login');
    expect(f.authenticate).toHaveBeenCalledTimes(1);
    expect(f.goto).toHaveBeenCalledTimes(1);
    expect(f.fill).not.toHaveBeenCalled();
  });

  it('fails after a persistent login redirect and does not fill or click controls', async () => {
    const f = fixture();
    f.goto.mockImplementation(async () => { f.setUrl('https://fixture.test/login?next=%2Fdashboard'); return { status: () => 200 }; });
    await expect(f.navigate(f.page, '/dashboard')).rejects.toThrow(/authentication/i);
    expect(f.authenticate).toHaveBeenCalledTimes(2);
    expect(f.goto).toHaveBeenCalledTimes(2);
    expect(f.fill).not.toHaveBeenCalled();
  });

  it('does not retry a failed authentication or failed navigation', async () => {
    const f = fixture();
    f.authenticate.mockRejectedValueOnce(new Error('auth unavailable'));
    await expect(f.navigate(f.page, '/dashboard')).rejects.toThrow('auth unavailable');
    expect(f.goto).not.toHaveBeenCalled();
    f.goto.mockRejectedValueOnce(new Error('navigation failed'));
    await expect(f.navigate(f.page, '/dashboard')).rejects.toThrow('navigation failed');
    expect(f.goto).toHaveBeenCalledTimes(1);
    expect(f.fill).not.toHaveBeenCalled();
  });
});


it('waits for visible initialization to settle before returning a page for discovery', async () => {
  const f = fixture();
  let release!: (handle: { dispose: () => Promise<void> }) => void;
  f.waitForFunction.mockImplementation(() => new Promise<{ dispose: () => Promise<void> }>(resolve => { release = resolve; }));
  const complete = vi.fn();
  const navigating = f.navigate(f.page, '/inbox').then(complete);
  await new Promise(resolve => setTimeout(resolve, 5));
  expect(complete).not.toHaveBeenCalled();
  expect(f.goto).toHaveBeenCalledTimes(1);
  release({ dispose: async () => undefined }); await navigating;
  expect(complete).toHaveBeenCalledTimes(1);
  expect(f.goto).toHaveBeenCalledTimes(1);
  expect(f.fill).not.toHaveBeenCalled();
});

it('fails permanent initialization rather than repeating a navigation or user action', async () => {
  const f = fixture();
  f.waitForFunction.mockRejectedValue(new Error('initialization did not settle'));
  await expect(f.navigate(f.page, '/inbox')).rejects.toThrow('initialization did not settle');
  expect(f.goto).toHaveBeenCalledTimes(1);
  expect(f.authenticate).toHaveBeenCalledTimes(1);
  expect(f.fill).not.toHaveBeenCalled();
});
