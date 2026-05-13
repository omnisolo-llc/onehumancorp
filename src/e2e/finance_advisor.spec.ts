import { test, expect } from '@playwright/test';

test('AI Accountant Business Briefing displays on dashboard', async ({ page }) => {
    // Start at login
    await page.goto(process.env.BASE_URL || 'http://localhost:18789/');

    // The default landing page shows options to login or sign up.
    // If there's a login button visible, click it, otherwise check if we're already on login screen
    const loginButton = page.locator('button:has-text("Sign In")');
    if (await loginButton.isVisible()) {
        await loginButton.click();
    }

    // Fill credentials and log in
    await page.locator('#login-screen input[type="email"]').fill('test_finance@owner.com');
    await page.locator('#login-screen button:has-text("Sign In")').click();

    // Wait for the dashboard to render
    const dashboard = page.locator('#dashboard-screen');
    await expect(dashboard).toBeVisible();

    // Verify the "Business Briefing" card is present and visible
    const briefingCard = page.locator('#daily-briefing-card');
    await expect(briefingCard).toBeVisible();

    // Verify the content paragraph is present
    const briefingContent = page.locator('#briefing-content');
    await expect(briefingContent).toBeVisible();
    await expect(briefingContent).not.toBeEmpty();
});
