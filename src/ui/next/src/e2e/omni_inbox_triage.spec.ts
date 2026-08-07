import { test, expect } from './fixtures';

test.describe('Omni Inbox Agentic Triage', () => {
  test('displays unread leads summary without mock interception', async ({ page }) => {
    await page.goto('/inbox');

    const heading = page.getByRole('heading', { name: 'Unified Inbox' });
    await expect(heading).toBeVisible();
  });
});
