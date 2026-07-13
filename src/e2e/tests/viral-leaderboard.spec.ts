import { test, expect } from '@playwright/test';

test.describe('Viral Leaderboard Widget Generator', () => {
  test('should allow users to generate and embed a viral leaderboard widget', async ({ page }) => {
    // Navigate to the generator page
    await page.goto('/viral-leaderboard-generator');

    // Wait for the page to load
    await expect(page.locator('h1')).toContainText('Viral Leaderboard Generator 🏆');

    // Update settings
    await page.fill('input[type="text"]', 'My Top Affiliates');
    await page.selectOption('select', 'referrers');

    // Toggle branding removal (should trigger paywall)
    const brandingCheckbox = page.locator('label', { hasText: 'Remove "Powered by OHC" Badge' });
    await brandingCheckbox.click();

    // Verify paywall modal appears
    await expect(page.locator('h2', { hasText: 'Upgrade to Remove Branding' })).toBeVisible();

    // Close paywall modal
    await page.click('button:has-text("Cancel")');

    // Ensure iframe preview is loaded and visible
    const previewIframe = page.locator('iframe').first();
    await expect(previewIframe).toBeVisible();

    // Copy embed code
    const copyButton = page.locator('button', { hasText: 'Copy Embed Code' });
    await copyButton.click();
    await expect(copyButton).toContainText('Copied to Clipboard!');
  });
});
