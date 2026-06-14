import { test, expect } from '@playwright/test';

test.describe('Cart Recovery', () => {
  test('generates cart recovery campaign without mock paywall', async ({ page }) => {
    await page.goto('/cart-recovery');

    // Enter customer name
    await page.getByLabel('Customer Name (Optional preview)').fill('Alice');

    // Enter cart value
    await page.getByLabel('Cart Value (Optional preview)').fill('$45.00');

    // Click generate button
    await page.getByRole('button', { name: 'Generate AI Campaign' }).click();

    // Verify it generates and we see the generated draft section
    await expect(page.locator('text=✨ AI Generated Draft')).toBeVisible();

    // The backend should return text incorporating our inputs
    await expect(page.locator('pre')).toContainText('Alice');
    await expect(page.locator('pre')).toContainText('$45.00');
  });
});
