import { test, expect } from '@playwright/test';

test.describe('Performance & SEO Dashboard Widget', () => {

  test('displays edge cache hit ratio, load time, and indexing status', async ({ page }) => {
    await page.addInitScript(() => {
        localStorage.setItem('tenant_id', '11111111-1111-1111-1111-111111111111');
    });

    await page.goto('http://127.0.0.1:18789/dashboard');

    // Wait for the performance SEO card to be visible
    const seoCard = page.locator('text=Performance & SEO');
    await expect(seoCard).toBeVisible({ timeout: 15000 });

    // Assert that the three main metrics are present in the card
    await expect(page.locator('text=Edge Cache Hit Ratio')).toBeVisible();
    await expect(page.locator('text=Est. Storefront Load Time')).toBeVisible();
    await expect(page.locator('text=Search Indexing')).toBeVisible();

    // Verify correct styling class usage (glassmorphism/premium style)
    const activeSpan = page.locator('text=Active').first();
    await expect(activeSpan).toHaveClass(/bg-\[#34C759\]/);
  });
});
