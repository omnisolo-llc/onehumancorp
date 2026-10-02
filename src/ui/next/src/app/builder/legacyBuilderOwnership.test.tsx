import { act, cleanup, render, screen } from '@testing-library/react';
import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import Builder from './page';
import { useBuilderStore } from './store';
import { notifyQueueIdentityChange } from '@/lib/sync/queueIdentity';
import { installBuilderLocks } from './testLocks';

vi.mock('../../components/help', () => ({ useWalkthrough: () => ({ startWalkthrough: vi.fn() }) }));
vi.mock('../../components/TooltipRegistry', () => ({ WithTooltip: ({ children }: React.PropsWithChildren) => <>{children}</> }));
vi.mock('../../components/Walkthrough', () => ({ WalkthroughTarget: ({ children }: React.PropsWithChildren) => <>{children}</>, InteractiveWalkthrough: () => null }));
const owner = { userId: 'builder-owner', tenantId: 'builder-business' };
beforeEach(() => {
  localStorage.clear(); notifyQueueIdentityChange(); installBuilderLocks();
  useBuilderStore.setState(useBuilderStore.getInitialState(), true);
  vi.stubGlobal('fetch', vi.fn(async url => String(url).endsWith('/session-identity')
    ? Response.json({ ...owner, expiresAt: Date.now() + 60_000 })
    : Response.json([])));
});
afterEach(() => { cleanup(); notifyQueueIdentityChange(); vi.unstubAllGlobals(); });

it('holds an unowned legacy builder cache without exposing or relabeling its private content', async () => {
  const legacy = JSON.stringify({ state: { ...useBuilderStore.getState(), businessName: 'Other owner private business', blocks: [{ type: 'Hero', props: { headline: 'Other owner private headline' } }], status: 'live', liveUrl: 'https://invented.cloud.omnisolo.co' }, version: 2 });
  localStorage.setItem('builder-storage', legacy);
  await useBuilderStore.persist.rehydrate();
  await act(async () => { render(<Builder />); });
  expect(await screen.findByText(/older unowned builder draft is held separately/i)).toBeVisible();
  expect(screen.queryByText(/Other owner private|invented.cloud.omnisolo.co/)).toBeNull();
  expect(localStorage.getItem('builder-storage')).toBe(legacy);
  for (const key of Object.keys(localStorage).filter(key => key.includes('legacy-builder-draft'))) {
    expect(localStorage.getItem(key)).not.toContain('Other owner private');
  }
});

it('requires verified identity before showing an editable legacy builder', async () => {
  vi.mocked(fetch).mockResolvedValue(Response.json({ error: 'not_authenticated' }, { status: 401 }));
  render(<Builder />);
  expect(await screen.findByRole('alert')).toHaveTextContent(/session|verif|sign in/i);
  expect(screen.queryByText('What are you building today?')).toBeNull();
  expect(vi.mocked(fetch).mock.calls.some(([, options]) => options?.method === 'POST')).toBe(false);
});
