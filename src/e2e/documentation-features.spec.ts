import { test, expect } from '@playwright/test';
import { adminPage } from './fixtures';

test.describe('Documentation Features', () => {
  test('API Docs Link exists in Help Center', async ({ page }) => {
    await adminPage(page);
    await page.goto('/help');
    await page.waitForLoadState('networkidle');

    const apiDocsLink = page.locator('text=Advanced: API Documentation');
    await expect(apiDocsLink).toBeVisible();

    // Check if the link points to the right place
    await expect(apiDocsLink).toHaveAttribute('href', '/api-docs');
  });

  test('Dashboard Tooltips are present', async ({ page }) => {
    await adminPage(page);
    await page.goto('/dashboard');
    await page.waitForLoadState('networkidle');

    // Wait for the widgets to load
    await expect(page.locator('.app-metric-label').first()).toBeVisible({ timeout: 15000 });

    // Check "Customers" tooltip wrapper
    const customersLabelWrapper = page.locator('#customers-metric-tooltip');
    await expect(customersLabelWrapper).toBeVisible();

    // Check "Pending Orders" tooltip wrapper
    const pendingOrdersLabelWrapper = page.locator('#pending-orders-metric-tooltip');
    await expect(pendingOrdersLabelWrapper).toBeVisible();

    // Check "Low Stock" tooltip wrapper
    const lowStockLabelWrapper = page.locator('#low-stock-metric-tooltip');
    await expect(lowStockLabelWrapper).toBeVisible();
  });
});
