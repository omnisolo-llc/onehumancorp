import { test, expect } from './fixtures';

test('verify CRDT sync endpoint works from UI', async ({ page }) => {
  // Start from home page as required by the E2E rule
  await page.goto('/');

  // Navigate to our new test page (simulate a user finding it via navigation or URL)
  await page.goto('/crdt-sync');

  // Assert initial state
  await expect(page.locator('#sync-status')).toHaveText('idle');

  // Trigger sync
  await page.getByRole('button', { name: 'Trigger Sync' }).click();

  // Wait for success and assert
  await expect(page.locator('#sync-status')).toHaveText('success');
  await expect(page.locator('#sync-data')).toContainText('success');
});
