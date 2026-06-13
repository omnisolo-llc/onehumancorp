import { test, expect } from '@playwright/test';
import * as path from 'path';
import * as fs from 'fs';

test.describe('OHC Setup Wizard Flow', () => {

  test.beforeEach(async ({ page }) => {
      const tauriUiDir = path.join(process.cwd(), 'src/ui/tauri/src/ui');
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
    await page.locator('[data-testid=\"next-step-btn\"][data-next=\"step-context\"]').click();
    await expect(page.getByText('How do you work?')).toBeVisible();

    // Context step
    await page.locator('.radio-option', { hasText: 'Storefront or Cafe' }).click();
    await page.locator('[data-testid=\"next-step-btn\"][data-next=\"step-categories\"]').click();

    // Categories step
    const categorySelect = page.getByTestId('business-categories');
    await expect(categorySelect).toBeVisible();
    await page.waitForTimeout(100);
    await categorySelect.selectOption('Bakery');
    await page.locator('[data-testid=\"next-step-btn\"][data-next=\"step-name\"]').click();

    // Name step
    await page.getByTestId('business-name').fill('Test Bakery');
    await page.locator('[data-testid=\"next-step-btn\"][data-next=\"step-assistant\"]').click();

    // Assistant step
    await page.getByTestId('assistant-name').fill('Buddy');
    await page.getByTestId('assistant-tone').selectOption('Friendly');
    await page.locator('[data-testid=\"next-step-btn\"][data-next=\"step-admin\"]').click();

    // Admin step
    await page.getByTestId('admin-email').fill('admin@testbakery.local');
    await page.getByTestId('admin-password').fill('SuperSecretPassword123');
    await page.locator('[data-testid=\"next-step-btn\"][data-next=\"step-offer\"]').click();

    // Offer step
    await page.getByTestId('first-offer').fill('Chocolate Cake');
    await page.locator('[data-testid=\"next-step-btn\"][data-next=\"step-template\"]').click();

    // Template step
    await page.getByTestId('template-selection').selectOption('Modern');

    // Make sure finish btn is visible before interacting
    await expect(page.getByTestId('finish-btn')).toBeVisible();

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
    const saveDraftBtn = page.getByTestId('save-draft-btn').last();
    await expect(saveDraftBtn).toBeVisible();
    await saveDraftBtn.click();
    await expect(saveDraftBtn).toHaveText('Saved!', { timeout: 3000 });

    await page.route('**/success.html', async route => {
      await route.fulfill({ status: 200, body: 'Success' });
    });

    // Submit setup
    await page.getByTestId('finish-btn').click();

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

    await page.locator('[data-testid=\"next-step-btn\"][data-next=\"step-context\"]').click();
    const inputbox = await page.locator('.radio-option').first().boundingBox();
    expect(inputbox?.height).toBeGreaterThanOrEqual(44);
  });
});

test.describe('OHC Setup Wizard Form Configuration', () => {

  test.beforeEach(async ({ page }) => {
      const tauriUiDir = require('path').join(process.cwd(), 'src/ui/tauri/src/ui');
      await page.route('**/setup.html', async route => {
          const content = require('fs').readFileSync(require('path').join(tauriUiDir, 'setup.html'), 'utf-8');
          await route.fulfill({ contentType: 'text/html', body: content });
      });
  });

  test('should have appropriate HTML attributes for mobile input configuration', async ({ page }) => {
    await page.goto('http://mock/setup.html');

    // Name step
    const businessName = page.getByTestId('business-name');
    await expect(businessName).toHaveAttribute('autocomplete', 'organization');

    // Admin step
    const adminEmail = page.getByTestId('admin-email');
    await expect(adminEmail).toHaveAttribute('autocomplete', 'email');
    await expect(adminEmail).toHaveAttribute('inputmode', 'email');

    const adminPassword = page.getByTestId('admin-password');
    await expect(adminPassword).toHaveAttribute('autocomplete', 'new-password');
  });

  test('should have border-radius of 16px for .glassmorphism styling', async ({ page }) => {
    // This tests the CSS inline in setup.html and imported globals.css
    await page.goto('http://mock/setup.html');

    const container = page.locator('.container.glassmorphism').first();
    await expect(container).toHaveCSS('border-radius', '16px');

    const textInput = page.locator('#instant-bio');
    // Inputs also use glassmorphism but might be overridden to 8px.
    // However, the mandate specifies containers need 16px.
    await expect(textInput).toHaveCSS('border-radius', '8px');
  });

  test('should support 375px mobile view without horizontal scroll', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812 });
    await page.goto('http://mock/setup.html');

    // Evaluate horizontal scroll
    const hasHorizontalScroll = await page.evaluate(() => {
        return document.documentElement.scrollWidth > window.innerWidth;
    });
    expect(hasHorizontalScroll).toBe(false);
  });

  test('should have minimum 44px touch targets on buttons', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812 });
    await page.goto('http://mock/setup.html');

    // Verify touch targets height
    const btnBox = await page.locator('.next-step-btn').first().boundingBox();
    expect(btnBox?.height).toBeGreaterThanOrEqual(44);
  });

  test('should have minimum 44px touch targets on radio options', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812 });
    await page.goto('http://mock/setup.html');

    await page.locator('[data-testid=\"next-step-btn\"][data-next=\"step-context\"]').click();
    const inputbox = await page.locator('.radio-option').first().boundingBox();
    expect(inputbox?.height).toBeGreaterThanOrEqual(44);
  });
});
