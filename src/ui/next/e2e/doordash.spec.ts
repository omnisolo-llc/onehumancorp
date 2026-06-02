import { test, expect } from '@playwright/test';

test.describe('DoorDash Local Delivery Flow', () => {
  // Use mock for the fetch request so it resolves correctly, in e2e mode we use interceptor
  test.beforeEach(async ({ page }) => {
    await page.route('/api/v1/growth/referrals/generate', async route => {
      const json = { referral_link: 'http://ohc.store/join?ref=test-tenant' };
      await route.fulfill({ json });
    });
  });

  test('configures doordash, places order with local delivery, and dispatches driver', async ({ page }) => {
    // 1. Owner configures DoorDash in Settings
    await page.goto('/settings');
    await expect(page.getByRole('heading', { name: 'Settings' })).toBeVisible();

    // Enable Local Delivery
    await page.locator('label:has-text("Enable Local Delivery") > input').check();

    // Check if radius/prep time inputs appear
    await expect(page.getByLabel('Delivery Radius (miles)')).toBeVisible();
    await expect(page.getByLabel('Preparation Time (minutes)')).toBeVisible();

    // Save settings
    await page.getByRole('button', { name: 'Save' }).click();

    // 2. Customer places order with local delivery
    await page.goto('/checkout');
    await expect(page.getByRole('heading', { name: 'Delivery Information' })).toBeVisible();

    // Enter address
    await page.getByPlaceholder('e.g. 123 Main St, San Francisco, CA').fill('456 Delivery Ave');

    // Check quote
    await page.getByRole('button', { name: 'Check Local Delivery Quote' }).click();
    await expect(page.getByText('Checking availability...')).toBeVisible();

    // Verify fee is added
    await expect(page.getByText('Address is within delivery radius. DoorDash delivery fee: $7.50')).toBeVisible();
    await expect(page.getByText('Local Delivery (DoorDash)')).toBeVisible();
    await expect(page.getByText('$52.50')).toBeVisible();

    // Pay Now
    await page.getByRole('button', { name: 'Pay Now' }).click();
    await expect(page.getByText('Payment Successful!')).toBeVisible();

    // 3. Owner dispatches driver in Orders
    await page.goto('/orders');

    // Find the newly created order (it will be unfulfilled and have a total of $52.50)
    // We click the first 'View' button assuming it's prepended
    await page.getByRole('button', { name: 'View' }).first().click();

    // Verify DoorDash fulfillment card
    await expect(page.getByText('Powered by DoorDash Drive')).toBeVisible();
    await expect(page.getByText('Request DoorDash Driver')).toBeVisible();

    // Request driver
    await page.getByRole('button', { name: 'Request DoorDash Driver' }).click();
    await expect(page.getByText('Dispatching...')).toBeVisible();

    // Verify driver dispatched and tracking link visible
    await expect(page.getByText('Driver Dispatched')).toBeVisible();
    await expect(page.getByText('A DoorDash driver is on the way to pick up this order.')).toBeVisible();
    await expect(page.getByRole('link', { name: 'View Live Tracking' })).toBeVisible();
  });
});
