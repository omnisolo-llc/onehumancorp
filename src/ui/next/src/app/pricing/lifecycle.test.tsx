import React from 'react';
import { act, fireEvent, render, screen, waitFor } from '@testing-library/react';
import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import PricingPage from './page';
import { invalidateOnboardingSession } from '../onboarding/draftSession';
import { invalidateQueueOwner, QUEUE_IDENTITY_EPOCH_KEY, readQueueOwner } from '@/lib/sync/queueIdentity';
vi.mock('next/navigation', () => ({ useRouter: () => ({ push: vi.fn() }) }));
vi.mock('../../components/TooltipRegistry', () => ({ WithTooltip: ({ children }: { children: React.ReactNode }) => <>{children}</> }));
vi.mock('../components/PoweredByOmniSolo', () => ({ PoweredByOmniSolo: () => null }));
vi.mock('../components/ViralTrialExtensionWidget', () => ({ ViralTrialExtensionWidget: () => null }));
const location = window.location;
const ownerA = { userId: 'owner-a', tenantId: 'tenant-a' };
const ownerB = { userId: 'owner-b', tenantId: 'tenant-b' };
let owner = ownerA;
let reply: () => Promise<Response>;
const deferred = <T,>() => { let resolve!: (value: T) => void; const promise = new Promise<T>(yes => { resolve = yes; }); return { promise, resolve }; };
beforeEach(() => {
  localStorage.clear(); invalidateQueueOwner(); invalidateOnboardingSession(); owner = ownerA;
  Object.defineProperty(window, 'location', { configurable: true, writable: true, value: { ...location, href: 'https://app.example.test/pricing' } });
  reply = async () => Response.json({});
  vi.stubGlobal('fetch', vi.fn(async url => {
    if (url === '/api/v1/auth/session-identity') return Response.json({ ...owner, expiresAt: Date.now() + 60_000 });
    if (url === '/api/v1/billing/my-plan') return Response.json({ current_plan: 'Pro' });
    return reply();
  }));
});
afterEach(() => { Object.defineProperty(window, 'location', { configurable: true, writable: true, value: location }); vi.unstubAllGlobals(); vi.restoreAllMocks(); });
const flows = [
  ['portal', 'Manage Plan & Billing', { url: 'https://billing.stripe.com/p/session/private-owner-a' }],
  ['checkout', 'Upgrade to Business via Stripe', { checkout_url: 'https://checkout.stripe.com/c/pay/private-owner-a' }],
] as const;
for (const [flow, buttonName, receipt] of flows) {
  it.each(['unmount', 'auth', 'storage', 'null-storage', 'pagehide', 'verified-owner', 'silent-storage'])('%s retires a late ' + flow + ' response without navigating to the previous owner', async change => {
    const body = deferred<typeof receipt>();
    const readBody = vi.fn(() => body.promise);
    reply = async () => ({ status: 200, ok: true, json: readBody }) as unknown as Response;
    let view!: ReturnType<typeof render>;
    await act(async () => { view = render(<PricingPage />); });
    await act(async () => fireEvent.click(screen.getByRole('button', { name: buttonName })));
    await waitFor(() => expect(readBody).toHaveBeenCalled());
    await act(async () => {
      if (change === 'unmount') view.unmount();
      else if (change === 'auth') window.dispatchEvent(new Event('omnisolo_auth_changed'));
      else if (change === 'storage') window.dispatchEvent(new StorageEvent('storage', { key: QUEUE_IDENTITY_EPOCH_KEY }));
      else if (change === 'null-storage') window.dispatchEvent(new StorageEvent('storage', { key: null }));
      else if (change === 'pagehide') window.dispatchEvent(new Event('pagehide'));
      else if (change === 'verified-owner') { owner = ownerB; await readQueueOwner(); }
      else localStorage.setItem(QUEUE_IDENTITY_EPOCH_KEY, 'different-login');
    });
    await act(async () => body.resolve(receipt));
    expect(window.location.href).toBe('https://app.example.test/pricing');
  });
}

for (const [flow, buttonName, receipt] of flows) {
  it('reverifies the same owner after the local identity cache expires before opening ' + flow, async () => {
    reply = async () => Response.json(receipt);
    await act(async () => { render(<PricingPage />); });
    const later = Date.now() + 120_000;
    vi.spyOn(Date, 'now').mockReturnValue(later);
    await act(async () => fireEvent.click(screen.getByRole('button', { name: buttonName })));
    expect(window.location.href).toBe('url' in receipt ? receipt.url : receipt.checkout_url);
  });
}
