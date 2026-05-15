import { test, expect } from '@playwright/test';

test.describe('Grandmother UX End-to-End Flow Validation Dashboard UI', () => {
  test.use({ viewport: { width: 375, height: 800 } });

  test('Flow 1: Verify Stacked UI Elements for Quick Actions', async ({ page }) => {
    await page.goto('/login');
    await page.fill('input[type="email"]', 'grandma@example.com');
    await page.fill('input[type="password"]', 'password123');
    await page.click('button:has-text("Sign In")');
    await page.waitForURL('**/*');

    // Check all floating action buttons exist and have correct labels
    await expect(page.locator('text=Add Product')).toBeVisible();
    await expect(page.locator('text=Create Post')).toBeVisible();
    await expect(page.locator('text=Share Store')).toBeVisible();
  });

  test('Flow 2: Connect Apps plain language button click works', async ({ page }) => {
    await page.goto('/login');
    await page.fill('input[type="email"]', 'grandma@example.com');
    await page.fill('input[type="password"]', 'password123');
    await page.click('button:has-text("Sign In")');
    await page.waitForURL('**/*');

    await page.click('button:has-text("Menu")');
    await expect(page.locator('button:has-text("Connect Apps")')).toBeVisible();
  });

  test('Flow 3: Missing API technical jargon check', async ({ page }) => {
    await page.goto('/login');
    await page.fill('input[type="email"]', 'grandma@example.com');
    await page.fill('input[type="password"]', 'password123');
    await page.click('button:has-text("Sign In")');
    await page.waitForURL('**/*');

    await page.click('button:has-text("Menu")');
    await page.click('button:has-text("Connect Apps")');

    // Verify API screen uses grandma-friendly terms
    await expect(page.locator('text=Custom Integration')).toBeVisible();
    await expect(page.locator('text=API Endpoint')).toHaveCount(0);
    await expect(page.locator('text=API Keys')).toHaveCount(0);
  });

  test('Flow 4: Add custom app button plain language', async ({ page }) => {
    await page.goto('/login');
    await page.fill('input[type="email"]', 'grandma@example.com');
    await page.fill('input[type="password"]', 'password123');
    await page.click('button:has-text("Sign In")');
    await page.waitForURL('**/*');

    await page.click('button:has-text("Menu")');
    await page.click('button:has-text("Connect Apps")');

    await page.click('text=Show technical details');
    await expect(page.locator('text=Add Custom API')).toHaveCount(0);
  });

  test('Flow 5: Review plain language text for email marketer setup', async ({ page }) => {
    await page.goto('/login');
    await page.fill('input[type="email"]', 'grandma@example.com');
    await page.fill('input[type="password"]', 'password123');
    await page.click('button:has-text("Sign In")');
    await page.waitForURL('**/*');

    await page.click('button:has-text("Manage my AI team")');
    // Ensure "Developer: Capability Connect Scope (JSON)" instead of "API Scope"
    await expect(page.locator('text=Developer: Capability API Scope (JSON)')).toHaveCount(0);
  });
});
