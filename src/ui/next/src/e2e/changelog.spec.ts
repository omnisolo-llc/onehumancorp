import { test, expect } from '@playwright/test';

test.describe('Changelog', () => {
  test('should load the changelog page and display recent updates', async ({ page }) => {
    // Navigate to the Changelog page
    await page.goto('/changelog');

    // Verify the page title
    await expect(page.getByRole('heading', { name: 'Release Notes & Changelog' })).toBeVisible();

    // Verify at least one version section is present
    await expect(page.getByText('v0.4.48 (Cloud) / v0.4.48+1 (Standalone)')).toBeVisible();

    // Verify it links to the full technical changelog
    await expect(page.getByRole('link', { name: 'Read the full technical changelog on our website →' })).toBeVisible();
    await expect(page.getByRole('link', { name: 'Read the full technical changelog on our website →' })).toHaveAttribute('href', 'https://onehumancorp.com/changelog');
  });
});
