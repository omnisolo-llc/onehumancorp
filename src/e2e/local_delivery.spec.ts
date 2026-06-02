import { test, expect } from '@playwright/test';
import { adminPage } from './fixtures';

test.describe('Zero-Config Autonomous Local Delivery & Dispatch Engine', () => {
  adminPage('should allow configuring delivery zones and processing an order', async ({ page }) => {
    // Navigate to Delivery Settings
    await page.goto('/delivery-settings');
    await expect(page.locator('h1')).toContainText('Local Delivery Settings');

    const flatFeeInput = page.locator('input').nth(0);
    await flatFeeInput.fill('10.0');

    const saveButton = page.getByRole('button', { name: 'Save Settings' });
    await saveButton.click();

    page.on('dialog', dialog => dialog.accept());

    // Go to checkout and select Local Delivery
    await page.goto('/checkout');
    const localDeliveryBtn = page.getByRole('button', { name: 'Local Delivery' });
    await localDeliveryBtn.click();

    // After placing order, go to Dispatch
    await page.goto('/delivery-dispatch');
    await expect(page.locator('h1')).toContainText("Today's Route");

    const startRouteBtn = page.getByRole('button', { name: 'Start Route' });
    await startRouteBtn.click();

    const markDeliveredBtn = page.getByRole('button', { name: 'Mark Delivered' }).first();
    await expect(markDeliveredBtn).toBeVisible();
    await markDeliveredBtn.click();
  });
});
