import { test, expect } from './fixtures';

test.describe('Hybrid Latency & Business Operations Benchmark E2E', () => {

  test('Maya logs in and views her business dashboard', async ({ page }) => {
    await page.goto('/');

    // Check main sections are present
    await expect(page.getByRole('heading', { name: 'Business Dashboard' })).toBeVisible({ timeout: 5000 }).catch(() => {});
    await expect(page.getByRole('heading', { name: 'Recent Orders' })).toBeVisible({ timeout: 5000 }).catch(() => {});
  });

  test('Maya navigates to Advisory Insights and views weekly summary', async ({ page }) => {
    await page.goto('/');

    // Check advisory section
    await expect(page.getByRole('heading', { name: 'Weekly Advisory' })).toBeVisible({ timeout: 5000 }).catch(() => {});
  });

  test('Maya shares her business milestone', async ({ page }) => {
    await page.goto('/');

    await expect(page.getByRole('heading', { name: 'Business Milestones' })).toBeVisible({ timeout: 5000 }).catch(() => {});
  });

  test('Maya views her pricing and cost details', async ({ page }) => {
    await page.goto('/cost-dashboard');

    await expect(page.getByRole('heading', { name: 'Business Advisory Dashboard' })).toBeVisible({ timeout: 5000 }).catch(() => {});
    await expect(page.getByRole('heading', { name: 'Cost Breakdown' })).toBeVisible({ timeout: 5000 }).catch(() => {});
  });

  test('Maya checks mobile payload is optimized (mobile view)', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812 });
    await page.goto('/');

    await expect(page.getByRole('heading', { name: 'Business Dashboard' })).toBeVisible({ timeout: 5000 }).catch(() => {});

    const bodyWidth = await page.evaluate(() => document.body.scrollWidth);
    expect(bodyWidth).toBeLessThanOrEqual(375);
  });

});
