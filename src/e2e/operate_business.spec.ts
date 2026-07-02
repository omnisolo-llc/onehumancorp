import { test, expect } from './fixtures';

test('Maya operates her custom cake business', async ({ page }) => {
  await page.goto('/setup.html');
  await expect(page.locator('h1', { hasText: 'Tell us about your business' })).toBeVisible({ timeout: 15000 });
  await page.goto('/dashboard');
  await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
});
