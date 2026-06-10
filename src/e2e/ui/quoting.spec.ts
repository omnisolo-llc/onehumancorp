import { test, expect } from '@playwright/test';

test.describe('Unified Booking, Quoting & Deposit Engine', () => {
  test('Customer can review and accept a quote', async ({ page }) => {
    // Navigate to quoting interface
    await page.goto('/quoting?quoteId=quote-1');

    // Wait for data to load
    await expect(page.locator('text=Review Draft Quote')).toBeVisible({ timeout: 10000 });

    // Assert elements that should be visible based on our mockup behavior
    await expect(page.locator('text=Approve & Send')).toBeVisible();

    // Setup an interaction test
    page.on('dialog', dialog => dialog.accept());
    await page.click('button:has-text("Approve & Send")');

    // The status should update
    await expect(page.locator('text=Sent to Customer')).toBeVisible();
  });
});