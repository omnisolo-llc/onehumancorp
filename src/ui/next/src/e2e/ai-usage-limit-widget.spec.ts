import { test, expect } from '../../../../e2e/fixtures';

test.describe('AI Usage Limit Widget', () => {
  test('displays usage data, upgrade CTA, and viral Share button', async ({ page }) => {
    await page.goto('/dashboard');

    // Check if the widget is visible
    const widget = page.locator('[data-testid="ai-usage-limit-widget"]');
    await expect(widget).toBeVisible();

    // Check texts
    await expect(widget).toContainText('Approaching Free Tier Limit');
    await expect(widget).toContainText('85');
    await expect(widget).toContainText('/ 100');

    // Check Upgrade to Pro link
    const upgradeLink = widget.locator('a', { hasText: 'Upgrade to Pro (Unlimited)' });
    await expect(upgradeLink).toBeVisible();
    await expect(upgradeLink).toHaveAttribute('href', '/pricing');

    // Check share to get bonus actions button
    const shareBtn = widget.locator('button', { hasText: 'Share on X to get +50 Actions' });
    await expect(shareBtn).toBeVisible();

    // Click it to generate link
    await shareBtn.click();
    await expect(widget.locator('button', { hasText: 'Generating...' })).toBeVisible();

    // Wait for generation
    await expect(widget.locator('button', { hasText: 'Copy Link' })).toBeVisible({ timeout: 2000 });

    // Click copy link and check optimistic update
    await widget.locator('button', { hasText: 'Copy Link' }).click();

    // It should now show "Copied Link!"
    await expect(widget.locator('button', { hasText: 'Copied Link!' })).toBeVisible();

    // After 1.5s, the limit should be updated (85 -> 35)
    await page.waitForTimeout(1600);
    await expect(widget).toContainText('35');

    // Check if progress bar still exists
    await expect(widget.locator('.bg-green-500')).toBeVisible(); // 35 is < 80% so it should be green
  });
});
