import { test, expect } from '@playwright/test';
import { e2eDataSeed } from './fixtures';

test.describe('Brand Voice Tuning Engine', () => {
  test('small business owner completes AB test tuning', async ({ page }) => {
    // Navigate to the new tuning flow
    await page.goto('/brand-voice');

    // Verify UI renders correctly
    await expect(page.getByText("Let's teach your AI how you sound.")).toBeVisible();
    await expect(page.getByText("Pick the response that sounds most like you.")).toBeVisible();

    // Select the bubbly option
    await page.locator('text="Hi! 🍰 We ship within 2 days! Let me know if you need it sooner! ✨"').click();

    // Verify it redirects back to dashboard
    await expect(page).toHaveURL(/.*\/dashboard/);
  });
});
