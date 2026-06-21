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

  test('Tooltips load and display properly', async ({ page }) => {
    // Go to the dashboard
    await page.goto('/api/ui/dashboard.html');

    // Make sure the help button exists
    const walkBtn = page.locator('#dashboard-walkthrough-btn');
    await expect(walkBtn).toBeVisible();

    // Hover over the help button to trigger the tooltip
    await walkBtn.dispatchEvent('touchstart');
    await page.waitForTimeout(600); // 500ms for long press

    // Verify the tooltip loads with expected content from our backend
    const tooltipText = page.getByText(/Start an interactive guide to learn how to use OHC./i).last();
    await expect(tooltipText).toBeVisible({ timeout: 10000 });
  });
});
