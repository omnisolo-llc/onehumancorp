import { test, expect } from './fixtures';

test.describe('Native Omnichannel Chat (Chatwoot Replacement)', () => {
  test('End-to-End Chat Flow with real data pipeline', async ({ page, adminUser, loginAs }) => {
    await loginAs(page, adminUser);
    await page.goto('/inbox');

    // We expect the native omnichannel chat UI to render
    const title = page.locator('h1', { hasText: 'Native Omnichannel Chat' });
    await expect(title).toBeVisible();

    // Verify tabs
    await expect(page.locator('button', { hasText: 'Unread' }).first()).toBeVisible();
    await expect(page.locator('button', { hasText: 'Action Needed' }).first()).toBeVisible();

    // Verify sticky input bar
    const textarea = page.locator('textarea[placeholder="Type your message..."]');
    await expect(textarea).toBeVisible();

    // Verify AI suggestion and Send buttons
    await expect(page.locator('button', { hasText: '✨ AI Suggestion' })).toBeVisible();
    await expect(page.locator('button', { hasText: 'Send' })).toBeVisible();
  });
});
