import { test, expect } from '@playwright/test';

test.describe('In-App Help Documentation features', () => {

  test.beforeEach(async ({ page }) => {
    // Navigate to login
    await page.goto('/login');
    // Login flow
    await page.fill('input[type="email"]', 'ceo@onehumancorp.com');
    await page.fill('input[type="password"]', 'admin123');
    await page.click('button:has-text("Sign in")');
    // Wait for navigation to dashboard
    await expect(page).toHaveURL(/\/dashboard/);
  });

  test('User can navigate to Help Center and view topics', async ({ page }) => {
    // Open the sidebar if on mobile
    const isMobile = await page.evaluate(() => window.innerWidth <= 768);
    if (isMobile) {
      await page.click('button:has(svg.lucide-menu)');
    }

    // Click on Help Center navigation item
    await page.click('text="Help Center"');

    // Wait for navigation
    await expect(page).toHaveURL(/\/help/);

    // Assert Help Center content
    await expect(page.locator('text="Help Center"').first()).toBeVisible();
    await expect(page.locator('input[placeholder="Search for articles, guides, and more..."]')).toBeVisible();
    await expect(page.locator('text="Getting Started"')).toBeVisible();
    await expect(page.locator('text="Video Tutorials"')).toBeVisible();
    await expect(page.locator('text="API Documentation (Advanced)"')).toBeVisible();
  });

  test('User can navigate to Release Notes', async ({ page }) => {
    const isMobile = await page.evaluate(() => window.innerWidth <= 768);
    if (isMobile) {
      await page.click('button:has(svg.lucide-menu)');
    }

    // Click on Release Notes
    await page.click('text="Release Notes"');

    // Wait for navigation
    await expect(page).toHaveURL(/\/release_notes/);

    // Assert Release Notes content
    await expect(page.locator('text="What\'s New in OHC"').first()).toBeVisible();
    await expect(page.locator('text="AI Marketing Agent Update"')).toBeVisible();
    await expect(page.locator('text="NEW"')).toBeVisible();
  });

  test('Global Help Chat floating action button exists on dashboard', async ({ page }) => {
    // Ensure the FAB is visible on the dashboard
    const fab = page.locator('button[aria-label="Ask AI Help"], button[title="Ask AI Help"]');
    await expect(fab).toBeVisible();
  });
});
