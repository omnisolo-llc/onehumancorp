import { test, expect } from '@playwright/test';

test.describe('E2E Onboarding Journey', () => {
  test('Mobile-first 3-screen flow', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 667 });
    await page.goto('/');

    // Click Start
    await page.click('text=🚀 Start My Business');

    // Screen 1: Input idea
    await expect(page.locator('text=What are you building today?')).toBeVisible();
    await page.fill('input[type="text"]', "A custom cake shop");
    await page.click('text=Next');

    // Screen 2: Shimmer/Generation
    await expect(page.locator('text=Our AI (The Promoter) is designing your site...')).toBeVisible();

    // Screen 3: Review
    await expect(page.locator('text=Looks Good, Go Live')).toBeVisible({ timeout: 10000 });
    await page.click('text=Looks Good, Go Live');

    // Screen 4: Success
    await expect(page.locator('text=Share on Instagram')).toBeVisible({ timeout: 10000 });
  });
});
