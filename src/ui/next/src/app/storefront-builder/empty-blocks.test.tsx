import { cleanup, fireEvent, render, screen, waitFor } from '@testing-library/react';
import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import Page from './page';
import { notifyQueueIdentityChange } from '@/lib/sync/queueIdentity';
import { installBuilderLocks } from '../builder/testLocks';
vi.mock('../../components/help', () => ({ useWalkthrough: () => ({ startWalkthrough: vi.fn() }) }));
vi.mock('../../components/TooltipRegistry', () => ({ WithTooltip: ({ children }: React.PropsWithChildren) => <>{children}</> }));
vi.mock('../../components/Walkthrough', () => ({ WalkthroughTarget: ({ children }: React.PropsWithChildren) => <>{children}</>, InteractiveWalkthrough: () => null }));
const owner = { userId: 'owner', tenantId: 'workspace' };
const key = 'omnisolo_onboarding_owned_v1:' + encodeURIComponent(JSON.stringify([owner.userId, owner.tenantId])) + ':storefront-builder-draft';
beforeEach(() => {
  localStorage.clear(); notifyQueueIdentityChange(); installBuilderLocks();
  localStorage.setItem(key, JSON.stringify({ format: 1, revision: 'owned-layout', data: { bio: 'Owner description', status: 'draft', blocks: [{ type: 'Hero', props: { headline: 'Owner headline', copy: 'Owner text' } }] } }));
  vi.stubGlobal('fetch', vi.fn(async (url) => Response.json(String(url).endsWith('/session-identity') ? { ...owner, expiresAt: Date.now() + 60_000 } : {})));
});
afterEach(() => { cleanup(); vi.unstubAllGlobals(); });
it.each([
  ['Hero', { headline: '', copy: '' }],
  ['Catalog', { items: [{ name: '', price: '', description: '' }] }],
  ['Booking', { title: '', availability: '', booking_url: '' }],
  ['Contact', { email: '', phone: '' }],
  ['Referral', { offerTitle: '', offerDescription: '' }],
])('starts a new%s block with owner-entered fields instead of invented business facts', async (type, props) => {
  render(<Page />); fireEvent.click(await screen.findByRole('button', { name: 'Add Block' }));
  fireEvent.click(screen.getByRole('button', { name: String(type) }));
  await waitFor(() => expect(JSON.parse(localStorage.getItem(key)!).data.blocks.at(-1)).toEqual({ type, props }));
});
it('adds another catalog item without inventing its name, price or description', async () => {
  render(<Page />); fireEvent.click(await screen.findByRole('button', { name: 'Add Block' }));
  fireEvent.click(screen.getByRole('button', { name: 'Catalog' }));
  fireEvent.click(await screen.findByRole('button', { name: '+ Add Item' }));
  fireEvent.click(screen.getByRole('button', { name: 'Save Changes' }));
  await waitFor(() => expect(JSON.parse(localStorage.getItem(key)!).data.blocks.at(-1).props.items).toEqual([
    { name: '', price: '', description: '' }, { name: '', price: '', description: '' },
  ]));
});
