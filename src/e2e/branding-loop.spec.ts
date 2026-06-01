import { test, expect } from './fixtures';

test.describe('Growth Loop: Footer Branding Toggle', () => {
  test('User can toggle Powered by OHC branding in storefront builder', async ({ page, aiJudge }) => {
    // Navigate to the storefront builder
    await page.goto('/storefront-builder');

    // Make sure we are on the builder page by checking for the branding toggle
    const brandingToggle = page.locator('input#branding-toggle');
    await expect(brandingToggle).toBeVisible();

    // Verify it is checked by default
    await expect(brandingToggle).toBeChecked();

    // Verify the "Powered by OHC" pill is visible in the preview
    const brandingPill = page.locator('a[href*="ohc://join?ref="]').filter({ hasText: 'Powered by OHC' });
    await expect(brandingPill).toBeVisible();

    // Verify text explaining the growth incentive exists
    const incentiveText = page.getByText('+100 bonus AI actions');
    await expect(incentiveText).toBeVisible();

    // Toggle off the branding
    await brandingToggle.uncheck();
    await expect(brandingToggle).not.toBeChecked();

    // Wait for network response if any (mocking not strictly necessary if endpoint exists, but good practice to wait for it)
    // Verify the branding pill is no longer visible
    await expect(brandingPill).not.toBeVisible();

    // Reload the page and ensure the state persists (off)
    await page.reload();
    await expect(brandingToggle).not.toBeChecked();
    await expect(brandingPill).not.toBeVisible();

    // Toggle back on
    await brandingToggle.check();
    await expect(brandingToggle).toBeChecked();
    await expect(brandingPill).toBeVisible();

    // Reload the page and ensure the state persists (on)
    await page.reload();
    await expect(brandingToggle).toBeChecked();
    await expect(brandingPill).toBeVisible();
  });
});
