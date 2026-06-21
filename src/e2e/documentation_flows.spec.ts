import { test, expect } from './fixtures';

test.describe('Documentation Flows', () => {

  test.beforeEach(async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
  });

  test('Help Widget interactions and Videos', async ({ page }) => {
    // Wait for the help page to load
    await page.goto('/api/ui/help.html');

    // Make sure the title renders
    await expect(page.locator('h1:has-text("In-App Help Center")')).toBeVisible();

    await expect(page.locator('input[placeholder="Search for help articles and videos..."]')).toBeAttached();
    // Verify articles are rendered from the backend
    await expect(page.getByText('Connecting a bank account to accept payments').first()).toBeVisible({ timeout: 10000 });
  });
});
