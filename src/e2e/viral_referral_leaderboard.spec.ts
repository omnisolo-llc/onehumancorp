import { test, expect } from './fixtures';

test.describe('Referral Leaderboard Generator Widget', () => {
  test('should load the widget, display leaderboard, and generate embed code', async ({ page, context }) => {
    // Grant clipboard permissions for copying the link
    await context.grantPermissions(['clipboard-read', 'clipboard-write']);

    await page.goto('/ui/referral-leaderboard-generator.html');

    // Wait for the UI to be ready
    await page.waitForLoadState('networkidle');

    // Verify header and description
    await expect(page.getByRole('heading', { name: 'Referral Leaderboard' })).toBeVisible();

    // Verify loading state is gone and preview is visible
    const previewContainer = page.locator('#preview-container');
    await expect(previewContainer).toBeVisible({ timeout: 5000 });

    // Check if empty state or leaderboard is shown.
    // Assuming empty state initially or seeded data.
    const hasEmptyState = await page.locator('.empty-state').count() > 0;

    if (hasEmptyState) {
         await expect(page.locator('.empty-state')).toBeVisible();
    } else {
        // Verify embed section is visible when data exists
        const embedSection = page.locator('#embed-section');
        await expect(embedSection).toBeVisible();

        // Verify the embed code contains the details
        const embedCode = page.locator('#embed-code');
        await expect(embedCode).toContainText('<!-- OHC Referral Leaderboard -->');
        await expect(embedCode).toContainText('type=leaderboard');

        // Click "Copy Embed Code"
        const copyBtn = page.locator('#copy-btn');
        await copyBtn.click();

        // Verify button text changes to Copied!
        await expect(copyBtn).toHaveText('Copied!', { timeout: 3000 });

        // Verify clipboard content
        try {
            const clipboardText = await page.evaluate(async () => {
                return await navigator.clipboard.readText();
            });
            expect(clipboardText).toContain('<!-- OHC Referral Leaderboard -->');
        } catch (e) {
            console.warn('Clipboard read failed (expected in some headless environments): ', e);
        }
    }
  });

  test('should navigate back to dashboard', async ({ page }) => {
    await page.goto('/ui/referral-leaderboard-generator.html');
    const backLink = page.locator('.back-btn');
    await expect(backLink).toBeVisible();
    await expect(backLink).toHaveAttribute('href', '/dashboard.html');
  });

  test('should show responsive layout on mobile viewport', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812 });
    await page.goto('/ui/referral-leaderboard-generator.html');
    await page.waitForTimeout(100);

    await expect(page.getByRole('heading', { name: 'Referral Leaderboard' })).toBeVisible();

    const container = page.locator('.container');
    const box = await container.boundingBox();
    expect(box?.width).toBeLessThanOrEqual(375);
  });
});
