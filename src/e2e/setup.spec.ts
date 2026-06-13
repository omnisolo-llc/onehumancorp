import { test, expect } from '@playwright/test';
import * as path from 'path';
import * as fs from 'fs';

test.describe('OHC Setup Wizard Flow', () => {

  test.beforeEach(async ({ page }) => {
      const workspaceRoot = process.env.TEST_WORKSPACE ? path.join(process.env.TEST_SRCDIR, process.env.TEST_WORKSPACE) : process.cwd();
      const tauriUiDir = path.join(workspaceRoot, 'src/ui/tauri/src/ui');
      await page.route('**/setup.html', async route => {
          const content = fs.readFileSync(path.join(tauriUiDir, 'setup.html'), 'utf-8');
          await route.fulfill({ contentType: 'text/html', body: content });
      });
  });

  test('should complete the interactive setup wizard flow smoothly on desktop', async ({ page }) => {
    // intercept tooltips
    await page.route('**/api/tooltips', async route => {
      await route.fulfill({ status: 200, body: JSON.stringify({}) });
    });

    await page.goto('http://mock/setup.html');

    // Check initial UI loading
    await expect(page.locator('h1').first()).toBeVisible();
    // Start My Business
    await page.getByRole('button', { name: "Start My Business" }).click();
    await expect(page.getByText('How do you work?')).toBeVisible();

    // Context step
    await page.locator('.radio-option', { hasText: 'Storefront or Cafe' }).click();
    await page.locator('.next-step-btn[data-next="step-categories"]').click();

    // Categories step
    const categorySelect = page.locator('#business-categories');
    await expect(categorySelect).toBeVisible();
    await page.waitForTimeout(100);
    await categorySelect.selectOption('Bakery');
    await page.locator('.next-step-btn[data-next="step-name"]').click();

    // Name step
    await page.locator('#business-name').fill('Test Bakery');
    await page.locator('.next-step-btn[data-next="step-assistant"]').click();

    // Assistant step
    await page.locator('#assistant-name').fill('Buddy');
    await page.locator('#assistant-tone').selectOption('Friendly');
    await page.locator('.next-step-btn[data-next="step-admin"]').click();

    // Admin step
    await page.locator('#admin-email').fill('admin@testbakery.local');
    await page.locator('#admin-password').fill('SuperSecretPassword123');
    await page.locator('.next-step-btn[data-next="step-offer"]').click();

    // Offer step
    await page.locator('#first-offer').fill('Chocolate Cake');
    await page.locator('.next-step-btn[data-next="step-template"]').click();

    // Template step
    await page.locator('#template-selection').selectOption('Modern');

    // Make sure finish btn is visible before interacting
    await expect(page.locator('#finish-btn')).toBeVisible();

    // Intercept backend call
    await page.route('**/api/onboarding/start', async route => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({ organization_id: 'test-org-123' })
      });
    });

    await page.route('**/api/onboarding/state', async route => {
       await route.fulfill({ status: 200, body: JSON.stringify({}) });
    });

    // Click Save Draft
    const saveDraftBtn = page.locator('.save-draft-btn').last();
    await expect(saveDraftBtn).toBeVisible();
    await saveDraftBtn.click();
    await expect(saveDraftBtn).toHaveText('Saved!', { timeout: 3000 });

    await page.route('**/success.html', async route => {
      await route.fulfill({ status: 200, body: 'Success' });
    });

    // Submit setup
    await page.locator('#finish-btn').click();

    await page.waitForURL('**/success.html', { timeout: 10000 });
    await expect(page.url()).toContain('success.html');
  });

  test('should support 375px mobile view without horizontal scroll and minimum 44px touch targets', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812 });
    // intercept tooltips
    await page.route('**/api/tooltips', async route => {
      await route.fulfill({ status: 200, body: JSON.stringify({}) });
    });

    await page.goto('http://mock/setup.html');

    // Evaluate horizontal scroll
    const hasHorizontalScroll = await page.evaluate(() => {
        return document.documentElement.scrollWidth > window.innerWidth;
    });
    expect(hasHorizontalScroll).toBe(false);

    // Verify touch targets height
    const btnBox = await page.locator('.next-step-btn').first().boundingBox();
    expect(btnBox?.height).toBeGreaterThanOrEqual(44);

    await page.getByRole('button', { name: "Start My Business" }).click();
    const inputbox = await page.locator('.radio-option').first().boundingBox();
    expect(inputbox?.height).toBeGreaterThanOrEqual(44);
  });
});
