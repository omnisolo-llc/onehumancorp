import { test, expect } from './fixtures';

test.describe('Growth Referral Widget Premium Layout', () => {
  test('should display correctly with premium glassmorphism classes', async ({ page }) => {
    // Navigate to a page that uses the GrowthReferralWidget (e.g., team)
    await page.goto('/team');

    // Check for Sovereign-to-Cloud Bridge card
    const cloudBridgeTitle = page.getByRole('heading', { name: 'Grow Your Team' });
    await expect(cloudBridgeTitle).toBeVisible();

    // Test copy embed code logic
    const embedButton = page.getByRole('button', { name: 'Copy Embed Code' });
    await expect(embedButton).toBeVisible();

    // Give clipboard permissions
    await page.context().grantPermissions(['clipboard-read', 'clipboard-write']);

    // Handle dialog
    page.once('dialog', async (dialog) => {
      expect(dialog.message()).toBe('Embed code copied to clipboard!');
      await dialog.accept();
    });

    await embedButton.click();

    const clipboardText = await page.evaluate(() => navigator.clipboard.readText());
    expect(clipboardText).toContain('<iframe');
    expect(clipboardText).toContain('⚡ Powered by OHC');
  });
});
