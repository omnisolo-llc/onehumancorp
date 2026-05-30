import { test, expect } from '@playwright/test';
import { adminPage } from './fixtures';

test.describe('Viral Growth Loops - Milestones', () => {
  test('verify milestone alert is visible on dashboard and navigates to milestones page to share', async ({ page }) => {
    // 1. Navigate to dashboard
    await page.goto('/dashboard');

    // 2. Verify Success Milestone Alert is present
    const milestoneAlert = page.locator('section').filter({ hasText: "10th Order Milestone!" });
    await expect(milestoneAlert).toBeVisible({ timeout: 10000 });

    // 3. Click the 'Share to Celebrate' button
    const shareButton = milestoneAlert.getByRole('link', { name: 'Share to Celebrate' });
    await expect(shareButton).toBeVisible();
    await shareButton.click();

    // 4. Verify we navigated to the milestones page
    await expect(page).toHaveURL(/\/milestones/);
    await expect(page.getByRole('heading', { name: 'Success Milestones 🏆' })).toBeVisible();

    // 5. Select the unlocked 10th order milestone
    const tenthOrderMilestone = page.locator('div').filter({ hasText: "10th Order Milestone" }).nth(1);
    await tenthOrderMilestone.click();

    // 6. Verify the Share to X button exists
    const shareToXBtn = page.getByRole('button', { name: 'Share to X' });
    await expect(shareToXBtn).toBeVisible();

    // We cannot easily click it and verify the popup without mocking network or dealing with popup contexts,
    // but we can ensure it's visible and clickable
  });
});
