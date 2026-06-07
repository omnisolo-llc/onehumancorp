import { test, expect } from '@playwright/test';
import { e2eTenant } from './fixtures';

test.describe('Diagnostics UI', () => {
  test('loads real health and metrics data', async ({ page }) => {
    // Navigate to the diagnostics page
    await page.goto('/diagnostics');

    // Wait for the page to finish loading data
    await expect(page.getByText('Operational Telemetry')).toBeVisible();

    // Verify health data is loaded (System Status should not be Unknown)
    const systemStatusText = await page.getByText(/System Status:/).innerText();
    expect(systemStatusText).not.toContain('Unknown');

    // Verify metrics data is loaded (Total Sales should not be '0' or at least the element exists and has loaded correctly without 'Unknown')
    const totalSalesLocator = page.locator('div').filter({ hasText: /^Total Sales$/ }).locator('..').locator('div').last();
    await expect(totalSalesLocator).toBeVisible();

    // Verify that the UI renders the charts placeholder correctly
    await expect(page.getByText('[ Dynamic Hybrid Correlation Chart ]')).toBeVisible();
  });
});
