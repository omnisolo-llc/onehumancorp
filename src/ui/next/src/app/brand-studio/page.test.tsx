import { act, cleanup, fireEvent, render, screen, waitFor } from '@testing-library/react';
import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import BrandStudio from './page';
import { notifyQueueIdentityChange } from '@/lib/sync/queueIdentity';
import { installBuilderLocks } from '../builder/testLocks';
import { prepareSiteSnapshot } from '../builder/publicationContracts';

// Security assertions must exercise the real sanitizer, not the global UI stub.
vi.unmock('dompurify');

let owner = { userId: 'brand-owner', tenantId: 'brand-business' };
function toolbox() {
  return {
    id: '10000000-0000-4000-8000-000000000001',
    generation: { kind: 'model_draft', provider: 'local-fixture', model: 'draft-fixture', input_source: 'supplied_text', website_fetched: false, assets_read: false, business_facts_verified: false, generated_at: '2026-10-02T19:45:00Z' },
    private_note: 'private-toolbox-not-public',
    brand_dna: { name: 'Owner supplied brand', business_type: 'Owner services', positioning: 'A proposed positioning', audience: 'Proposed audience', tone_of_voice: [], colors: ['#123456'], fonts: [], image_style: [] },
    logo_concepts: [{ title: 'Owner logo proposal', svg: '<svg xmlns="http://www.w3.org/2000/svg"><text x="0" y="10">Brand</text></svg>', usage_notes: [] }],
    brand_book: [], catalog: [], campaign_ideas: [], social_calendar: [], assets: [],
    photoshoot: { product_source: 'Owner reference', templates: [], prompts: [], shots: [], refinement_controls: [] },
    store_profile: { domain: 'unverified.example', pages: [{ path: '/', title: 'Owner page', seo_metadata: { description: 'Selected metadata' }, blocks: [{ block_type: 'HeroBlock', content: { headline: 'Reviewed brand headline', subtitle: 'Owner supplied proposal' }, sort_order: 0 }] }] },
    export_formats: [],
  };
}
let generate: () => Promise<Response>;
let posted: Record<string, unknown>[];
beforeEach(() => {
  localStorage.clear(); notifyQueueIdentityChange(); installBuilderLocks(); owner = { userId: 'brand-owner', tenantId: 'brand-business' }; posted = [];
  generate = async () => Response.json(toolbox());
  vi.stubGlobal('fetch', vi.fn(async (url, options) => {
    if (String(url).endsWith('/session-identity')) return Response.json({ ...owner, expiresAt: Date.now() + 60_000 });
    if (url === '/api/v1/builder/brand_toolbox/generate') return generate();
    if (url === '/api/v1/builder/publications' && options?.method === 'POST') {
      const body = JSON.parse(String(options.body)); posted.push(body); const selected = await prepareSiteSnapshot(body.snapshot);
      return Response.json({ schema_version: 1, user_id: owner.userId, organization_id: owner.tenantId, publication_id: '20000000-0000-4000-8000-000000000002', operation_id: body.operation_id, site_id: '30000000-0000-4000-8000-000000000003', version: 1, status: 'pending', snapshot_sha256: selected.snapshot_sha256, snapshot_encoding: selected.snapshot_encoding, public_path: null }, { status: 202 });
    }
    return Response.json({ domain: 'saved-only.example' });
  }));
});
afterEach(() => { cleanup(); notifyQueueIdentityChange(); vi.unstubAllGlobals(); });
async function generateOwned() {
  render(<BrandStudio />);
  const description = await screen.findByLabelText('Business');
  fireEvent.change(description, { target: { value: 'Owner described professional services' } });
  const button = screen.getByRole('button', { name: 'Generate Toolbox' }); await waitFor(() => expect(button).toBeEnabled()); fireEvent.click(button);
  await screen.findByText('Owner supplied brand');
}

it('starts without invented bakery or campaign facts', async () => {
  render(<BrandStudio />);
  expect(await screen.findByLabelText('Business')).toHaveValue('');
  expect(screen.getByLabelText('Campaign')).toHaveValue('');
});
it('requires verified owner access before generation or publication', async () => {
  vi.mocked(fetch).mockResolvedValue(Response.json({ error: 'unauthorized' }, { status: 401 }));
  render(<BrandStudio />); expect(await screen.findByRole('alert')).toHaveTextContent(/verif|session|sign in/i);
  expect(vi.mocked(fetch).mock.calls.some(([, options]) => options?.method === 'POST')).toBe(false);
});

