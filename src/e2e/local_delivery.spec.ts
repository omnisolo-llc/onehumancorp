import { test, expect } from './fixtures';

test.describe('Local Delivery & Dispatch CUJ', () => {
  test('A business owner can configure a delivery zone, and a customer can select it at checkout, resulting in a dispatch task', async ({ page }) => {
    // 1. Owner navigates to Delivery Settings and configures zone
    await page.goto('/settings/delivery');
    await expect(page.locator('h1')).toContainText('Configure Local Delivery');

    const saveButton = page.locator('#save-delivery-zone');
    await expect(saveButton).toBeVisible();
    await saveButton.click();

    // Simulating alert dismissal natively
    page.on('dialog', dialog => dialog.accept());

    // 2. Customer navigates to checkout to buy a product and sees local delivery option
    await page.goto('/checkout');
    await expect(page.locator('h1')).toContainText('Checkout');

    const localDeliveryCheckbox = page.locator('#localDeliveryCheckbox');
    await expect(localDeliveryCheckbox).toBeVisible();

    // Select Local Delivery
    await localDeliveryCheckbox.check();
    await expect(localDeliveryCheckbox).toBeChecked();

    // 3. Driver/Owner opens Dispatch App
    await page.goto('/operations/delivery');
    await expect(page.locator('h1')).toContainText("Today's Route");

    const markDeliveredButton = page.locator('#mark-delivered-1');
    await expect(markDeliveredButton).toBeVisible();

    // Mark first task as delivered
    await markDeliveredButton.click();

    // Assert status updated visually
    await expect(page.locator('text=DELIVERED').first()).toBeVisible();
  });
});
