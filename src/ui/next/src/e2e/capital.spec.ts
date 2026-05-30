import { test, expect } from '@playwright/test';

test.describe('Capital Page', () => {
  test('should display capital offers and allow acceptance', async ({ page }) => {
    // Navigate to the capital page
    await page.goto('http://localhost:3000/capital');

    // Wait for the offers to load
    await expect(page.locator('text=Capital Offers')).toBeVisible();

    // Verify the offer details
    await expect(page.locator('text=Advance: $2000.00')).toBeVisible();
    await expect(page.locator('text=One-time fee: $150.00')).toBeVisible();
    await expect(page.locator('text=Repayment: 10% of daily sales until repaid')).toBeVisible();

    // Accept the offer
    page.on('dialog', dialog => dialog.accept());
    await page.click('button:has-text("Accept Offer")');

    // Verify the offer is accepted
    await expect(page.locator('text=Accepted')).toBeVisible();
  });
});
