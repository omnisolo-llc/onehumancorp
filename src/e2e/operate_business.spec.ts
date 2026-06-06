import { test, expect } from './fixtures';

test('Maya operates her custom cake business', async ({ page }) => {
  await page.goto('/website-builder');
  await expect(page.getByRole('heading', { name: 'Your business, live in minutes.' })).toBeVisible();
  await page.goto('/dashboard');
  await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
});
