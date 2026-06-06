import { test, expect } from './fixtures';

test.describe('Documentation & Help Features', () => {

  test('should display tooltips correctly', async ({ page }) => {
    await page.goto('/');

    const tooltipTarget = page.locator('#nav-store-link');
    if (await tooltipTarget.count() > 0) {
      await tooltipTarget.hover();
      await expect(page.locator('.animate-fade-in-up')).toBeVisible({ timeout: 5000 });
    }
  });

  test('should open help widget and view articles', async ({ page }) => {
    await page.goto('/');

    // Help widget button
    const helpBtn = page.getByRole('button', { name: 'Help', exact: true });
    await expect(helpBtn).toBeVisible();
    await helpBtn.click();

    // Verify widget opened by checking if "Help Center" title inside the widget is visible
    const helpCenterTitle = page.getByRole('heading', { name: 'Help Center', exact: true });
    await expect(helpCenterTitle).toBeVisible();

    // Check for article (since backend might be returning it slowly, add retries or let playwright handle it)
    await expect(page.getByText('Getting Started')).toBeVisible({ timeout: 10000 });
  });

  test('should search for an article', async ({ page }) => {
    await page.goto('/help');

    const searchInput = page.getByPlaceholder('Search for help articles and videos...');
    await searchInput.fill('payment');

    // Wait for the results to update
    await expect(page.getByText('Getting Paid')).toBeVisible({ timeout: 10000 });
  });

  test('should show video tutorials', async ({ page }) => {
    await page.goto('/help/videos');

    await expect(page.getByRole('heading', { name: 'Video Guides', exact: true })).toBeVisible();
    await expect(page.getByText('How to set up your first store easily')).toBeVisible({ timeout: 10000 });
  });

});
