import { test, expect } from '@playwright/test';

test.describe('Agent Marketplace Viral Share Loop', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to the marketplace
    await page.goto('/agent-marketplace');
    await expect(page.locator('h1')).toHaveText('Agent Marketplace');
  });

  test('User can copy a viral share link for an agent', async ({ page, context }) => {
    // Grant clipboard permissions
    await context.grantPermissions(['clipboard-read', 'clipboard-write']);

    // Find the first agent card's share button
    const shareButton = page.locator('button:has-text("Share Agent")').first();
    await expect(shareButton).toBeVisible();

    // Click the share button
    await shareButton.click();

    // Verify it shows copied state
    await expect(shareButton).toHaveText('Copied Link!');

    // Read clipboard and verify contents contain the referral loop format
    const clipboardText = await page.evaluate("navigator.clipboard.readText()");
    expect(clipboardText).toContain('/api/v1/growth/referrals/click');
    expect(clipboardText).toContain('target=/agent-marketplace');
  });
});
