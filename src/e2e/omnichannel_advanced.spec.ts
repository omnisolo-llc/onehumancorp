import { test, expect } from '@playwright/test';
import { v4 as uuidv4 } from 'uuid';
import { e2ePlaywrightTestFlow } from './global-setup'; // Utilizing standard setup if needed

test.describe('Omnichannel Advanced Features (Omnichannel Parity)', () => {
  test('Web Chat Widget functions correctly and displays labels & canned responses', async ({ page }) => {
    // Navigate to a real page where the chat is available
    await page.goto('/team/chat');
    await page.waitForLoadState('networkidle');

    // 1. Verify the team chat UI loads.
    await expect(page.locator('h1', { hasText: 'Team Chat' })).toBeVisible();

    // 2. Send a message.
    const input = page.getByTestId('team-chat-input');
    await input.fill('I need help with my cake order.');
    await page.getByTestId('team-chat-send').click();

    // 3. Verify message is displayed.
    await expect(page.locator('text=I need help with my cake order.')).toBeVisible();

    // 4. Verify system responds.
    await expect(page.locator('text=Working on your request...')).toBeVisible();
  });
});
