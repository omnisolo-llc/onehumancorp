import { test, expect } from '@playwright/test';

test.use({ baseURL: 'http://localhost:8081' });

test.describe('CUJ: Documentation and Onboarding Flows', () => {
  test('Verify Tooltips, Walkthrough, and Help Center', async ({ page }) => {
    // 1. Force semantics to be enabled for Flutter web canvas
    await page.goto('/?FLUTTER_WEB_USE_SKIA=false#/login');
    await page.waitForLoadState('networkidle');

    // Enable semantics in Flutter Web
    await page.evaluate(() => {
      if ((window as any)._flutter_semantics_enable) {
        (window as any)._flutter_semantics_enable();
      }
    });

    // 2. Login via the UI
    const usernameInput = page.locator('input[aria-label="Email"]').or(page.locator('input[name="username"]')).first();
    await usernameInput.waitFor({ state: 'visible', timeout: 15000 });
    await usernameInput.fill('admin');

    const passwordInput = page.locator('input[aria-label="Password"]').or(page.locator('input[name="password"]')).first();
    await passwordInput.fill('admin');

    const loginButton = page.locator('button:has-text("Sign In")').or(page.locator('button:has-text("Login")')).first();
    await loginButton.click();

    // Explicit wait for navigation to dashboard
    await page.waitForURL('**/dashboard', { timeout: 15000 });
    await page.waitForLoadState('networkidle');

    // 3. Trigger Walkthrough
    await page.click('text=Start Tour');
    // Assert Walkthrough popup appears correctly
    await expect(page.locator('text=Welcome to your Dashboard!')).toBeVisible();
    await page.click('text=Finish');

    // 4. Trigger Ask AI Support (which uses RegisteredTooltip)
    await page.click('text=Ask AI Support');
    await expect(page.locator('text=AI Support Agent')).toBeVisible();
    // Click close button inside bottom sheet
    await page.click('button:has-text("Close")');

    // 5. Navigate to Help Center via the sidebar UI
    await page.click('text=Help Center');
    await expect(page).toHaveURL(/.*\/help/);

    // 6. Navigate to Changelog via the Help Center UI
    await page.click('text=What\'s New');
    await expect(page).toHaveURL(/.*\/help\/changelog/);
    // Go back to help center
    await page.goBack();

    // 7. Navigate to API Docs via the Help Center UI
    await page.click('text=API Documentation');
    await expect(page).toHaveURL(/.*\/help\/api-docs/);
  });
});
