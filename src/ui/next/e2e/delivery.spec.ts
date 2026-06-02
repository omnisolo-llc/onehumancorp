import { test, expect } from '@playwright/test';

test.describe('Zero-Config Autonomous Local Delivery & Dispatch Engine', () => {

  test('Owner can configure local delivery zone', async ({ page }) => {
    // Navigate directly to the delivery admin page (mocking an authenticated owner)
    await page.goto('http://localhost:3000/delivery');

    // Assert the page loaded
    await expect(page.locator('h1')).toHaveText('Local Delivery Dispatch');

    // Enable local delivery
    await page.check('input[type="checkbox"]');

    // Fill in zip codes
    await page.fill('input[placeholder="e.g. 10001, 10002, 10003"]', '90210, 10001');

    // Fill in flat fee and min order
    const inputs = page.locator('input[type="number"]');
    await inputs.nth(0).fill('12.50'); // Flat fee
    await inputs.nth(1).fill('25'); // Min order

    // Save settings
    await page.click('button:has-text("Save Zone Settings")');

    // Assert mock api behavior - the mock sets loading to false and displays tasks
    await expect(page.locator('h3:has-text("Today\'s Route")')).toBeVisible();
    await expect(page.locator('text=Stop 1')).toBeVisible();
  });

  test('Owner can manage delivery tasks on driver app view', async ({ page }) => {
    await page.goto('http://localhost:3000/delivery');

    // Ensure tasks are loaded
    await expect(page.locator('text=123 Main St, New York, NY 10001')).toBeVisible();

    // Update task status
    await page.locator('.bg-black:has-text("Start Route")').first().click();

    // Assert the state changed to IN_TRANSIT visually
    await expect(page.locator('span.text-yellow-700').first()).toContainText('IN_TRANSIT');

    // Mark as delivered
    await page.locator('.bg-green-500:has-text("Mark Delivered")').first().click();

    // Assert the state changed to DELIVERED visually
    await expect(page.locator('span.text-green-700').first()).toContainText('DELIVERED');
  });

  test('Customer sees local delivery option at checkout', async ({ page }) => {
    await page.goto('http://localhost:3000/checkout');

    // Customer checks local delivery
    await page.check('input[type="checkbox"]');

    // Delivery address input should become visible
    await expect(page.locator('input[placeholder="Enter your delivery address"]')).toBeVisible();

    // Enter an address
    await page.fill('input[placeholder="Enter your delivery address"]', '123 Fake St, 90210');

    // Proceed to pay
    await page.click('button:has-text("Pay Now")');

    // Assert successful payment modal
    await expect(page.locator('h2')).toHaveText('Payment Successful!');
  });
});
