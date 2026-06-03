import { test, expect } from './fixtures';

test.describe('Viral Storefront Embed E2E', () => {
  test('exposes embeddable storefront widget', async ({ page }) => {
    await page.goto('/referrals');
    await expect(page.getByRole('heading', { name: 'Embed on Your Website' })).toBeVisible();
    await expect(page.locator('#embed-code')).toContainText('<iframe src="https://mybusiness.ohc.store"');
    await expect(page.getByRole('button', { name: 'Copy Embed Code' })).toBeVisible();
  });
});
