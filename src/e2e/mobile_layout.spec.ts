import { test, expect } from './fixtures';

test.describe('Mobile First Design', () => {
  test.use({ viewport: { width: 375, height: 667 } }); // iPhone SE viewport

  test('app shell is responsive and usable at 375px', async ({ page }) => {
    // Navigate using relative URL to onboarding which doesn't require auth/setup
    await page.goto('/onboarding');

    // Wait for the page to load by checking for the AppShell title
    await expect(page.locator('.app-title')).toBeVisible();

    // Check that we can see the sidebar navigation icons, but we might be in a row
    const nav = page.locator('.app-nav').first();
    await expect(nav).toBeVisible();
  });
});