it('reviews only selected actual pages and never treats a private save as a published domain', async () => {
  await generateOwned();
  const review = await screen.findByRole('button', { name: 'Review public version' }); await waitFor(() => expect(review).toBeEnabled()); fireEvent.click(review);
  expect(await screen.findByLabelText('Public website snapshot')).toHaveTextContent('Reviewed brand headline'); expect(posted).toHaveLength(0);
  fireEvent.click(screen.getByRole('button', { name: 'Publish reviewed version' })); await screen.findByText(/Publication is queued/);
  expect(posted).toHaveLength(1); expect(posted[0]).toMatchObject({ snapshot: { domain: null, pages: [{ path: '/', title: 'Owner page', seo_metadata: { description: 'Selected metadata' } }] } });
  expect(JSON.stringify(posted[0])).not.toMatch(/private-toolbox-not-public|unverified.example|tenant_id|user_id/);
  expect(screen.queryByRole('link', { name: 'Open published website' })).toBeNull();
  expect(screen.queryByText(/Published domain/)).toBeNull();
  expect(vi.mocked(fetch).mock.calls.some(([url]) => String(url).endsWith('/publish_website'))).toBe(false);
});

it('does not mount untrusted returned SVG in the application DOM', async () => {
  generate = async () => { const value = toolbox(); value.logo_concepts[0].svg = '<svg xmlns="http://www.w3.org/2000/svg" id="unsafe-brand-svg" onload="globalThis.brandAttack=true"><script>globalThis.brandAttack=true</script><foreignObject><iframe src="https://example.invalid"></iframe></foreignObject><image href="https://example.invalid/track"/><text x="0" y="10">Safe text</text></svg>'; return Response.json(value); };
  await generateOwned();
  expect(document.querySelector('#unsafe-brand-svg')).toBeNull();
  const image = screen.getByRole('img', { name: 'Owner logo proposal' }); const source = decodeURIComponent(image.getAttribute('src')!);
  expect(source).toContain('Safe text'); expect(source).not.toMatch(/<script|foreignObject|iframe|onload|example.invalid/i);
});

it('rejects contradictory generation receipts without rendering or publishing their draft', async () => {
  generate = async () => Response.json({ ...toolbox(), success: false, error: 'not generated' });
  render(<BrandStudio />); fireEvent.change(await screen.findByLabelText('Business'), { target: { value: 'Owner described professional services' } });
  const button = screen.getByRole('button', { name: 'Generate Toolbox' }); await waitFor(() => expect(button).toBeEnabled()); fireEvent.click(button);
  expect(await screen.findByRole('alert')).toHaveTextContent(/could not|unconfirmed|failed/i);
  expect(screen.queryByText('Owner supplied brand')).toBeNull(); expect(screen.queryByRole('button', { name: 'Review public version' })).toBeNull();
});

it('retires a late result after an owner change', async () => {
  let release!: (response: Response) => void; generate = () => new Promise(resolve => { release = resolve; });
  render(<BrandStudio />); fireEvent.change(await screen.findByLabelText('Business'), { target: { value: 'Owner A private description' } });
  const button = screen.getByRole('button', { name: 'Generate Toolbox' }); await waitFor(() => expect(button).toBeEnabled()); fireEvent.click(button); await waitFor(() => expect(release).toBeDefined());
  await act(async () => { owner = { userId: 'owner-b', tenantId: 'business-b' }; notifyQueueIdentityChange(); });
  await waitFor(() => expect(screen.getByLabelText('Business')).toHaveValue(''));
  await act(async () => { release(Response.json(toolbox())); });
  expect(screen.queryByText('Owner supplied brand')).toBeNull(); expect(posted).toHaveLength(0);
});

it('retires a late generated result when its input changes', async () => {
  let release!: (response: Response) => void; generate = () => new Promise(resolve => { release = resolve; });
  render(<BrandStudio />); fireEvent.change(await screen.findByLabelText('Business'), { target: { value: 'First owner description' } });
  const button = screen.getByRole('button', { name: 'Generate Toolbox' }); await waitFor(() => expect(button).toBeEnabled()); fireEvent.click(button); await waitFor(() => expect(release).toBeDefined());
  fireEvent.change(screen.getByLabelText('Business'), { target: { value: 'Newer owner description' } });
  await act(async () => { release(Response.json(toolbox())); });
  expect(screen.getByLabelText('Business')).toHaveValue('Newer owner description'); expect(screen.queryByText('Owner supplied brand')).toBeNull();
});

it('explains the missing authorized provider without inventing brand output or losing the brief', async () => {
  generate = async () => Response.json({ code: 'generation_unavailable' }, { status: 503 });
  render(<BrandStudio />);
  fireEvent.change(await screen.findByLabelText('Business'), { target: { value: 'Owner described professional services' } });
  fireEvent.click(screen.getByRole('button', { name: 'Generate Toolbox' }));
  expect(await screen.findByRole('alert')).toHaveTextContent(/Configure the builder operator tenant and an authorized text-generation provider/);
  expect(screen.getByLabelText('Business')).toHaveValue('Owner described professional services');
  expect(screen.queryByRole('button', { name: 'Review public version' })).toBeNull();
});
