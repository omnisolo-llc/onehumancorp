import { test, expect } from '@playwright/test';

test.describe('AI Agent Department Architecture', () => {
  test('should display approval inbox and activity feed', async ({ page }) => {
    // Navigate to agents page
    await page.goto('/agents');

    // Ensure "My Team" tab is visible
    await expect(page.locator('text=My Team')).toBeVisible();
    await expect(page.locator('text=The Manager')).toBeVisible();

    // Navigate to "Activity Feed" tab
    await page.locator('text=Activity Feed').click();
    await expect(page.locator('text=Fetching feed...').or(page.locator('text=No activity yet.'))).toBeVisible();

    // Navigate to "Needs Approval" tab
    await page.locator('text=Needs Approval').click();
    await expect(page.locator('text=Fetching approvals...').or(page.locator('text=All Caught Up!'))).toBeVisible();
  });

  test('should show soft paywall for Pro Mode when not pro, and allow trial extension via share', async ({ page }) => {
    // Navigate to the agents page
    await page.goto('/agents');

    // Make sure we don't have pro
    await page.evaluate(() => {
        localStorage.removeItem('has_pro');
    });
    await page.reload();

    // Click on the Pro Mode toggle button
    const proModeToggle = page.locator('span', { hasText: 'Pro Mode' }).locator('..').locator('button');
    await proModeToggle.click({ force: true });

    // The soft paywall modal should appear
    await expect(page.getByRole('heading', { name: 'Upgrade to Pro' })).toBeVisible();
    await expect(page.getByRole('link', { name: /Upgrade to Pro/ })).toBeVisible();
    const shareButton = page.getByRole('button', { name: /Share on X to get 7 Days Free/i });
    await expect(shareButton).toBeVisible();

    // Intercept window.open
    await page.evaluate(() => {
        window.open = () => null;
    });

    await shareButton.click();

    // The paywall should disappear
    await expect(page.getByRole('heading', { name: 'Upgrade to Pro' })).not.toBeVisible();
  });
});
