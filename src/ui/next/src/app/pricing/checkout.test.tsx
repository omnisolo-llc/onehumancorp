import React from 'react';
import { act, fireEvent, render, screen, waitFor } from '@testing-library/react';
import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import PricingPage from './page';

vi.mock('next/navigation', () => ({ useRouter: () => ({ push: vi.fn() }) }));
vi.mock('../../components/TooltipRegistry', () => ({ WithTooltip: ({ children }: { children: React.ReactNode }) => <>{children}</> }));
vi.mock('../components/PoweredByOmniSolo', () => ({ PoweredByOmniSolo: () => null }));
vi.mock('../components/ViralTrialExtensionWidget', () => ({ ViralTrialExtensionWidget: () => null }));
const location = window.location;
const validUrl = 'https://checkout.stripe.com/c/pay/cs_test_fixture_only';
let checkout: () => Promise<Response>;
beforeEach(() => {
  Object.defineProperty(window, 'location', { configurable: true, writable: true, value: { ...location, href: 'https://app.example.test/pricing' } });
  checkout = async () => Response.json({ checkout_url: validUrl });
  vi.spyOn(window, 'alert').mockImplementation(() => {});
  vi.spyOn(console, 'error').mockImplementation(() => {});
  vi.stubGlobal('fetch', vi.fn(async input => {
    if (input === '/api/v1/billing/my-plan') return Response.json({ current_plan: 'Free' });
    if (input === '/api/v1/billing/create-checkout-session') return checkout();
    throw new Error(`Unexpected request: ${String(input)}`);
  }));
});
afterEach(() => {
  Object.defineProperty(window, 'location', { configurable: true, writable: true, value: location });
  vi.unstubAllGlobals(); vi.restoreAllMocks();
});

it.each([
  ['nonterminal acceptance', () => Promise.resolve(Response.json({ checkout_url: validUrl }, { status: 202 }))],
  ['failure status', () => Promise.resolve(Response.json({ checkout_url: validUrl }, { status: 503 }))],
  ['network failure', () => Promise.reject(new Error('offline'))],
  ['malformed JSON', () => Promise.resolve(new Response('{invalid'))],
  ['missing URL', () => Promise.resolve(Response.json({}))],
  ['non-string URL', () => Promise.resolve(Response.json({ checkout_url: 123 }))],
  ['untrusted host', () => Promise.resolve(Response.json({ checkout_url: 'https://checkout.stripe.com.attacker.test/pay' }))],
  ['script URL', () => Promise.resolve(Response.json({ checkout_url: 'javascript:alert(1)' }))],
  ['insecure URL', () => Promise.resolve(Response.json({ checkout_url: 'http://checkout.stripe.com/pay' }))],
  ['URL credentials', () => Promise.resolve(Response.json({ checkout_url: 'https://user:secret@checkout.stripe.com/pay' }))],
  ['contradictory result', () => Promise.resolve(Response.json({ success: false, checkout_url: validUrl }))],
  ['error result', () => Promise.resolve(Response.json({ error: 'unavailable', checkout_url: validUrl }))],
] as const)('keeps pricing open with a recoverable error for %s', async (_name, response) => {
  checkout = response;
  await act(async () => { render(<PricingPage />); });
  await act(async () => fireEvent.click(screen.getByRole('button', { name: 'Upgrade to Starter via Stripe' })));
  expect(await screen.findByRole('alert')).toHaveTextContent('Checkout is unavailable. Your plan has not changed. Please try again.');
  expect(window.location.href).toBe('https://app.example.test/pricing');
  expect(screen.getByRole('button', { name: 'Upgrade to Starter via Stripe' })).toBeEnabled();
  checkout = async () => Response.json({ checkout_url: validUrl });
  await act(async () => fireEvent.click(screen.getByRole('button', { name: 'Upgrade to Starter via Stripe' })));
  await waitFor(() => expect(window.location.href).toBe(validUrl));
  expect(screen.queryByRole('alert')).not.toBeInTheDocument();
});

it('does not create duplicate checkout sessions during a pending request', async () => {
  let resolve!: (response: Response) => void;
  checkout = () => new Promise(done => { resolve = done; });
  await act(async () => { render(<PricingPage />); });
  const button = screen.getByRole('button', { name: 'Upgrade to Starter via Stripe' });
  fireEvent.click(button); fireEvent.click(button);
  expect(vi.mocked(fetch).mock.calls.filter(([input]) => input === '/api/v1/billing/create-checkout-session')).toHaveLength(1);
  await act(async () => resolve(Response.json({ checkout_url: validUrl })));
  expect(window.location.href).toBe(validUrl);
});
