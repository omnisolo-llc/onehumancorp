import { test, expect } from './fixtures';

test.describe('Omni-Inbox Auto-Reply Agent', () => {
  test('displays the database-backed inbox experience', async ({ page }) => {
    await page.goto('/inbox');
    await expect(page.getByRole('heading', { name: 'Inbox' })).toBeVisible();

    await expect(page.getByText('Message Queue')).toBeVisible();
    await expect(page.getByText('Conversation Detail')).toBeVisible();
    await expect(page.getByText('Loaded from `/api/ui/inbox/messages`')).toBeVisible();
  });
});
