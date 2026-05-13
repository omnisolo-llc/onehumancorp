import { test, expect } from '@playwright/test';

test.describe('Grandmother UX End-to-End Flow Validation', () => {
  test.use({ viewport: { width: 375, height: 800 } });

  test.beforeEach(async ({ page }) => {
    await page.goto('/');
  });

  test('Flow 1: Plain language login error', async ({ page }) => {
    await page.evaluate(() => {
        // @ts-ignore
        if (window.showScreen) window.showScreen('login-screen');
    });
    // The button that has "Sign In" is literally "Sign In", the other is "Have an account? Sign In"
    // Use exact text match to find the correct button
    const loginBtns = page.locator('button', { hasText: /^Sign In$/ }).first();
    await loginBtns.click({ force: true });
    // Error message should use plain language
    await expect(page.locator('text=We couldn\'t sign you in. Please check that your email and password are correct, and try again.')).toBeVisible({ timeout: 5000 });
  });

  test('Flow 2: Login button shows loading shimmer state CSS is available', async ({ page }) => {
    await page.evaluate(() => {
        // @ts-ignore
        if (window.showScreen) window.showScreen('login-screen');
    });
    const btn = page.locator('button', { hasText: /^Sign In$/ }).first();
    await btn.click({ force: true });
    // Text changes to Signing in... and skeleton class is added
    const loadingBtn = page.locator('button:has-text("Signing in...")').first();
    await expect(loadingBtn).toHaveClass(/skeleton/);
  });

  test('Flow 3: Dashboard simplification - Today\'s Sales metric', async ({ page }) => {
    await page.evaluate(() => {
        // @ts-ignore
        if (window.showScreen) window.showScreen('dashboard-screen');
    });

    // Check for "Today's Sales" plain language metric
    await expect(page.locator('text=Today\'s Sales')).toBeVisible();
    await expect(page.locator('text=$1,240.50')).toBeVisible();
  });

  test('Flow 4: Mobile nav clarity and tap targets', async ({ page }) => {
    await page.evaluate(() => {
        // @ts-ignore
        if (window.showScreen) window.showScreen('dashboard-screen');
    });

    // Check for the 5 most-used actions
    await expect(page.locator('button:has-text("Add Product")')).toBeVisible();
    await expect(page.locator('button:has-text("View Orders")')).toBeVisible();
    await expect(page.locator('button:has-text("Check Messages")').last()).toBeVisible();
    await expect(page.locator('button:has-text("See Analytics")')).toBeVisible();
    await expect(page.locator('button:has-text("Share Store")')).toBeVisible();

    // Check tap target sizes (min-width/height 44px)
    const btn = page.locator('button:has-text("Add Product")');
    const box = await btn.boundingBox();
    if(box) {
        expect(box.width).toBeGreaterThanOrEqual(44);
        expect(box.height).toBeGreaterThanOrEqual(44);
    }
  });

  test('Flow 5: First-Time User Tour contextual hint', async ({ page }) => {
    await page.evaluate(() => {
        // @ts-ignore
        if (window.showScreen) window.showScreen('dashboard-screen');
    });

    // Click the ? icon exactly
    const questionMarkBtn = page.locator('button', { hasText: /^\?$/ }).first();
    await questionMarkBtn.click({ force: true });

    // Verify the plain language hint
    await expect(page.locator('text=This dashboard shows you how your business is doing today.')).toBeVisible();
  });
});
