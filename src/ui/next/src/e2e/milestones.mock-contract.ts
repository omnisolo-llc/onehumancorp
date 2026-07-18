import { test, expect } from '../../../../e2e/fixtures';

test.describe('Milestones Page UI', () => {
  // We navigate naturally through the frontend to reach the page.
  test.beforeEach(async ({ page }) => {
    // Navigate to root to start the flow
    await page.goto('/dashboard');

    // Provide the expected logged in storage data to emulate the frontend state
    await page.evaluate(() => {
      window.localStorage.setItem('has_onboarded', 'true');
      window.localStorage.setItem('tenant', 'test-tenant');
    });

    // Simulate login and go to the dashboard
    await page.goto('/dashboard');

    // Find our new link
    const link = page.locator('a', { hasText: 'Milestones 🏆' });
    await expect(link).toBeVisible();
    await link.click();

    // Confirm we arrive
    await expect(page).toHaveURL(/.*\/milestones/);
  });

  test('should display milestones after natural login flow', async ({ page }) => {
    await expect(page.locator('text=Your Achievements')).toBeVisible();

    // Verify Glassmorphism styles are applied
    const firstMilestone = page.locator('.glassmorphism').first();
    await expect(firstMilestone).toBeVisible();
  });

  test('should display locked milestones correctly', async ({ page }) => {
    const lockedMilestone = page.locator('text=$1,000 Revenue').locator('..');
    await expect(lockedMilestone).toContainText(/LOCKED/i);

    const milestoneContainer = lockedMilestone.locator('..').locator('..');
    await expect(milestoneContainer).toHaveClass(/glassmorphism/);
  });
});
