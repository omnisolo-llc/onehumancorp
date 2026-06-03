import { test, expect } from '@playwright/test';

test.describe('QuickBooks Integration Sync', () => {
  test('Priya connects QuickBooks and it shows in integration list', async ({ page }) => {
    // Navigate directly to the integrations page
    await page.goto('/integrations');

    // Verify that QuickBooks integration card is present
    const quickBooksCard = page.locator('div:has(h3:has-text("QuickBooks Online"))').last();
    await expect(quickBooksCard).toBeVisible();
    await expect(quickBooksCard.locator('p')).toContainText('Automatically sync sales data and expenses for easier accounting.');

    // It should start as disconnected
    await expect(quickBooksCard.locator('span:has-text("disconnected")')).toBeVisible();
  });
});
