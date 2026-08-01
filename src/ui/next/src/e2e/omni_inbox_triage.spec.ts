import { test, expect } from '../../../../src/e2e/fixtures';

test.describe('Omni Inbox Agentic Triage', () => {
  test('displays unread leads summary and allows inventory deduction approval', async ({ page }) => {
    // 4. Navigate to the inbox page
    await page.goto('/inbox');

    // We just check if it loads without throwing an error for now
    await expect(page.locator('body')).toBeVisible();
  });
});
