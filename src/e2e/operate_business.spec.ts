import { test, expect } from './fixtures';

test('Maya operates her custom cake business', async ({ page }) => {
  await page.goto('/onboarding');
  await expect(page.getByRole('heading', { name: '10-Minute Setup Wizard' })).toBeVisible();
  await page.goto('/dashboard');
  await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
});
