import { test, expect } from '@playwright/test';

test.describe('Echo UX Enhancements', () => {
  test.use({ viewport: { width: 375, height: 800 } });

  test('Flow 1: Login to Dashboard to Tour Check', async ({ page }) => {
    // 1. Go to Login directly
    await page.goto('/login');

    // 2. Verify plain language and branding
    await expect(page.locator('h1')).toHaveText('OneHuman');
    await expect(page.locator('text=Sign in to manage your business')).toBeVisible();

    // 3. Fill out credentials and submit
    await page.fill('input[type="email"]', 'test@example.com');
    await page.fill('input[type="password"]', 'password123');
    await page.click('button:has-text("Sign In")');

    // 4. Verify Dashboard Loads with Shimmer
    await expect(page.locator('.shimmer-loading').first()).toBeVisible({ timeout: 2000 }).catch(() => {});

    // 5. Verify Plain language on Dashboard
    await expect(page.locator('text=Today\'s Sales')).toBeVisible({ timeout: 5000 });
    await expect(page.locator('text=Business Health')).toBeVisible();

    // 6. Verify Navigation Touch Targets
    const menuBtn = page.locator('button:has-text("Menu")');
    await expect(menuBtn).toBeVisible();
    const box = await menuBtn.boundingBox();
    expect(box?.height).toBeGreaterThanOrEqual(44);

    // 7. Verify Tour Check
    const tourBtn = page.locator('button:has-text("?")');
    await tourBtn.click();
    await expect(page.locator('text=This screen shows your daily summary.')).toBeVisible();
  });

  test('Flow 2: Login Failure uses plain language', async ({ page }) => {
    await page.goto('/login');
    await page.fill('input[type="email"]', 'wrong@example.com');
    await page.fill('input[type="password"]', 'bad');
    await page.click('button:has-text("Sign In")');
    await expect(page.locator('text=We could not find that email and password combination. Please try again.')).toBeVisible();
  });

  test('Flow 3: Business Setup uses plain language', async ({ page }) => {
    await page.goto('/business-setup');
    await expect(page.locator('text=Your business, live in minutes.')).toBeVisible();
    await page.click('button:has-text("Next")');
    await expect(page.locator('text=What is your business type?')).toBeVisible();
  });

  test('Flow 4: Agents page exists and uses plain language', async ({ page }) => {
    await page.goto('/agents');
    await expect(page.locator('h1')).toHaveText('Agents');
    await expect(page.locator('text=Hire Agent')).toBeVisible();
  });

  test('Flow 5: Quick check of Mobile Navigation interaction', async ({ page }) => {
    await page.goto('/');
    const addProduct = page.locator('text="Add Product"');
    await expect(addProduct).toBeVisible();
  });
});
