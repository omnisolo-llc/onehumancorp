import { randomUUID } from 'node:crypto';
import { test, expect } from './fixtures';
import { createGrowthOwner } from './growth_owner';

test('team copies only a refreshed owned published storefront and withdraws revoked code', async ({ page, baseURL, context }) => {
  const owner = await createGrowthOwner(page, baseURL);
  const operationId = randomUUID();
  const snapshot = { domain: null, pages: [{ path: '/', title: 'Owned published storefront fixture', seo_metadata: {}, blocks: [{ block_type: 'HeroBlock', content: { headline: 'Reviewed fixture storefront' }, sort_order: 0 }] }] };
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
  const publicPage = await page.request.get(receipt.public_path);
  expect(publicPage.status()).toBe(200);
  expect(await publicPage.text()).toContain('Reviewed fixture storefront');
  await context.grantPermissions(['clipboard-read', 'clipboard-write']);
  await page.goto('/team');
  await page.evaluate(({ owner, operationId, snapshot, receipt }) => {
    const key = 'omnisolo_onboarding_owned_v1:' + encodeURIComponent(JSON.stringify([owner.userId, owner.tenantId])) + ':site-publication-operation:storefront-builder';
    localStorage.setItem('business_display_name', 'untrusted local tenant');
    localStorage.setItem(key, JSON.stringify({ format: 1, phase: 'acknowledged', operation: { owner: { userId: owner.userId, tenantId: owner.tenantId }, operation_id: operationId, site_id: null, snapshot_encoding: 'jcs-rfc8785-v1', snapshot_sha256: receipt.snapshot_sha256, snapshot }, receipt }));
  }, { owner, operationId, snapshot, receipt });
  await page.reload();
  const widget = page.getByRole('region', { name: 'Published storefront embed' });
  await expect(widget.getByRole('button', { name: 'Check publication' })).toBeEnabled();
  await expect(widget.getByRole('button', { name: 'Copy Embed Code' })).toBeDisabled();
  await widget.getByRole('button', { name: 'Check publication' }).click();
  await expect(widget.getByRole('button', { name: 'Copy Embed Code' })).toBeEnabled();
  const code = await widget.getByRole('textbox', { name: 'Published embed code' }).inputValue();
  expect(code).toContain(new URL(receipt.public_path, baseURL).href);
  expect(code).not.toContain('referrals/click'); expect(code).not.toContain('untrusted');
  await widget.getByRole('button', { name: 'Copy Embed Code' }).click();
  await expect(widget.getByRole('status')).toHaveText('Copied to clipboard.');
  expect(await page.evaluate(() => navigator.clipboard.readText())).toBe(code);
  await page.evaluate(html => {
    const template = document.createElement('template'); template.innerHTML = html;
    const frame = template.content.querySelector('iframe');
    if (!frame) throw new Error('Copied embed has no iframe');
    frame.id = 'published-embed-proof'; document.body.append(frame);
  }, code);
  await expect(page.frameLocator('#published-embed-proof').getByRole('heading', { name: 'Reviewed fixture storefront' })).toBeVisible();
  const revoked = await page.request.delete(`/api/v1/builder/publications/${receipt.publication_id}`, { headers });
  expect(revoked.status()).toBe(200); expect(await revoked.json()).toMatchObject({ status: 'revoked' });
  await widget.getByRole('button', { name: 'Copy Embed Code' }).click();
  await expect(widget.getByRole('button', { name: 'Copy Embed Code' })).toBeDisabled();
  await expect(widget.getByRole('textbox', { name: 'Published embed code' })).toHaveCount(0);
  await expect(widget.getByRole('status')).toContainText('revoked');
  expect((await page.request.get(receipt.public_path)).status()).toBe(404);
});
