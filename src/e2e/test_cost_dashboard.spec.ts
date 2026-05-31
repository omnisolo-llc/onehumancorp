import { test, expect } from './fixtures';

test('navigate to cost dashboard', async ({ page }) => {
  await page.goto('/cost-dashboard');

  // Verify dashboard title
  await expect(page.locator('h1').filter({ hasText: 'Business Advisory Dashboard' })).toBeVisible();

  // Verify Cost Transparency section
  await expect(page.locator('h2').filter({ hasText: 'Cost Transparency' })).toBeVisible();

  // Verify Cost Breakdown section
  await expect(page.locator('h2').filter({ hasText: 'Cost Breakdown' })).toBeVisible();
  await expect(page.locator('span').filter({ hasText: 'LLM Usage' })).toBeVisible();
  await expect(page.locator('span').filter({ hasText: 'Storage' })).toBeVisible();
  await expect(page.locator('span').filter({ hasText: 'Payment Fees' })).toBeVisible();
});
