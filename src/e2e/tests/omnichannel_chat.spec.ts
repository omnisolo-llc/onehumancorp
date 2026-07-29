import { test, expect } from '@playwright/test';

test.describe('Native Omnichannel Chat (Chatw00t Replacement)', () => {
  test('Owner can view, receive drafts, and approve messages in the unified inbox feed', async ({ page }) => {
    // Note: Mocking auth or navigating to test route in real usage, assuming /inbox exists
    // This is a representative structure based on the PR prompt constraints

    await page.goto('/inbox');

    // Verify Mobile-First UX Layout
    await page.setViewportSize({ width: 375, height: 812 });

    // 1. Inbox Feed (owner sees unified vertical feed)
    // Wait for feed to load
    await expect(page.locator('.inbox-feed')).toBeVisible();

    // The test simulates an incoming external message via API or assumes seed data
    // Assuming seed data for a conversation exists:
    const conversationCard = page.locator('.conversation-card').first();
    await expect(conversationCard).toBeVisible();
    await expect(conversationCard.locator('.channel-icon')).toBeVisible();

    // 2. Conversation View
    await conversationCard.click();

    // Verify context half and message thread
    await expect(page.locator('.customer-context')).toBeVisible();
    await expect(page.locator('.message-thread')).toBeVisible();

    // Simulate AI Draft Ready
    const draftCard = page.locator('.ai-draft-card');
    await expect(draftCard).toContainText('AI Draft Ready');

    // 3. Agent Interaction - Approve & Send
    const approveButton = draftCard.locator('button:has-text("Approve & Send")');
    await expect(approveButton).toBeVisible();
    await approveButton.click();

    // Verify draft goes to 'sent' state
    await expect(draftCard).not.toBeVisible();
    await expect(page.locator('.message-thread .sent-message').last()).toBeVisible();
  });
});
