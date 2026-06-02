import { expect } from '@playwright/test';
import { test } from './fixtures';

test.describe('Subscription Engine', () => {
  test('CUJ: Create a subscription plan and print fulfillment batch', async ({ adminPage: page }) => {
    // Navigate to the Dashboard
    await page.goto('/dashboard/subscriptions');

    // Create Subscription Plan
    await page.click('text=Add Product');

    await page.fill('input[name="name"]', 'Monthly Coffee Bean');
    await page.fill('input[name="price"]', '25');
    await page.selectOption('select[name="billing_interval"]', 'month');

    await page.click('button:has-text("Save")');
    await expect(page.locator('text=Subscription saved successfully')).toBeVisible();

    // Verify Fulfillment Dashboard Widget
    await page.goto('/dashboard/subscriptions/fulfillment');
    await expect(page.locator('text=Active Subscribers')).toBeVisible();
    await expect(page.locator('text=Upcoming Fulfillment')).toBeVisible();

    // There shouldn't be labels until subscribed, but we should see the list
    await expect(page.locator('text=Fulfillment Batches')).toBeVisible();
    await expect(page.locator('text=Monthly Coffee Bean')).toBeVisible();
  });
});
