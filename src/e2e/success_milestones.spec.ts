import { test, expect } from './fixtures';

test.describe('Success Milestones Growth Loop', () => {
  test('verify milestones page and share capabilities', async ({ page }) => {
    // 1. Navigate to the milestones page
    await page.goto('/milestones');
    await page.waitForLoadState('networkidle');

    // 2. Verify the page heading
    await expect(page.getByRole('heading', { name: 'Success Milestones 🏆' })).toBeVisible();

    // 3. Verify the first milestone is present and unlocked (clickable)
    const firstOrderMilestone = page.locator('h3', { hasText: 'First Order! 🎉' }).first();
    await expect(firstOrderMilestone).toBeVisible();
    await firstOrderMilestone.click();

    // 4. Verify the "Share Your Success" section appears after clicking
    await expect(page.getByRole('heading', { name: 'Share Your Success' })).toBeVisible();

    // 5. Verify the "Copy Share Message" button is present
    await expect(page.getByRole('button', { name: 'Copy Share Message' })).toBeVisible();

    // 6. Verify the "Share on X" button/link is present
    await expect(page.locator('a', { hasText: 'Share on X' })).toBeVisible();
  });
});
