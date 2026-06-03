import { test, expect } from './fixtures';

test.describe('Omni-Inbox Auto-Reply Agent', () => {
  test('simulates incoming message and auto-replies correctly', async ({ page }) => {
    await page.goto('/inbox');

    // Click Simulate Incoming Message
    await page.getByRole('button', { name: '🤖 Simulate Incoming Message' }).click();

    // Verify user message is added
    await expect(page.getByText('Are you open today?')).toBeVisible();

    // Wait for AI Reply
    const aiBadge = page.getByText('AI Replied');
    await expect(aiBadge).toBeVisible({ timeout: 10000 });

    // Verify reply content
    await expect(page.getByText('Hi! Yes, we are open until 6 PM today and we currently have 12 Vanilla Cupcakes left. Shall I set one aside for you?')).toBeVisible();
  });

  test('handles specific intents and toggles AI auto-reply', async ({ page }) => {
    await page.goto('/inbox');

    // Test "vegan" intent
    await page.click('#simulate-vegan');
    await expect(page.getByText('do you have anything vegan?')).toBeVisible();
    await expect(page.getByText('Yes! We offer a variety of vegan options')).toBeVisible({ timeout: 10000 });

    // Test "where is my order" escalation
    await page.click('#simulate-order');
    await expect(page.getByText('where is my order?')).toBeVisible();
    await expect(page.getByText('Escalated to Human')).toBeVisible({ timeout: 10000 });
    await expect(page.getByText('This message could not be handled automatically')).toBeVisible();

    // Open settings and toggle off AI Auto-Reply
    await page.getByRole('button', { name: '⚙️' }).click();
    await expect(page.getByText('AI Auto-Reply')).toBeVisible();

    // Click the toggle to disable it
    await page.locator('div').filter({ hasText: /^AI Auto-Reply$/ }).getByRole('button').click();
    await page.getByRole('button').filter({ hasText: /^$/ }).first().click(); // close modal

    // Click simulate again (default message)
    await page.getByRole('button', { name: '🤖 Simulate Incoming Message' }).click();

    // Auto-reply is off, so there shouldn't be a new AI Replied badge
    await page.waitForTimeout(2000);

    // Should still only be 4 total (3 original + 1 from vegan test)
    const aiBadges = await page.getByText('AI Replied').count();
    expect(aiBadges).toBe(4);
  });
});
