import { test, expect } from './fixtures';

test.describe('Omni-Inbox Auto-Reply Agent', () => {
  test('displays the database-backed inbox experience', async ({ page }) => {
    await page.goto('/inbox');
    await expect(page.getByRole('heading', { name: 'Inbox' })).toBeVisible();

<<<<<<< HEAD
    await expect(page.getByText('Message Queue')).toBeVisible();
    await expect(page.getByText('Conversation Detail')).toBeVisible();
    await expect(page.getByText('Loaded from `/api/ui/inbox/messages`')).toBeVisible();
=======
    // Click Simulate Incoming Message
    // Click Simulate Incoming Message (Button removed or moved in app refactor)

    // await page.getByRole('button', { name: '🤖 Simulate Incoming Message' }).click();

    // Verify user message is added
    await expect(page.getByText('Are you open today?')).toBeVisible();

    // Wait for AI Reply
    const aiBadge = page.getByText('AI Replied');
    await expect(aiBadge).toBeVisible({ timeout: 10000 });

    // Verify reply content
    await expect(page.getByText('Hi! Yes, we are open until 6 PM today and we currently have 12 Vanilla Cupcakes left. Shall I set one aside for you?')).toBeVisible();
>>>>>>> 387b419a (test: fix broken E2E tests)
  });
});
