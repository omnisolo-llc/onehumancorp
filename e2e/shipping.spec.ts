import { test, expect } from '@playwright/test';

test.describe('Shippo Integration E2E', () => {
  test('A business owner can view shipping rates and purchase a label', async ({ page }) => {
    // Navigate directly to the orders screen for the purpose of the demo
    await page.goto('/dashboard');

    // Simulate user navigating to the Orders screen
    await page.click('text="Orders"');

    // Wait for the orders screen to load
    await expect(page.locator('#orders-screen')).toBeVisible();

    // Check that our demo order is present
    await expect(page.locator('#order-card-demo-123')).toBeVisible();

    // Click "Create Shipping Label" button for the order
    await page.click('#shipping-actions-demo-123 button');

    // Wait for the rates container to appear
    await expect(page.locator('#shipping-rates-container-demo-123')).toBeVisible({ timeout: 10000 });

    // Wait for the rate to show up in the list
    await expect(page.locator('#rates-list-demo-123')).toContainText('USPS');

    // Select the first rate and purchase label
    await page.click('#rates-list-demo-123 button');

    // Wait for label purchase to complete and display tracking info
    await expect(page.locator('#order-tracking-info-demo-123')).toBeVisible({ timeout: 10000 });

    // Check that tracking number is updated
    await expect(page.locator('#tracking-num-demo-123')).not.toBeEmpty();
    await expect(page.locator('#tracking-num-demo-123')).toContainText('TRACK_');

    // Check that status was updated to "Shipped"
    await expect(page.locator('#order-status-demo-123')).toHaveText('Shipped');
  });
});
