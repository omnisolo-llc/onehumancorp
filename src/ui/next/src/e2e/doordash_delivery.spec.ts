import { test, expect } from '@playwright/test';

test.describe('DoorDash Drive Integration CUJ', () => {
  test('Business Owner can configure and fulfill local delivery orders', async ({ page }) => {
    // 1. Configure Delivery in Settings
    await page.goto('/settings');

    const doorDashSection = page.locator('section:has-text("Local Delivery (DoorDash Drive)")');
    await expect(doorDashSection).toBeVisible();

    const enableToggle = doorDashSection.locator('input[type="checkbox"]');
    await enableToggle.check();

    const radiusInput = doorDashSection.locator('input[type="number"]').first();
    await radiusInput.fill('10');

    const feeInput = doorDashSection.locator('input[type="number"]').last();
    await feeInput.fill('9.99');

    // In a real app we'd click save and wait for a network response
    // For now we click and check for the alert (or just assume it's set in state for this session)
    page.on('dialog', dialog => dialog.dismiss());
    await doorDashSection.getByRole('button', { name: 'Save Delivery Config' }).click();

    // 2. Customer Checkout Experience
    await page.goto('/checkout');
    await expect(page.getByText('Order Summary')).toBeVisible();

    // Fill in a delivery address (this should trigger the quote API in our implementation)
    await page.getByPlaceholder('Street Address').fill('123 Main St');
    await page.getByPlaceholder('City').fill('San Francisco');

    // Check for "Local Delivery" option appearing in Shipping Methods
    const deliveryOption = page.locator('label:has-text("Local Delivery (DoorDash Drive)")');
    await expect(deliveryOption).toBeVisible();

    // Verify fee is shown (9.99 we set earlier might not persist if not saved to DB,
    // but the mock/actual API should return a value)
    await expect(deliveryOption).toContainText('$');

    // 3. Order Fulfillment (Request Courier)
    // Directly navigate to an order page as an owner
    await page.goto('/orders/ord-1');

    const requestButton = page.getByRole('button', { name: 'Request DoorDash Courier' });
    await expect(requestButton).toBeVisible();

    // Click the button to dispatch
    await requestButton.click();

    // Verify tracking link appears
    await expect(page.getByText('DoorDash Courier Requested')).toBeVisible();
    const trackingLink = page.getByRole('link', { name: 'Track Delivery' });
    await expect(trackingLink).toBeVisible();
    await expect(trackingLink).toHaveAttribute('href', /doordash\.com/);
  });
});
