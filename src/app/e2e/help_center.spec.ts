import { test, expect } from '@playwright/test';

test.describe('Help Center CUJ', () => {
  test('User can navigate to help center and view categories', async ({ page }) => {
    // Navigate to the login page (starting point)
    await page.goto('/#/login');

    // Perform login
    await page.fill('input[type="email"]', 'admin@onehumancorp.com');
    await page.fill('input[type="password"]', 'admin');
    await page.click('button:has-text("Sign in")');

    // Wait for the dashboard to load (assuming it redirects here)
    await page.waitForURL('**/dashboard');

    // Click the global help FAB button
    const helpButton = page.getByRole('button', { name: 'Help Center' });
    await expect(helpButton).toBeVisible();
    await helpButton.click();

    // Wait for the Help Center screen to load
    await page.waitForURL('**/help');

    // Verify the Help Center page title
    await expect(page.locator('text="Help Center"').first()).toBeVisible();

    // Verify search input
    await expect(page.locator('input[placeholder="Search for help..."]')).toBeVisible();

    // Simulate a search
    await page.fill('input[placeholder="Search for help..."]', 'Apple Pay');

    // Verify search result appears
    await expect(page.locator('text="Search Results"')).toBeVisible();
    await expect(page.locator('text="Search result for \\"Apple Pay\\""')).toBeVisible();

    // Verify categories are present
    const categories = [
      'Getting Started',
      'My Store',
      'Payments',
      'AI Agents',
      'Marketing',
      'Account & Billing'
    ];

    for (const category of categories) {
      await expect(page.locator(`text="${category}"`)).toBeVisible();
    }
  });
});
