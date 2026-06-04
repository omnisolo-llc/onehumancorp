import { test, expect } from '@playwright/test';

test.describe('Dashboard Milestone Celebration', () => {
  test('should display celebration nudge when a milestone was recently reached', async ({ page }) => {
    // Navigate to root to start the flow
    await page.goto('/');

    // Provide the expected logged in storage data to emulate the frontend state
    // 'e2e-tenant' has a milestone seeded in e2e-seed.sql
    await page.evaluate(() => {
      window.localStorage.setItem('has_onboarded', 'true');
      window.localStorage.setItem('tenant_id', 'e2e-tenant');
      window.localStorage.setItem('tenant', 'e2e-tenant');
    });

    // Navigate to dashboard
    await page.goto('/dashboard');

    // Assert that the celebration nudge is visible
    const celebrationNudge = page.locator('text=Congratulations!');
    await expect(celebrationNudge).toBeVisible();
    await expect(page.locator('text=🎉 Milestone: First Sale!')).toBeVisible();

    // Verify "Share Success" button
    const shareButton = page.locator('button', { hasText: 'Share Success' });
    await expect(shareButton).toBeVisible();

    // Dismiss the nudge
    await page.locator('button[aria-label="Dismiss"]').click();
    await expect(celebrationNudge).not.toBeVisible();
  });
});
