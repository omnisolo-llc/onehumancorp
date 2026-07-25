import { test, expect } from '@playwright/test';

test.describe('Omnichannel Chat', () => {
  test('Owner can see customer message, AI drafts reply, owner approves', async ({ page }) => {
    // Navigate to the chat page directly for the test
    await page.goto('http://localhost:3000/team/chat');

    // Check initial state
    await expect(page.locator('text=Customer: Do you do vegan cakes?')).toBeVisible();

    // Simulate Maya (owner) sending a quick note
    const input = page.locator('[data-testid="team-chat-input"]');
    await input.fill('Yes');
    await page.locator('[data-testid="team-chat-send"]').click();

    // Wait for the AI Draft to appear
    await expect(page.locator('text=I\'ve drafted a reply for your approval.')).toBeVisible({ timeout: 5000 });
    await expect(page.locator('text=Drafted reply: Yes, we can certainly help with that.')).toBeVisible();

    // Owner approves the draft
    const approveBtn = page.locator('[data-testid="approve-action-btn"]');
    await approveBtn.click();

    // Verify status changes to "Sent"
    await expect(page.locator('text=Sent')).toBeVisible();
  });
});
