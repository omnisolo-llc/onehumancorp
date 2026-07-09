import { test, expect } from '@playwright/test';

test.describe('Help Videos Page', () => {
  test('displays video tutorials correctly and includes durations', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812 });

    await page.goto('/help/videos');

    await expect(page.locator('h1', { hasText: 'Video Guides' })).toBeVisible();

    await expect(page.locator('.absolute.bottom-3.right-3').first()).toBeVisible({ timeout: 10000 });

    const durationBadge = page.locator('.absolute.bottom-3.right-3').first();
    await expect(durationBadge).toBeVisible();
    await expect(durationBadge).not.toBeEmpty();

    const text = await durationBadge.textContent();
    expect(text?.trim()).toMatch(/^\d+:\d{2}$/);
  });
});
