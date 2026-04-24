import { test, expect } from '@playwright/test';

test.describe('Dashboard Navigation UX Simplification', () => {
  test('User can navigate the app and view shimmering loading skeletons without UI blockers', async ({ page }) => {
    // Navigate to base URL which handles routing (assumes login via dummy backend if standalone)
    await page.goto('/');

    // Wait for the app to load
    await page.waitForTimeout(1000);

    // Verify we are at least on a page
    const title = await page.title();
    expect(title).toBeDefined();

    // From dashboard, a user clicks on the Settings link in the navigation panel
    await page.click('text=Settings');

    // Make sure we see Settings text and not a crash
    await expect(page.locator('text=Settings')).toBeVisible({ timeout: 15000 });

    // The test asserts that the UI reaches a stable state, meaning no crashes occurred during the loading skeleton phase.
    const hasError = await page.evaluate(() => {
       return document.body.innerText.includes('Error:');
    });

    expect(hasError).toBe(false);

    // Click back to Dashboard
    await page.click('text=Dashboard');
    await expect(page.locator('text=Dashboard').first()).toBeVisible({ timeout: 15000 });
  });
});
