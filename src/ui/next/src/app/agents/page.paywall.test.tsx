import { act, cleanup, fireEvent, render, screen, waitFor } from '@testing-library/react';
import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import AgentsPage from './page';
import { TooltipProvider } from '@/components/TooltipRegistry';
import { invalidateQueueOwner, readQueueOwner } from '@/lib/sync/queueIdentity';
import { invalidateOnboardingSession } from '../onboarding/draftSession';

// Exercise the actual page, plan hook, canonical identity and business readers.
// Only HTTP is supplied by fixtures; no plan or identity implementation is mocked.
const owner = { userId: 'paywall-owner', tenantId: 'paywall-tenant' };
let currentOwner = owner;
let currentPlan: unknown = 'Pro';
let identityReply: (() => Promise<Response>) | null;
let planReply: (() => Promise<Response>) | null;
const fetchMock = vi.fn<typeof fetch>();
const toggle = () => screen.getByRole('button', { name: 'Toggle Pro Mode' });
const paywall = () => screen.queryByRole('heading', { name: 'Upgrade to Pro' });
function deferred<T>() { let resolve!: (value: T) => void; const promise = new Promise<T>(done => { resolve = done; }); return { promise, resolve }; }
const identity = () => Response.json({ ...currentOwner, expiresAt: Date.now() + 60_000 });
async function open() {
  render(<TooltipProvider><AgentsPage /></TooltipProvider>);
  await waitFor(() => expect(fetchMock.mock.calls.some(([url]) => url === '/api/v1/billing/my-plan')).toBe(true));
  await act(async () => {});
}
beforeEach(() => {
  localStorage.clear(); invalidateQueueOwner(); invalidateOnboardingSession(false);
  currentOwner = owner; currentPlan = 'Pro'; identityReply = null; planReply = null;
  Object.defineProperty(navigator, 'onLine', { configurable: true, value: true });
  fetchMock.mockReset();
  fetchMock.mockImplementation(async (input) => {
    const url = String(input);
    if (url === '/api/v1/auth/session-identity') return identityReply ? identityReply() : identity();
    if (url === '/api/v1/billing/my-plan') return planReply ? planReply() : Response.json({ current_plan: currentPlan });
    if (url === '/api/v1/agents/execution-policy') return Response.json({ available: false, mode: 'text_analysis', workspace_access: false, tools: [], policy: null });
    if (url === '/api/v1/agents/approvals' || url === '/api/v1/agents/approvals/activity') return Response.json({ pending_approvals: [] });
    if (url === '/api/v1/agents/workflows') return Response.json({ workflows: [], next_cursor: null });
    throw new Error(`Unexpected HTTP boundary: ${url}`);
  });
  vi.stubGlobal('fetch', fetchMock);
});
afterEach(() => { cleanup(); invalidateQueueOwner(); invalidateOnboardingSession(false); vi.unstubAllGlobals(); });

it.each(['Pro', 'Business'])('does not open a false upgrade prompt during a %s owner recheck', async plan => {
  currentPlan = plan; await open();
  await waitFor(() => expect(toggle()).toHaveAttribute('aria-pressed', 'true'));
  const pending = deferred<Response>(); identityReply = () => pending.promise;
  let checking!: Promise<unknown>; act(() => { checking = readQueueOwner(); });
  // Paid authority must still be withheld while canonical verification is pending.
  expect(toggle()).toHaveAttribute('aria-pressed', 'false');
  fireEvent.click(toggle());
  expect(paywall()).not.toBeInTheDocument();
  expect(toggle()).toBeDisabled();
  await act(async () => { pending.resolve(identity()); await checking; });
  expect(toggle()).toHaveAttribute('aria-pressed', 'true');
  expect(toggle()).toBeEnabled();
  expect(paywall()).not.toBeInTheDocument();
  fireEvent.click(toggle()); expect(paywall()).not.toBeInTheDocument();
  expect(fetchMock.mock.calls.filter(([, options]) => options?.method === 'POST')).toHaveLength(0);
});

it.each(['Free', 'Starter'])('keeps a verified %s plan gated and trial activation unavailable', async plan => {
  currentPlan = plan; await open();
  await waitFor(() => expect(toggle()).toBeEnabled());
  expect(toggle()).toHaveAttribute('aria-pressed', 'false');
  fireEvent.click(toggle()); expect(paywall()).toBeVisible();
  fireEvent.click(screen.getByRole('button', { name: 'Check trial availability' }));
  expect(screen.getByRole('alert')).toHaveTextContent('durable grant is not verified');
  expect(toggle()).toHaveAttribute('aria-pressed', 'false');
  fireEvent.click(screen.getByRole('button', { name: 'Close' }));
  expect(paywall()).not.toBeInTheDocument();
  expect(fetchMock.mock.calls.filter(([, options]) => options?.method === 'POST')).toHaveLength(0);
});

it('holds the toggle until the initial plan body is verified', async () => {
  const pending = deferred<Response>(); planReply = () => pending.promise;
  await open(); expect(toggle()).toHaveAttribute('aria-pressed', 'false');
  fireEvent.click(toggle()); expect(paywall()).not.toBeInTheDocument(); expect(toggle()).toBeDisabled();
  await act(async () => pending.resolve(Response.json({ current_plan: 'Pro' })));
  await waitFor(() => expect(toggle()).toHaveAttribute('aria-pressed', 'true'));
  expect(toggle()).toBeEnabled(); expect(paywall()).not.toBeInTheDocument();
});

it.each(['Unknown', null])('does not label an unverified plan %j as an upgrade opportunity', async plan => {
  currentPlan = plan; await open();
  expect(toggle()).toBeDisabled(); fireEvent.click(toggle()); expect(paywall()).not.toBeInTheDocument();
  expect(toggle()).toHaveAttribute('aria-pressed', 'false');
});

it('hides an open Free paywall when owner authority is retired', async () => {
  currentPlan = 'Free'; await open();
  await waitFor(() => expect(toggle()).toBeEnabled()); fireEvent.click(toggle()); expect(paywall()).toBeVisible();
  act(() => window.dispatchEvent(new Event('pagehide')));
  expect(toggle()).toHaveAttribute('aria-pressed', 'false');
  expect(toggle()).toBeDisabled(); expect(paywall()).not.toBeInTheDocument();
});
