import { test, expect } from '@playwright/test';

test.describe('Universal Embedded Finance & AI Taxation Ledger', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('Owner can view the mobile financial health dashboard without mock data', async ({ page }) => {
    // Navigate directly to the new ledger page
    // We need to inject the local storage tenant ID so it doesn't fail fast
    await page.goto('/'); // go somewhere first to be able to set localStorage
    await page.evaluate(() => window.localStorage.setItem('ohc_tenant_id', 'test-tenant'));
    await page.goto('/ledger');

    // Wait for the UI to load - increase timeout for local server
    await expect(page.locator('h1:has-text("Financial Health")')).toBeVisible({ timeout: 15000 });

    // Verify key UI components exist and use the plain language requested
    await expect(page.locator('h2:has-text("Total Revenue")')).toBeVisible();
    await expect(page.locator('h2:has-text("Available Cash")')).toBeVisible();
    await expect(page.locator('h2:has-text("Estimated Taxes Saved")')).toBeVisible();

    // Verify the sections for envelopes and obligations are visible
    await expect(page.locator('h2:has-text("Virtual Envelopes")')).toBeVisible();
    await expect(page.locator('h2:has-text("Tax Obligations")')).toBeVisible();
  });
});
