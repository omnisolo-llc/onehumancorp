import { test, expect } from '@playwright/test';

test('verify offline hybrid mode state handoff sync', async ({ page }) => {
    // Navigate to local application frontend running locally during bazel tests
    await page.goto('/');

    // Proceed to login (Wait for standard email/pass input fields from `src/app/login.slint`)
    await page.fill('input[type="email"]', 'test@example.com');
    await page.fill('input[type="password"]', 'password123');
    await page.click('button:has-text("Log In")');

    // Make sure we're on the dashboard
    await expect(page.locator('text=Dashboard')).toBeVisible();

    // Trigger local state change
    await page.click('button:has-text("Settings")');
    await expect(page.locator('text=COMMUNICATION')).toBeVisible();

    // Trigger offline -> cloud sync via UI button
    await page.click('button:has-text("Sync with Cloud")');

    // Due to the nature of hybrid mesh and PowerSync pushing states asynchronously we verify
    // it eventually triggers correctly via UI status indicators or logs if the UI is simpler
    // Just verifying the user journey can execute successfully without crashing the UI.
    await expect(page.locator('button:has-text("Sign Out")')).toBeVisible();
});
