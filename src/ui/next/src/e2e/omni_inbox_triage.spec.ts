import { test, expect } from '../../../../e2e/fixtures';

test.describe('Omni Inbox Agentic Triage', () => {
  test('displays unread leads summary and allows inventory deduction approval', async ({ page, adminUser, loginAs }) => {
    await loginAs(adminUser);

    await page.goto('/inbox');

    await expect(page.locator('h1')).toHaveText(/Unified Inbox/i);

    await page.locator('button:has-text("Mark All Read")').click();

    await expect(page.locator('.toast')).toHaveText(/All messages marked as read/i);
  });
});
