import { test, expect } from '@playwright/test';

test.describe('Changelog', () => {
  test('should load the changelog page and display recent updates', async ({ page }) => {
    // Navigate to the Changelog page
    await page.goto('/changelog');

    // Verify the page title
    await expect(page.getByRole('heading', { name: 'Release Notes & Changelog' })).toBeVisible();

    // Verify at least one version section is present
    await expect(page.getByText('Version 1.0 (Latest)')).toBeVisible();

    // Verify it links to the full technical changelog
    await expect(page.getByRole('link', { name: 'Read the full technical changelog on our website →' })).toBeVisible();
    await expect(page.getByRole('link', { name: 'Read the full technical changelog on our website →' })).toHaveAttribute('href', 'https://onehumancorp.com/changelog');
  });

  test('should allow navigation back from changelog to dashboard', async ({ page }) => {
    // E2E Test 5: Back navigation from Changelog
    await page.goto('/dashboard');
    await page.goto('/changelog');

    // Test back button
    const backButton = page.locator('button', { hasText: 'Back' });
    await backButton.click();

    await page.waitForURL('**/dashboard');

    await expect(page.locator('h1', { hasText: 'Dashboard' })).toBeVisible();
  });
});
