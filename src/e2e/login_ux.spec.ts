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
});
