import { test, expect } from '@playwright/test';

test.describe('Login Screen Visual Audit', () => {
  test('should display canonical design elements and actions', async ({ page }) => {
    await page.goto('/');

    // Canonical Text check
    const headerText = page.locator('text=One Human Corp').first();
    await expect(headerText).toBeVisible({ timeout: 15000 });

    // Verify critical buttons
    const startBusinessBtn = page.locator('button:has-text("🚀 Start Business Setup")').first();
    await expect(startBusinessBtn).toBeVisible();

    const settingsBtn = page.locator('button:has-text("⚙ App Settings")').first();
    await expect(settingsBtn).toBeVisible();

    const oauthBtn = page.locator('button:has-text("Continue with Google/Apple")').first();
    await expect(oauthBtn).toBeVisible();
  });

  test('should handle loading states dynamically on submit', async ({ page }) => {
    await page.goto('/');

    // Fill credentials
    await page.fill('input[type="email"]', 'test@example.com');
    await page.fill('input[type="password"]', 'password123');

    // Click submit
    const submitBtn = page.locator('button:has-text("Sign In")').first();
    await submitBtn.click();

    // Check loading text - relying on the immediate UI state change via Slint
    const loadingBtn = page.locator('button:has-text("Signing in...")').first();
    await expect(loadingBtn).toBeVisible();
    await expect(loadingBtn).toBeDisabled();

    // Toggle button should be disabled during loading
    const toggleBtn = page.locator('button:has-text("Don\'t have an account? Sign Up")').first();
    await expect(toggleBtn).toBeDisabled();
  });

  test('should handle loading states dynamically on signup', async ({ page }) => {
    await page.goto('/');

    // Switch to sign up
    await page.locator('button:has-text("Don\'t have an account? Sign Up")').first().click();

    // Fill credentials
    await page.fill('input[type="email"]', 'new@example.com');
    await page.fill('input[type="password"]', 'password123');

    // Click submit
    const submitBtn = page.locator('button:has-text("Sign Up")').first();
    await submitBtn.click();

    // Check loading text
    const loadingBtn = page.locator('button:has-text("Creating account...")').first();
    await expect(loadingBtn).toBeVisible();
    await expect(loadingBtn).toBeDisabled();
  });
});
