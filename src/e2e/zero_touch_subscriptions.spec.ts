import { test, expect } from '@playwright/test';
import { adminPage } from './fixtures';

test.describe('Zero-Config AI-Powered Subscriptions', () => {

  test('Merchant can toggle recurring billing on a product', async ({ page }) => {
    // Navigate to product setup
    await page.goto('/products/setup');

    // Wait for the UI
    await expect(page.locator('h1').filter({ hasText: 'Edit Product' })).toBeVisible();

    // Toggle recurring
    const toggle = page.locator('#toggle-recurring');
    await toggle.check();

    // Verify interval select appears
    const select = page.locator('#select-interval');
    await expect(select).toBeVisible();
    await select.selectOption('monthly');

    // Save
    await page.locator('#btn-save-product').click();

    // In E2E we verify it doesn't crash and maybe show a toast or navigation (stubbed here)
  });

  test('Customer can subscribe via 1-tap checkout', async ({ page }) => {
    // Navigate to checkout
    await page.goto('/checkout/subscription');

    // Wait for checkout UI
    await expect(page.locator('h1').filter({ hasText: 'VIP Membership' })).toBeVisible();

    // Click Apple Pay simulate
    await page.locator('#subscribe-with-apple-pay').click();

    // Wait for success screen
    await expect(page.locator('text=Subscribed!')).toBeVisible({ timeout: 5000 });
  });

  test('Customer can manage subscription via magic link portal', async ({ page }) => {
    // Navigate to portal
    await page.goto('/portal/subscription');

    // Wait for portal UI
    await expect(page.locator('h1').filter({ hasText: 'Your Subscription' })).toBeVisible();

    // Pause subscription
    const pauseBtn = page.locator('#btn-pause-sub');
    await expect(pauseBtn).toBeVisible();
    await pauseBtn.click();

    // Verify status changed to paused
    await expect(page.locator('span.bg-yellow-100')).toContainText('paused');

    // Resume subscription
    const resumeBtn = page.locator('#btn-resume-sub');
    await expect(resumeBtn).toBeVisible();
    await resumeBtn.click();

    // Verify status changed to active
    await expect(page.locator('span.bg-green-100')).toContainText('active');
  });

});
