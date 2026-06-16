import { test, expect } from '@playwright/test';

test.describe('Discovery Report Navigation', () => {
  test('should navigate from dashboard to discovery report page', async ({ page }) => {
    // Start at dashboard
    await page.goto('/dashboard');
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });

    // Locate the "AI Discovery Report" card and click it
    const discoveryCard = page.locator('a[href="/discovery-report"]');
    await expect(discoveryCard).toBeVisible({ timeout: 15000 });
    await discoveryCard.click();

    // Verify we landed on the Discovery Report page
    await expect(page).toHaveURL(/\/discovery-report/);

    // Verify the page title
    await expect(page.locator('h1', { hasText: 'AI Discovery Report' })).toBeVisible({ timeout: 10000 });

    // Ensure it's not throwing an unhandled runtime error
    // It should show either "Loading...", "No Reports Yet", or the actual report
    const loadingText = page.locator('text=Loading your report...');
    const emptyStateText = page.locator('text=No Reports Yet');
    const reportText = page.locator('text=Optimized');

    await Promise.any([
      expect(loadingText).toBeVisible({ timeout: 10000 }),
      expect(emptyStateText).toBeVisible({ timeout: 10000 }),
      expect(reportText.first()).toBeVisible({ timeout: 10000 })
    ]).catch(() => {
      throw new Error('None of the expected states (loading, empty, or report) were visible.');
    });
  });
});
