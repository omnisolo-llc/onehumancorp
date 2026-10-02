import { act, cleanup, fireEvent, render, screen } from '@testing-library/react';
import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import Website from '../website-builder/page';
import Storefront from '../storefront-builder/page';
import { useWebsiteBuilderStore } from '../website-builder/store';
import { notifyQueueIdentityChange } from '@/lib/sync/queueIdentity';
import { installBuilderLocks } from './testLocks';
import { prepareSiteSnapshot } from './publicationContracts';
vi.mock('next/navigation', () => ({ useRouter: () => ({ push: vi.fn(), back: vi.fn() }) }));
vi.mock('../../components/help', () => ({ useWalkthrough: () => ({ startWalkthrough: vi.fn() }) }));
vi.mock('../../components/TooltipRegistry', () => ({ WithTooltip: ({ children }: React.PropsWithChildren) => <>{children}</> }));
vi.mock('../../components/Walkthrough', () => ({ WalkthroughTarget: ({ children }: React.PropsWithChildren) => <>{children}</>, InteractiveWalkthrough: () => null }));
vi.mock('./components', () => ({ SmartBlock: () => null, DraggableBlock: ({ children }: React.PropsWithChildren) => <>{children}</>, ActionSheet: () => null }));
const owner = { userId: 'editor', tenantId: 'business' };
let posted: Record<string, unknown> | null;
beforeEach(() => {
  localStorage.clear(); notifyQueueIdentityChange(); installBuilderLocks(); posted = null;
  vi.stubGlobal('fetch', vi.fn(async (url, options) => {
    if (String(url).endsWith('/session-identity')) return Response.json({ ...owner, expiresAt: Date.now() + 60_000 });
    if (url === '/api/v1/builder/publications' && options?.method === 'POST') {
      const body = JSON.parse(String(options.body)); posted = body;
      const prepared = await prepareSiteSnapshot(body.snapshot);
      return Response.json({ schema_version: 1, user_id: owner.userId, organization_id: owner.tenantId, publication_id: '20000000-0000-4000-8000-000000000002', operation_id: body.operation_id, site_id: '30000000-0000-4000-8000-000000000003', version: 1, status: 'pending', snapshot_sha256: prepared.snapshot_sha256, snapshot_encoding: prepared.snapshot_encoding, public_path: null }, { status: 202 });
    }
    return Response.json({});
  }));
});
afterEach(() => { cleanup(); vi.unstubAllGlobals(); });
it.each(['website', 'storefront'])('publishes only a separately reviewed current %s layout without a fabricated domain', async kind => {
  if (kind === 'website') {
    render(<Website />); await screen.findByText('Your business, live in minutes.');
    act(() => useWebsiteBuilderStore.setState({ status: 'draft', bio: 'Reviewed description', businessName: 'Real website title', blocks: [{ type: 'Hero', props: { headline: 'Reviewed headline' } }] }));
  } else {
    const key = 'omnisolo_onboarding_owned_v1:' + encodeURIComponent(JSON.stringify([owner.userId, owner.tenantId])) + ':storefront-builder-draft';
    localStorage.setItem(key, JSON.stringify({ format: 1, revision: 'saved', data: { status: 'draft', bio: 'Reviewed description', blocks: [{ type: 'Hero', props: { headline: 'Reviewed headline' } }] } }));
    render(<Storefront />);
  }
  fireEvent.click(await screen.findByRole('button', { name: 'Review public version' }));
  const review = await screen.findByLabelText('Public website snapshot');
  expect(review).toHaveTextContent('Reviewed headline'); expect(review).toHaveTextContent('Reviewed description'); expect(posted).toBeNull();
  fireEvent.click(screen.getByRole('button', { name: 'Publish reviewed version' }));
  await screen.findByText(/Publication is queued/);
  expect(posted).toMatchObject({ site_id: null, snapshot: { domain: null, pages: [{ path: '/', blocks: [{ block_type: 'HeroBlock', content: { headline: 'Reviewed headline' }, sort_order: 0 }] }] } });
  expect(JSON.stringify(posted)).not.toMatch(/cloud.omnisolo.co|tenant_id|user_id/);
  expect(screen.queryByRole('link', { name: 'Open published website' })).toBeNull();
});
