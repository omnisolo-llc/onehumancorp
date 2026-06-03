import { test, expect } from '@playwright/test';

test.describe('DoorDash Drive Integration', () => {
  test('should display Local Delivery settings', async ({ page }) => {
    await page.goto('/settings');
    await page.waitForSelector('text=Local Delivery (DoorDash Drive)');
    const isVisible = await page.isVisible('text=Enable Local Delivery via DoorDash Drive');
    expect(isVisible).toBeTruthy();
  });

  test('should fetch delivery fee in checkout', async ({ page }) => {
    await page.goto('/checkout');
    await page.fill('input[placeholder="Delivery Address"]', '123 Test St');
    await page.click('button:has-text("Check")');
    await page.waitForSelector('text=Delivery Available!');
    const feeVisible = await page.isVisible('text=+$8.50');
    expect(feeVisible).toBeTruthy();
  });

  test('should show Request Courier (DoorDash) button in fulfillment hub', async ({ page }) => {
    await page.goto('/fulfillment-hub');
    await page.waitForSelector('text=Request Courier (DoorDash)');
    const buttonVisible = await page.isVisible('button:has-text("Request Courier (DoorDash)")');
    expect(buttonVisible).toBeTruthy();
  });
});
