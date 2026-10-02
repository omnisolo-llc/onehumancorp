import { act, cleanup, fireEvent, render, screen, waitFor } from '@testing-library/react';
import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import Builder from './page';
import { useBuilderStore } from './store';
import { notifyQueueIdentityChange } from '@/lib/sync/queueIdentity';
import { installBuilderLocks } from './testLocks';
import { prepareSiteSnapshot } from './publicationContracts';

vi.mock('../../components/help', () => ({ useWalkthrough: () => ({ startWalkthrough: vi.fn() }) }));
vi.mock('../../components/TooltipRegistry', () => ({ WithTooltip: ({ children }: React.PropsWithChildren) => <>{children}</> }));
vi.mock('../../components/Walkthrough', () => ({ WalkthroughTarget: ({ children }: React.PropsWithChildren) => <>{children}</>, InteractiveWalkthrough: () => null }));
let owner = { userId: 'owner-a', tenantId: 'business-a' };
let posts: Record<string, unknown>[];
let receipt: Record<string, unknown> | null;
let publicationFails: boolean;
let generation: () => Promise<Response>;
let seo: () => Promise<Response>;
beforeEach(() => {
  localStorage.clear(); notifyQueueIdentityChange(); installBuilderLocks();
  owner = { userId: 'owner-a', tenantId: 'business-a' }; posts = []; receipt = null; publicationFails = false;
  generation = async () => Response.json({ pages: [{ blocks: [{ block_type: 'HeroBlock', content: { headline: 'Actual returned draft' } }] }] });
  seo = async () => Response.json({ description: 'Returned private metadata' });
  vi.stubGlobal('fetch', vi.fn(async (url, options) => {
    if (String(url).endsWith('/session-identity')) return Response.json({ ...owner, expiresAt: Date.now() + 60_000 });
    if (url === '/api/v1/builder/generate') return generation();
    if (url === '/api/v1/builder/auto_seo') return seo();
    if (url === '/api/v1/builder/publications' && options?.method === 'POST') {
      const body = JSON.parse(String(options.body)); posts.push(body);
      if (publicationFails) return Response.json({ error: 'unavailable' }, { status: 500 });
      const snapshot = await prepareSiteSnapshot(body.snapshot);
      receipt = { schema_version: 1, user_id: owner.userId, organization_id: owner.tenantId, publication_id: '20000000-0000-4000-8000-000000000002', operation_id: body.operation_id, site_id: '30000000-0000-4000-8000-000000000003', version: 1, status: 'pending', snapshot_sha256: snapshot.snapshot_sha256, snapshot_encoding: snapshot.snapshot_encoding, public_path: null };
      return Response.json(receipt, { status: 202 });
    }
    if (String(url).includes('/publications/operations/')) return receipt ? Response.json(receipt) : Response.json({ error: 'missing' }, { status: 404 });
    return Response.json([]);
  }));
});
afterEach(() => { cleanup(); notifyQueueIdentityChange(); vi.unstubAllGlobals(); });
async function openEditor() {
  const view = render(<Builder />); await screen.findByText('What are you building today?');
  act(() => useBuilderStore.setState({ businessName: 'Owner business', bio: 'Owner description', blocks: [{ type: 'Hero', props: { headline: 'Reviewed headline' } }], status: 'draft' }));
  await screen.findByText('Mobile Editor');
  return view;
}
async function startGeneration() {
  render(<Builder />); await screen.findByText('What are you building today?');
  act(() => useBuilderStore.setState({ status: 'idle', wizardStep: 3, businessName: 'Real business', businessCategory: 'Owner services', bio: 'Owner requested service description', vibe: 'Minimalist' }));
  fireEvent.click(screen.getByRole('button', { name: 'Build Store' }));
}

it('requires explicit review and a published receipt before showing the canonical public link', async () => {
  await openEditor();
  const review = await screen.findByRole('button', { name: 'Review public version' }); await waitFor(() => expect(review).toBeEnabled()); fireEvent.click(review);
  expect(await screen.findByLabelText('Public website snapshot')).toHaveTextContent('Reviewed headline'); expect(posts).toHaveLength(0);
  fireEvent.click(screen.getByRole('button', { name: 'Publish reviewed version' }));
  await screen.findByText(/Publication is queued/);
  expect(posts).toHaveLength(1); expect(posts[0]).toMatchObject({ site_id: null, snapshot: { domain: null } });
  expect(JSON.stringify(posts[0])).not.toMatch(/cloud.omnisolo.co|tenant_id|user_id/);
  expect(screen.queryByText("You're Live!")).toBeNull(); expect(screen.queryByRole('link', { name: 'Open published website' })).toBeNull();
  receipt = { ...receipt!, status: 'published', public_path: '/api/v1/public/sites/30000000-0000-4000-8000-000000000003' };
  fireEvent.click(screen.getByRole('button', { name: 'Check publication status' }));
  expect(await screen.findByRole('link', { name: 'Open published website' })).toHaveAttribute('href', receipt.public_path);
  expect(vi.mocked(fetch).mock.calls.some(([url]) => String(url).endsWith('/publish_draft'))).toBe(false);
});

