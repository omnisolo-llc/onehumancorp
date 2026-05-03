import { test, expect } from '@playwright/test';

test('verify SwarmObservability is wired and visible (if data arrives) or empty state', async ({ page }) => {
    // Navigate to the app
    await page.goto('/');

    // Wait for the login screen to be visible
    await page.waitForSelector('text=Email', { state: 'visible', timeout: 5000 }).catch(() => null);

    const emailInput = await page.locator('input[type="email"]').count();
    if (emailInput > 0) {
        // Perform login
        await page.fill('input[type="email"]', 'test@example.com');
        await page.fill('input[type="password"]', 'password123');
        await page.click('text=Login');
    }

    // Wait for network idle
    await page.waitForLoadState('networkidle');

    // Assert that the SwarmObservability "Agent Actions Today" text is still there,
    // because it's wired and not removed.
    const isVisible = await page.locator('text=Agent Actions Today').isVisible();
    expect(isVisible).toBe(true);

    // Optionally check that the mock data mesh message isn't there anymore
    const mockMessageVisible = await page.locator('text=✅ Your Support Agent replied to 3 customers').isVisible();
    expect(mockMessageVisible).toBe(false);
});
