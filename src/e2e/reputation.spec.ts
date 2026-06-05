import { test, expect } from './fixtures';

test.describe('Reputation & Review Engine', () => {
  test('Maya can view reputation pulse and inbox', async ({ page }) => {
    await page.goto('/reputation');

    // Check Reputation Pulse
    await expect(page.getByRole('heading', { name: 'Reputation Pulse' })).toBeVisible();
    await expect(page.getByText('Overall Rating')).toBeVisible();

    // Check Automation Settings
    await expect(page.getByText('Auto-Request Reviews')).toBeVisible();
    await expect(page.getByText('The Publicist Drafts')).toBeVisible();

    // Check Review Inbox
    await expect(page.getByRole('heading', { name: 'Review Inbox' })).toBeVisible();

    // Wait for the reviews to load (might be empty or have data depending on seed)
    await page.waitForTimeout(2000);
  });
});
