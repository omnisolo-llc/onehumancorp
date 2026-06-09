import { test, expect } from './fixtures';

test.describe('Cost Dashboard Page', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/cost-dashboard');
  });

  test('should display Cost Breakdown', async ({ page }) => {
    await expect(page.locator('h1', { hasText: 'Cost & Usage Dashboard' })).toBeVisible({ timeout: 10000 });
    await expect(page.locator('h2', { hasText: 'Cost Breakdown' })).toBeVisible();
  });

  test('should display cost components', async ({ page }) => {
    await expect(page.locator('span', { hasText: 'LLM Usage' })).toBeVisible();
    await expect(page.locator('span', { hasText: 'Storage' })).toBeVisible();
    await expect(page.locator('span', { hasText: 'Payment Fees' })).toBeVisible();
    await expect(page.locator('span', { hasText: 'Compute Usage' })).toBeVisible();
    await expect(page.locator('span', { hasText: 'Network & Bandwidth' })).toBeVisible();
  });

  test('should display Department Tier Usage', async ({ page }) => {
    await expect(page.locator('h2', { hasText: 'Department Tier Usage' })).toBeVisible();
  });
});