it('holds an unknown publication across reload instead of inventing a live site or resubmitting', async () => {
  publicationFails = true; const view = await openEditor();
  const review = await screen.findByRole('button', { name: 'Review public version' }); await waitFor(() => expect(review).toBeEnabled()); fireEvent.click(review);
  fireEvent.click(await screen.findByRole('button', { name: 'Publish reviewed version' }));
  await waitFor(() => expect(posts).toHaveLength(1));
  await screen.findByRole('button', { name: 'Check publication status' }); view.unmount();
  render(<Builder />); await screen.findByText('Mobile Editor');
  expect(await screen.findByRole('button', { name: 'Review public version' })).toBeDisabled();
  expect(screen.queryByRole('link', { name: 'Open published website' })).toBeNull(); expect(posts).toHaveLength(1);
});

it('requires another review when the private draft changes after review', async () => {
  await openEditor(); const review = await screen.findByRole('button', { name: 'Review public version' }); await waitFor(() => expect(review).toBeEnabled()); fireEvent.click(review);
  await screen.findByLabelText('Public website snapshot');
  act(() => useBuilderStore.getState().setBlocks([{ type: 'Hero', props: { headline: 'Newer owner edit' } }]));
  fireEvent.click(screen.getByRole('button', { name: 'Publish reviewed version' }));
  await screen.findByText(/private draft changed/i); expect(posts).toHaveLength(0);
});

it('uses only the actual generated draft without invented discounts or duplicate variants', async () => {
  await startGeneration(); await screen.findByText('Pick your draft');
  expect(useBuilderStore.getState().drafts).toHaveLength(1);
  expect(useBuilderStore.getState().blocks).toEqual([{ type: 'Hero', props: { headline: 'Actual returned draft' } }]);
  expect(JSON.stringify(useBuilderStore.getState().blocks)).not.toMatch(/20%|Referral/);
  const call = vi.mocked(fetch).mock.calls.find(([url]) => url === '/api/v1/builder/generate')!;
  expect(new Headers(call[1]?.headers).get('x-ohc-expected-user')).toBe(owner.userId);
  expect(new Headers(call[1]?.headers).get('x-ohc-expected-tenant')).toBe(owner.tenantId);
  expect(JSON.parse(String(call[1]?.body)).description).toContain('Minimalist');
});

it('rejects contradictory generation acknowledgements without manufacturing draft content', async () => {
  generation = async () => Response.json({ success: false, pages: [{ blocks: [{ block_type: 'HeroBlock', content: { headline: 'Unconfirmed' } }] }] });
  await startGeneration(); await screen.findByText('The generated draft could not be confirmed.');
  expect(useBuilderStore.getState().blocks).toEqual([]); expect(screen.queryByText('Unconfirmed')).toBeNull();
});

it('retires late generated content when the verified owner changes', async () => {
  let finish!: (response: Response) => void; generation = () => new Promise(resolve => { finish = resolve; });
  await startGeneration(); await waitFor(() => expect(finish).toBeDefined());
  await act(async () => { owner = { userId: 'owner-b', tenantId: 'business-b' }; notifyQueueIdentityChange(); });
  await screen.findByText('What are you building today?');
  await act(async () => { finish(Response.json({ pages: [{ blocks: [{ block_type: 'HeroBlock', content: { headline: 'Late private A draft' } }] }] })); });
  expect(screen.queryByText('Late private A draft')).toBeNull(); expect(useBuilderStore.getState().blocks).toEqual([]);
  for (const key of Object.keys(localStorage).filter(key => key.includes(encodeURIComponent('owner-b')))) expect(localStorage.getItem(key)).not.toContain('Late private A draft');
});

it('applies confirmed SEO metadata only to the private reviewed snapshot', async () => {
  await openEditor(); fireEvent.click(screen.getByRole('button', { name: 'Prepare draft SEO metadata' }));
  expect(await screen.findByLabelText('Private SEO metadata')).toHaveTextContent('Returned private metadata');
  expect(screen.getByText('SEO metadata updated in this private draft. Review it before publishing.')).toBeVisible();
  const review = await screen.findByRole('button', { name: 'Review public version' }); await waitFor(() => expect(review).toBeEnabled()); fireEvent.click(review);
  expect(await screen.findByLabelText('Public website snapshot')).toHaveTextContent('Returned private metadata'); expect(posts).toHaveLength(0);
});

it('does not claim SEO was applied after a failed response', async () => {
  seo = async () => Response.json({ error: 'unavailable' }, { status: 503 });
  await openEditor(); fireEvent.click(screen.getByRole('button', { name: 'Prepare draft SEO metadata' }));
  await screen.findByText('Draft content suggestions could not be confirmed.');
  expect(useBuilderStore.getState().seoMetadata).toEqual({}); expect(screen.queryByLabelText('Private SEO metadata')).toBeNull();
});
