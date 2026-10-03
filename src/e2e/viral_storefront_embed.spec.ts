import { test, expect } from './fixtures';
import { createPublishedStorefront } from './published_storefront_fixture';

test('team copies only a refreshed owned published storefront and withdraws revoked code', async ({ page, baseURL, context }) => {
  const { headers, receipt } = await createPublishedStorefront(page, baseURL);
  await context.grantPermissions(['clipboard-read', 'clipboard-write']);
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
