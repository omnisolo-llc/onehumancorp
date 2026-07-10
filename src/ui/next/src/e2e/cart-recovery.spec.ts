import { test, expect } from '@playwright/test';

test.describe('Cart Recovery E2E', () => {
  test('should display Powered by OHC component when generating draft', async ({ page }) => {
    // Navigate using relative URL
    await page.goto('/cart-recovery');

    // Check initial state (should ask to generate draft)
    await expect(page.locator('text=Configure your campaign to generate a high-converting recovery draft.')).toBeVisible({ timeout: 10000 });

    // Click generate
    await page.getByRole('button', { name: 'Generate AI Campaign' }).click();

    // The mock or actual logic might take a bit. Wait for draft.
    // Once it loads, check that the PoweredByOHC footer is there
    // Using string matching to avoid locator issues
    await page.waitForTimeout(2000);
    const html = await page.innerHTML('body');
    expect(html).toMatch(/Powered by OHC/i);
  });
});
