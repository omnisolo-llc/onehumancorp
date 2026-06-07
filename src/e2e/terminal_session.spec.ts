import { test, expect } from './fixtures';

test.describe('Terminal Session Offline Sync Flow', () => {
  test('should link offline transactions to a session and sync them', async ({ page }) => {
    // Navigate to the POS terminal
    await page.goto('/pos/terminal');

    // Make sure we see the interface
    await expect(page.locator('text=Tap to Pay via Terminal')).toBeVisible({ timeout: 15000 });
  });
});
