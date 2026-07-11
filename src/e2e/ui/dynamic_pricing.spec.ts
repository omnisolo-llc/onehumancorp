import { test, expect } from '../fixtures';

test.describe('Dynamic Pricing Engine (Mobile First)', () => {

  test('Owner can preview and approve an AI suggested flash sale', async ({ page, adminUser, loginAs }) => {
    await page.setViewportSize({ width: 375, height: 667 });

    await loginAs(page, adminUser);

    // Should be on dashboard
    await page.goto('/dashboard');

    // Check for the AI Advisory card for dynamic pricing
    await expect(page.locator('text=✨ 3 items are moving slow. Run a flash sale to clear them out?')).toBeVisible();

    // Click Preview Sale
    await page.click('text=Preview Sale');

    // Ensure we reached the preview screen
    await page.waitForURL('**/smart-pricing/preview');
    await expect(page.locator('text=Review Proposed Sale')).toBeVisible();
    await expect(page.locator('text=Suggested Discount')).toBeVisible();
    await expect(page.locator('text=20% off for 48 hours')).toBeVisible();

    // Approve the sale
    await page.click('text=Approve & Notify Customers');

    // Wait for the active sale view
    await expect(page.locator('text=Sale Active! 🎉')).toBeVisible({ timeout: 5000 });
    await expect(page.locator('text=0 / 3 items sold so far')).toBeVisible();

    // Return to dashboard
    await page.click('text=Return to Dashboard');
    await page.waitForURL('**/dashboard');
  });

});
