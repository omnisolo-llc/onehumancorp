import { act, cleanup, render, screen, waitFor } from '@testing-library/react';
import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import { OnboardingChatAgent } from './components/OnboardingChatAgent';
import { installOnboardingLocks } from '../testLocks';
import { notifyQueueIdentityChange } from '@/lib/sync/queueIdentity';

const chips = [
  "I'm a local baker selling custom cakes", 'I run a neighborhood handyman service',
  'I am an online music tutor', 'I manage 15 long-term apartment rentals',
];
const owner = { userId: 'readiness-owner', tenantId: 'readiness-tenant' };

beforeEach(() => { installOnboardingLocks(); localStorage.clear(); notifyQueueIdentityChange(); });
afterEach(() => { cleanup(); vi.unstubAllGlobals(); });

it('stays visibly busy through identity and the complete saved-state body, then exposes every initial prompt', async () => {
  let identify!: (response: Response) => void;
  let identityReleased = false;
  let restore!: () => void;
  const state = new Response(new ReadableStream({ start(controller) {
    restore = () => { controller.enqueue(new TextEncoder().encode('{"chatMessages":[]}')); controller.close(); };
  } }));
  vi.stubGlobal('fetch', vi.fn<typeof fetch>(async input => {
    if (String(input).endsWith('/session-identity')) {
      if (identityReleased) return Response.json({ ...owner, expiresAt: Date.now() + 60_000 });
      return new Promise<Response>(resolve => { identify = response => { identityReleased = true; resolve(response); }; });
    }
    if (String(input).endsWith('/onboarding/state')) return state;
    throw new Error(`Unexpected setup read: ${String(input)}`);
  }));
  render(<OnboardingChatAgent onComplete={vi.fn()} />);
  expect(screen.getByRole('status', { name: 'Restoring setup' })).toHaveAttribute('aria-busy', 'true');
  expect(screen.queryByRole('button', { name: chips[2] })).not.toBeInTheDocument();
  await waitFor(() => expect(identify).toBeDefined());
  await act(async () => { identify(Response.json({ ...owner, expiresAt: Date.now() + 60_000 })); });
  await waitFor(() => expect(state.bodyUsed).toBe(true));
  expect(screen.getByRole('status', { name: 'Restoring setup' })).toHaveAttribute('aria-busy', 'true');
  await act(async () => { restore(); });
  for (const chip of chips) expect(await screen.findByRole('button', { name: chip })).toBeEnabled();
  expect(document.querySelector('[aria-busy="true"]')).toBeNull();
});

it('restores the current owner conversation without replacing it with initial prompt controls', async () => {
  vi.stubGlobal('fetch', vi.fn<typeof fetch>(async input => Response.json(String(input).endsWith('/session-identity')
    ? { ...owner, expiresAt: Date.now() + 60_000 }
    : { chatMessages: [{ role: 'assistant', content: 'Saved assistant response' }, { role: 'user', content: 'Saved owner description' }] })));
  render(<OnboardingChatAgent onComplete={vi.fn()} />);
  expect(await screen.findByText('Saved owner description')).toBeVisible();
  for (const chip of chips) expect(screen.queryByRole('button', { name: chip })).not.toBeInTheDocument();
  expect(document.querySelector('[aria-busy="true"]')).toBeNull();
});

it.each(['rejected', 'malformed'] as const)('ends busy restore with a truthful alert for a %s state read', async failure => {
  vi.stubGlobal('fetch', vi.fn<typeof fetch>(async input => {
    if (String(input).endsWith('/session-identity')) return Response.json({ ...owner, expiresAt: Date.now() + 60_000 });
    if (String(input).endsWith('/onboarding/state')) return failure === 'rejected'
      ? Response.json({ error: 'unavailable' }, { status: 503 }) : new Response('{', { status: 200 });
    throw new Error(`Unexpected setup read: ${String(input)}`);
  }));
  render(<OnboardingChatAgent onComplete={vi.fn()} />);
  expect(screen.getByRole('status', { name: 'Restoring setup' })).toHaveAttribute('aria-busy', 'true');
  expect(await screen.findByRole('alert')).toHaveTextContent('Your other session’s draft remains held.');
  expect(document.querySelector('[aria-busy="true"]')).toBeNull();
  for (const chip of chips) expect(screen.queryByRole('button', { name: chip })).not.toBeInTheDocument();
});
