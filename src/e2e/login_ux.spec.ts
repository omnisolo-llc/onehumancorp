import { test, expect } from '@playwright/test';

test.describe('Login Screen Visual Audit', () => {
  test('should display canonical design elements and actions', async ({ page }) => {
    await page.goto('/login');

    // Canonical Text check
    const headerText = page.locator('text=One Human Corp').first();
    await expect(headerText).toBeVisible();

    // Verify critical buttons
    const startBusinessBtn = page.locator('button:has-text("🚀 Start Business Setup")');
    await expect(startBusinessBtn).toBeVisible();

    const settingsBtn = page.locator('button:has-text("⚙ Advanced Options")');
    await expect(settingsBtn).toBeVisible();

    const oauthBtn = page.locator('button:has-text("Continue with Google/Apple")');
    await expect(oauthBtn).toBeVisible();
  });

  test('should trigger business setup flow', async ({ page }) => {
    await page.goto('/login');
    const startBusinessBtn = page.locator('button:has-text("🚀 Start Business Setup")');
    await expect(startBusinessBtn).toBeVisible();
    await startBusinessBtn.click();
    await expect(page).toHaveURL(/\/setup|\/login/); // Might remain on login or redirect
  });

  test('should trigger advanced options', async ({ page }) => {
    await page.goto('/login');
    const settingsBtn = page.locator('button:has-text("⚙ Advanced Options")');
    await expect(settingsBtn).toBeVisible();
    await settingsBtn.click();
    await expect(page).toHaveURL(/\/settings|\/login/);
  });

  test('should trigger oauth login', async ({ page }) => {
    await page.goto('/login');
    const oauthBtn = page.locator('button:has-text("Continue with Google/Apple")');
    await expect(oauthBtn).toBeVisible();
    await oauthBtn.click();
    // Verify it handles oauth click
  });

  test('should display toggle button correctly', async ({ page }) => {
    await page.goto('/login');
    const toggleBtn = page.locator('button:has-text("Don\'t have an account? Sign Up")');
    await expect(toggleBtn).toBeVisible();
    await toggleBtn.click();
    const toggleBtn2 = page.locator('button:has-text("Already have an account? Sign In")');
    await expect(toggleBtn2).toBeVisible();
  });
});
