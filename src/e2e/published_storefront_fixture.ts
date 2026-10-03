import { randomUUID } from 'node:crypto';
import { expect, type Page } from '@playwright/test';
import { createGrowthOwner } from './growth_owner';
import type { SiteSnapshot } from '../ui/next/src/app/builder/publicationContracts';

/** Submit a real immutable snapshot only inside the guarded disposable stack. */
export async function publishOwnedStorefront(page: Page, baseURL: string | undefined,
  owner: { userId: string; tenantId: string }, snapshot: SiteSnapshot) {
  if (!baseURL || !['localhost', '127.0.0.1', '[::1]'].includes(new URL(baseURL).hostname)
    || !process.env.E2E_POSTGRES_CONTAINER?.startsWith('ohc-e2e-pg-')) {
    throw new Error('Publication fixtures require the isolated native acceptance stack.');
  }
  const identity = await page.request.get('/api/v1/auth/session-identity');
  expect(identity.status()).toBe(200);
  expect(await identity.json()).toMatchObject({ userId: owner.userId, tenantId: owner.tenantId });
  const operationId = randomUUID();
  const headers = { Origin: new URL(baseURL!).origin, 'Sec-Fetch-Site': 'same-origin', 'x-ohc-expected-user': owner.userId, 'x-ohc-expected-tenant': owner.tenantId };
  // This is a real publication in the runner's guarded disposable tenant.
  const submitted = await page.request.post('/api/v1/builder/publications', { headers, data: { operation_id: operationId, site_id: null, snapshot_encoding: 'jcs-rfc8785-v1', snapshot } });
  expect([200, 202]).toContain(submitted.status());
  let receipt = await submitted.json();
  expect(receipt).toMatchObject({ operation_id: operationId, user_id: owner.userId, organization_id: owner.tenantId });
  await expect.poll(async () => {
    const response = await page.request.get(`/api/v1/builder/publications/operations/${operationId}`, { headers });
    expect(response.status()).toBe(200); receipt = await response.json();
    return receipt.status;
  }, { timeout: 30_000, intervals: [250, 500, 1000] }).toBe('published');
  expect(receipt.public_path).toBe(`/api/v1/public/sites/${receipt.site_id}`);
  return { operationId, headers, receipt };
}

/** Publish disposable-tenant content and save only its verified receipt. */
export async function createPublishedStorefront(page: Page, baseURL: string | undefined) {
  const owner = await createGrowthOwner(page, baseURL);
  const snapshot: SiteSnapshot = { domain: null, pages: [{ path: '/', title: 'Owned published storefront fixture', seo_metadata: {}, blocks: [{ block_type: 'HeroBlock', content: { headline: 'Reviewed fixture storefront' }, sort_order: 0 }] }] };
  const { operationId, headers, receipt } = await publishOwnedStorefront(page, baseURL, owner, snapshot);
  const publicPage = await page.request.get(receipt.public_path);
  expect(publicPage.status()).toBe(200);
  expect(await publicPage.text()).toContain('Reviewed fixture storefront');
  await page.goto('/team');
  await page.evaluate(({ owner, operationId, snapshot, receipt }) => {
    const key = 'omnisolo_onboarding_owned_v1:' + encodeURIComponent(JSON.stringify([owner.userId, owner.tenantId])) + ':site-publication-operation:storefront-builder';
    localStorage.setItem('business_display_name', 'untrusted local tenant');
    localStorage.setItem(key, JSON.stringify({ format: 1, phase: 'acknowledged', operation: { owner: { userId: owner.userId, tenantId: owner.tenantId }, operation_id: operationId, site_id: null, snapshot_encoding: 'jcs-rfc8785-v1', snapshot_sha256: receipt.snapshot_sha256, snapshot }, receipt }));
  }, { owner, operationId, snapshot, receipt });
  await page.reload();
  return { owner, operationId, headers, receipt };
}
