import { test, expect } from './fixtures';
import { createPublishedStorefront } from './published_storefront_fixture';

test.describe('Growth Referral Widget Premium Layout', () => {
  test('retains premium layout and copies a verified published storefront embed', async ({ page, baseURL, context }) => {
    const { operationId, receipt } = await createPublishedStorefront(page, baseURL);
    await expect(page.getByRole('heading', { name: 'Grow Your Team' })).toBeVisible();

    const widget = page.getByRole('region', { name: 'Published storefront embed' });
    await expect(widget.locator('.backdrop-blur-\\[30px\\]')).toHaveCount(1);
    const embedButton = widget.getByRole('button', { name: 'Copy Embed Code' });
    // A saved receipt alone is insufficient; refresh the real publication first.
    await expect(embedButton).toBeDisabled();
    await expect(widget.getByRole('button', { name: 'Check publication' })).toBeEnabled();
    const checked = page.waitForResponse(response => response.request().method() === 'GET'
      && new URL(response.url()).pathname === `/api/v1/builder/publications/operations/${operationId}`);
    await widget.getByRole('button', { name: 'Check publication' }).click();
    expect((await checked).status()).toBe(200);
    await expect(embedButton).toBeEnabled();

    const code = await widget.getByRole('textbox', { name: 'Published embed code' }).inputValue();
    expect(code).toContain(`<iframe src="${new URL(receipt.public_path, baseURL).href}"`);
    expect(code).toContain('⚡ OmniSolo');
    expect(code).not.toContain('referrals/click');
    await context.grantPermissions(['clipboard-read', 'clipboard-write']);
    const rechecked = page.waitForResponse(response => response.request().method() === 'GET'
      && new URL(response.url()).pathname === `/api/v1/builder/publications/operations/${operationId}`);
    await embedButton.click();
    expect((await rechecked).status()).toBe(200);
    await expect(widget.getByRole('status')).toHaveText('Copied to clipboard.');
    expect(await page.evaluate(() => navigator.clipboard.readText())).toBe(code);
  });
});
