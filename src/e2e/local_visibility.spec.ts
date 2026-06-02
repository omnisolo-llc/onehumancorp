import { test, expect } from '@playwright/test';

test.describe('Local Visibility CUJ', () => {
  test('connects Google Business Profile and approves an AI review reply', async ({ page }) => {
    // Navigate to the dashboard
    await page.goto('http://localhost:3000/dashboard');

    // Wait for the local visibility section to be visible
    const localVisibilitySection = page.locator('#local-visibility-section');
    await expect(localVisibilitySection).toBeVisible();

    // Verify initial state: not connected
    await expect(page.locator('text=Connect your Google Business Profile')).toBeVisible();

    // Connect Google Business Profile
    await page.click('#connect-google-business-btn');

    // Verify connected state
    await expect(page.locator('text=🟢 Synced with Google Maps')).toBeVisible();
    await expect(page.locator('text=Your hours, menu, and services are automatically syncing')).toBeVisible();

    // Verify Review Approval Feed is visible with a pending review
    await expect(page.locator('text=Review Approval Feed')).toBeVisible();
    await expect(page.locator('text=Sarah Jenkins')).toBeVisible();
    await expect(page.locator('text="Absolutely incredible service! They arrived on time and fixed the plumbing issue in no time."')).toBeVisible();
    await expect(page.locator('text=AI Draft Reply')).toBeVisible();

    // Approve & Reply
    await page.click('#approve-reply-r1');

    // Verify review disappears after approval
    await expect(page.locator('text=Sarah Jenkins')).not.toBeVisible();
  });
});
