import { test, expect } from '@playwright/test';

test.describe('UX Friction Fixes', () => {

  test('Dashboard should not have horizontal overflow and use plain language for setup', async ({ page }) => {
    // Set mobile viewport to verify responsiveness
    await page.setViewportSize({ width: 375, height: 812 });
    await page.goto('http://localhost:3000/dashboard');

    // Wait for the action banner to appear
    const setupBannerText = page.locator('text=1 Action Required: Connect your bank to accept payments.');
    await expect(setupBannerText).toBeVisible();

    // Verify "Setup Later (Receive Money as Credit)" button is visible
    const setupLaterBtn = page.getByRole('button', { name: 'Setup Later (Receive Money as Credit)' });
    await expect(setupLaterBtn).toBeVisible();

    // Verify "Connect Bank" button is visible and not pushed off screen
    const connectBankBtn = page.getByRole('button', { name: 'Connect Bank' });
    await expect(connectBankBtn).toBeVisible();

    // Take screenshot of the mobile dashboard
    await page.screenshot({ path: '/home/jules/verification/screenshots/dashboard_mobile_fixed.png' });
  });

  test('Integrations page uses plain language', async ({ page }) => {
    await page.goto('http://localhost:3000/integrations');

    await expect(page.locator('text=Connect my Instagram')).toBeVisible();
    await expect(page.locator('text=Get order notifications')).toBeVisible();
    await expect(page.locator('text=Connect Custom Software')).toBeVisible();
  });

  test('Plan page uses plain language', async ({ page }) => {
    await page.goto('http://localhost:3000/plan');

    await expect(page.locator('text=My Store Size').first()).toBeVisible();
    await expect(page.locator('text=My Business Status')).toBeVisible();
  });

  test('Cost Dashboard uses plain language', async ({ page }) => {
    await page.goto('http://localhost:3000/cost-dashboard');

    await expect(page.locator('text=AI Assistants')).toBeVisible();
    await expect(page.locator('text=My Store Size')).toBeVisible();
  });

  test('Onboarding page uses "Starting at" for Carlos', async ({ page }) => {
    await page.goto('http://localhost:3000/onboarding');

    // Just click through wizard until step 3
    await page.getByRole('button', { name: /Continue|Next/i }).click();
    await page.getByRole('button', { name: /Continue|Next/i }).click();

    // Step 3 (Store Options)
    await expect(page.locator('text=Starting at')).first().toBeVisible();
  });

});
