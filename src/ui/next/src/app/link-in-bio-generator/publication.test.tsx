import { cleanup, fireEvent, render, screen, waitFor } from '@testing-library/react';
import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import Page from './page';
import { notifyQueueIdentityChange } from '@/lib/sync/queueIdentity';
import { installBuilderLocks } from '../builder/testLocks';
import { prepareSiteSnapshot } from '../builder/publicationContracts';
vi.mock('next/navigation', () => ({ useRouter: () => ({ back: vi.fn() }) }));
vi.mock('../components/PoweredByOmniSolo', () => ({ PoweredByOmniSolo: () => null }));
const SITE = '30000000-0000-4000-8000-000000000003';
let owner = { userId: 'owner-a', tenantId: 'tenant-a' };
let publication: Record<string, unknown> | null;
let phase: string;
let payload: Record<string, unknown> | null;
beforeEach(() => {
  localStorage.clear(); notifyQueueIdentityChange(); installBuilderLocks(); owner = { userId: 'owner-a', tenantId: 'tenant-a' }; publication = null; payload = null; phase = 'pending';
  vi.stubGlobal('fetch', vi.fn(async (url, options) => {
    if (String(url).endsWith('/session-identity')) return Response.json({ ...owner, expiresAt: Date.now() + 60_000 });
    if (String(url).startsWith('/api/v1/growth/link-in-bio')) return options?.method === 'POST' ? new Response('') : Response.json({ store_name: 'Private bakery', bio: 'Owner biography', theme: 'dark', links: [{ title: 'Menu', url: 'https://example.test/menu' }], remove_branding: true });
    if (String(url).startsWith('/api/v1/builder/publications')) {
      if (options?.method === 'POST') {
        const data = JSON.parse(String(options.body)); payload = data;
        const prepared = await prepareSiteSnapshot(data.snapshot);
        publication = { schema_version: 1, user_id: owner.userId, organization_id: owner.tenantId, publication_id: '20000000-0000-4000-8000-000000000002', operation_id: data.operation_id, site_id: SITE, version: 1, snapshot_sha256: prepared.snapshot_sha256, snapshot_encoding: prepared.snapshot_encoding };
      }
      return Response.json({ ...publication, status: phase, public_path: phase === 'published' ? '/api/v1/public/sites/' + SITE : null }, { status: options?.method === 'POST' ? 202 : 200 });
    }
    return Response.json({ error: 'Unexpected request' }, { status: 500 });
  }));
});
afterEach(() => { cleanup(); vi.unstubAllGlobals(); vi.restoreAllMocks(); });
it('keeps the private save separate from a reviewed public-bio snapshot and confirmed site link', async () => {
  render(<Page />);
  await screen.findByDisplayValue('Private bakery');
  fireEvent.click(screen.getByRole('button', { name: 'Save private configuration' }));
  await screen.findByRole('button', { name: 'Saved private configuration' });
  expect(payload).toBeNull();
  fireEvent.click(await screen.findByRole('button', { name: 'Review public version' }));
  expect(await screen.findByLabelText('Public website snapshot')).toHaveTextContent('Private bakery');
  expect(screen.getByLabelText('Public website snapshot')).toHaveTextContent('https://example.test/menu');
  fireEvent.click(screen.getByRole('button', { name: 'Publish reviewed version' }));
  await screen.findByText(/Publication is queued/);
  expect(payload).toMatchObject({ site_id: null, snapshot: { domain: null, pages: [{ path: '/', blocks: [{ block_type: 'HeroBlock' }, { block_type: 'LinkListBlock', content: { links: [{ label: 'Menu', url: 'https://example.test/menu' }] } }] }] } });
  expect(JSON.stringify(payload)).not.toMatch(/tenant-a|owner-a|remove_branding|"theme"/);
  expect(screen.queryByRole('link', { name: 'Open published website' })).toBeNull();
  phase = 'published'; fireEvent.click(screen.getByRole('button', { name: 'Check publication status' }));
  expect(await screen.findByRole('link', { name: 'Open published website' })).toHaveAttribute('href', '/api/v1/public/sites/' + SITE);
});
it('removes reviewed profile fields when the verified publication owner changes', async () => {
  render(<Page />); await screen.findByDisplayValue('Private bakery');
  fireEvent.click(await screen.findByRole('button', { name: 'Review public version' }));
  await screen.findByLabelText('Public website snapshot');
  owner = { userId: 'owner-b', tenantId: 'tenant-b' };
  fireEvent.click(screen.getByRole('button', { name: 'Publish reviewed version' }));
  await waitFor(() => expect(screen.queryByDisplayValue('Private bakery')).toBeNull());
  expect(screen.queryByLabelText('Public website snapshot')).toBeNull();
  expect(payload).toBeNull();
  expect(screen.getByRole('status', { name: 'Private profile status' })).toHaveTextContent(/session changed/i);
});
it('does not publish an invalid edited destination', async () => {
  render(<Page />); await screen.findByDisplayValue('Private bakery');
  fireEvent.change(screen.getByRole('textbox', { name: 'Link 1 URL' }), { target: { value: 'javascript:alert(1)' } });
  fireEvent.click(await screen.findByRole('button', { name: 'Review public version' }));
  expect(await screen.findByText(/absolute HTTP or HTTPS/)).toBeVisible();
  expect(screen.queryByRole('button', { name: 'Publish reviewed version' })).toBeNull(); expect(payload).toBeNull();
});
