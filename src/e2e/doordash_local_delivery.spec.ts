import { test, expect } from '@playwright/test';

test.describe('DoorDash Local Delivery Flow', () => {

  test('Settings configuration, checkout with local delivery, and dispatch driver', async ({ page }) => {
    // Navigate to dashboard and settings
    await page.goto('/dashboard');
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();

    // Go to Settings screen
    await page.goto('/settings');
    await expect(page.getByRole('heading', { name: 'Settings' })).toBeVisible();

    // Enable Local Delivery
    await page.getByLabel('Enable Local Delivery').check();

    // Fill out the delivery configuration
    await page.getByPlaceholder('e.g. 5').fill('10'); // Delivery Radius
    await page.getByPlaceholder('e.g. 30').fill('45'); // Preparation Time
    await page.getByPlaceholder('e.g. 7.50').fill('8.50'); // Flat Delivery Fee

    await page.getByRole('button', { name: 'Save' }).click();

    // Verify returning to the dashboard
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();

    // Navigate to checkout
    await page.goto('/checkout');
    await expect(page.getByRole('heading', { name: 'Checkout' })).toBeVisible();

    // Check 'Include Local Delivery'
    await page.getByLabel('Include Local Delivery (Powered by DoorDash)').check();

    // Test Address that falls inside the radius
    await page.getByPlaceholder('Enter your delivery address').fill('123 Main St, Local City, LC 12345');
    await page.getByRole('button', { name: 'Check Availability & Fee' }).click();

    // Expect success message from the mocked backend
    await expect(page.getByText('Local delivery available! Flat fee: $7.50')).toBeVisible();

    // Pay
    await page.getByRole('button', { name: 'Pay Now' }).click();

    // Ensure success modal appears
    await expect(page.getByText('Payment Successful!')).toBeVisible();

    // Navigate to fulfillment hub
    await page.goto('/fulfillment-hub');
    await expect(page.getByRole('heading', { name: 'Fulfillment Hub' })).toBeVisible();

    // Locate the first Local Delivery order
    const localOrderCard = page.locator('.group').filter({ hasText: '🚚 Local' }).first();
    await expect(localOrderCard).toBeVisible();

    // Click 'Mark Ready' (simulates prep complete, moves order to awaiting pickup)
    await localOrderCard.getByRole('button', { name: 'Mark Ready' }).click();

    // Verify it moved to Awaiting Pickup section and status changed
    await expect(page.getByText('Ready for LocalDelivery')).toBeVisible();

    // Dispatch a driver
    await page.getByRole('button', { name: 'Request Driver (DoorDash)' }).first().click();

    // Assuming the "Request Driver (DoorDash)" button disappears or changes state upon clicking
    await expect(page.getByRole('button', { name: 'Request Driver (DoorDash)' })).toHaveCount(0);

    // Ensure "Swipe to Hand Off" is available
    await expect(page.getByRole('button', { name: 'Swipe to Hand Off' }).first()).toBeVisible();

  });

  test('Test delivery address outside radius', async ({ page }) => {
    // Navigate to checkout
    await page.goto('/checkout');
    await expect(page.getByRole('heading', { name: 'Checkout' })).toBeVisible();

    // Check 'Include Local Delivery'
    await page.getByLabel('Include Local Delivery (Powered by DoorDash)').check();

    // Test Address that falls outside the radius
    // We handle the browser 'alert' dialog
    page.on('dialog', dialog => dialog.accept());

    await page.getByPlaceholder('Enter your delivery address').fill('outside the city');
    await page.getByRole('button', { name: 'Check Availability & Fee' }).click();

    // Expect fee to not be present
    await expect(page.getByText('Local delivery available! Flat fee: $7.50')).not.toBeVisible();
  });
});
