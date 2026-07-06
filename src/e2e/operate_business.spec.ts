import { test, expect } from './fixtures';

test('Maya operates her custom cake business', async ({ page }) => {
  await page.goto('/setup.html');
  await expect(page.getByRole('heading', { name: 'Setup Assistant' })).toBeVisible();
  await page.goto('/dashboard');
  await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
});
