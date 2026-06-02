import { test, expect } from '@playwright/test';

test.describe('Checkout Delivery UI', () => {
  test('should display Local Delivery option on checkout page', async ({ page }) => {
    // Note: in a real E2E environment with auth, we'd need to mock login or navigate from a product page.
    // For this simple UI unit verification via playwright, we go straight to checkout.
    await page.goto('http://localhost:3000/checkout');

    // Verify the checkout screen loads
    await expect(page.locator('#checkout-screen')).toBeVisible();

    // Verify the Local Delivery button exists
    const localDeliveryBtn = page.locator('button:has-text("Local Delivery")');
    await expect(localDeliveryBtn).toBeVisible();

    // Setup an event listener for the alert dialog triggered by clicking it
    let dialogMessage = "";
    page.on('dialog', async dialog => {
      dialogMessage = dialog.message();
      await dialog.accept();
    });

    // Click the Local Delivery button
    await localDeliveryBtn.click();

    // Verify the alert text matches expected
    expect(dialogMessage).toContain('Local Delivery selected!');

    // The Success Modal should become visible
    await expect(page.locator('text=Payment Successful!')).toBeVisible();
  });
});
