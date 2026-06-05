import { test, expect } from './fixtures';

test.describe('Omni-Inbox Auto-Reply Agent', () => {
<<<<<<< HEAD
  test('displays the database-backed inbox experience', async ({ page }) => {
    await page.goto('/inbox');
    await expect(page.getByRole('heading', { name: 'Inbox' })).toBeVisible();

    await expect(page.getByText('Message Queue')).toBeVisible();
    await expect(page.getByText('Conversation Detail')).toBeVisible();
    await expect(page.getByText('Loaded from `/api/ui/inbox/messages`')).toBeVisible();
=======
  test('simulates incoming message and auto-replies correctly', async ({ page }) => {
    test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
    await page.goto('/inbox');
    await page.waitForLoadState('networkidle');

    // Click Simulate Incoming Message
    await page.getByRole('button', { name: '🤖 Simulate Incoming Message' }).click();

    // Verify user message is added
    await expect(page.getByText('Are you open today?')).toBeVisible();

    // Wait for AI Reply
    const aiBadge = page.getByText('AI Replied');
    await expect(aiBadge).toBeVisible({ timeout: 10000 });

    // Verify reply content
    await expect(page.getByText('Hi! Yes, we are open until 6 PM today and we currently have 12 Vanilla Cupcakes left. Shall I set one aside for you?')).toBeVisible();
>>>>>>> 95ce9988 (Autonomous Client Intake Questionnaire Engine Research Report (#23948))
  });
});
