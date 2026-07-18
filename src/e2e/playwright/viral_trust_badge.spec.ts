import { test, expect } from '@playwright/test';

test.describe('Viral Trust Badge Builder', () => {
  // Using an authenticated test by mocking the login state to reach the dashboard
  // and asserting that the dashboard successfully links to the new widget builder.
  test('Dashboard GrowBusinessCard links to the Trust Badge Builder', async ({ page }) => {
    // Navigate to the Dashboard which should have the growth business card
    // E2E infrastructure handles the database seeding and generic login session locally
    await page.goto('/dashboard');

    // Check that the link to the viral-trust-badge-builder exists in the UI
    const trustBadgeLink = page.getByRole('link', { name: /Trust Badge Builder/i });
    await expect(trustBadgeLink).toBeVisible();
    await expect(trustBadgeLink).toHaveAttribute('href', '/viral-trust-badge-builder');

    // Note: Due to limitations testing isolated widgets inside the e2e sandbox
    // we limit our test to the widget link verification.
  });
});
