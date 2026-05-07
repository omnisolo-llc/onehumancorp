import { test, expect } from '@playwright/test';

test.describe('End-to-End Business Journey', () => {
  test.beforeEach(async ({ page }) => {
    // Go to the local app URL
    await page.goto('http://localhost:3000');
  });

  test('Maya Acquisition to First Sale', async ({ page }) => {
    // 1. Sign up/Login
    await page.fill('input[type="email"]', 'maya@example.com');
    await page.fill('input[type="password"]', 'secure123');
    await page.click('text="Sign In"');

    // 2. Setup Wizard
    await page.waitForSelector('text="Fix Issue Wizard"', { state: 'hidden' }); // wait for login transition
    await page.click('text="Other"'); // Or Bakery
    await page.fill('input[placeholder="Enter your business name"]', "Maya's Vegan Cakes");
    await page.click('text="Next"');
    await page.click('text="Launch My Business"');

    // 3. Dashboard - Approve Draft
    await page.waitForSelector('text="Dashboard"');
    await page.click('text="Approve"');

    // 4. Verify Approval status
    await expect(page.locator('text="Approved"')).toBeVisible();
  });

  test('Carlos Booking and AI Quote', async ({ page }) => {
    await page.fill('input[type="email"]', 'carlos@example.com');
    await page.fill('input[type="password"]', 'secure123');
    await page.click('text="Sign In"');
    await page.waitForSelector('text="Dashboard"');

    // 1-Tap Approve Quote
    await page.click('text="Approve"');
    await expect(page.locator('text="Approved"')).toBeVisible();
  });

  test('Priya Omnichannel Inventory', async ({ page }) => {
    await page.fill('input[type="email"]', 'priya@example.com');
    await page.fill('input[type="password"]', 'secure123');
    await page.click('text="Sign In"');
    await page.waitForSelector('text="Dashboard"');

    // Click Daily Digest insight
    await page.click('text="Red Dress sold out fast. Reorder?"');
    await expect(page.locator('text="Reordered"')).toBeVisible();
  });

  test('Leo Subscription and Retention', async ({ page }) => {
    await page.fill('input[type="email"]', 'leo@example.com');
    await page.fill('input[type="password"]', 'secure123');
    await page.click('text="Sign In"');
    await page.waitForSelector('text="Dashboard"');

    // 1-Tap Approve Check-in Email
    await page.click('text="Approve"');
    await expect(page.locator('text="Approved"')).toBeVisible();
  });

  test('Fatima High-Velocity Pre-orders', async ({ page }) => {
    await page.fill('input[type="email"]', 'fatima@example.com');
    await page.fill('input[type="password"]', 'secure123');
    await page.click('text="Sign In"');
    await page.waitForSelector('text="Dashboard"');

    // Tap "Acknowledged" on an order
    await page.click('text="View Orders"');
    await expect(page.locator('text="Order Details"')).toBeVisible();
  });
});
