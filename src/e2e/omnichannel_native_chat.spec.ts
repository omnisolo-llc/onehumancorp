import { test, expect } from '@playwright/test';
import { setupMockUser } from './db_utils';

test.describe('Omnichannel Native Chat System', () => {
  let tenantId: string;

  test.beforeEach(async ({ page, request }) => {
    tenantId = `t-omni-${Date.now()}`;
    await setupMockUser(request, tenantId, 'omni_owner@example.com', 'owner');
  });

  test('user logs in, receives a message, and successfully sends a reply back', async ({ page }) => {
    // Navigate to the Dashboard
    await page.goto('/ui/dashboard.html');
    await page.evaluate(`localStorage.setItem('tenant_id', '${tenantId}')`);
    await page.reload();

    // Click on the Inbox link
    await page.locator('a[href="omnichannel-native.html"]').click();

    // Ensure we are on the native inbox view
    await expect(page).toHaveURL(/omnichannel-native\.html/);

    // Initial state: We should see loading or empty state.
    // However, our code mocks some data for test environments if DB is not fully seeded in E2E.
    // Let's assume the mock triggers. We should see "Customer"
    await expect(page.locator('h1').first()).toHaveText('Unified Inbox');

    // Click on the first conversation
    const conversationItem = page.locator('[data-testid="conversation-item"]').first();
    await expect(conversationItem).toBeVisible();
    await conversationItem.click();

    // We should now be in the conversation view
    await expect(page.locator('#chat-header-name')).toBeVisible();

    // AI draft should appear after a short delay (mocked in our JS)
    const approveBtn = page.locator('[data-testid="approve-ai-draft-btn"]');
    await expect(approveBtn).toBeVisible({ timeout: 5000 });

    // Click approve to send AI draft
    await approveBtn.click();

    // The message should appear as sent
    const messageBubbles = page.locator('.message-sent');
    await expect(messageBubbles.last()).toBeVisible();

    // Now test sending a manual reply
    const chatInput = page.locator('#chat-input');
    await chatInput.fill('This is a manual reply');

    const sendBtn = page.locator('[data-testid="send-msg-btn"]');
    await sendBtn.click();

    // Verify it was appended
    await expect(messageBubbles.last()).toHaveText('This is a manual reply');

    // Back to inbox
    await page.locator('button', { hasText: 'Back' }).click();
    await expect(page.locator('h1').first()).toHaveText('Unified Inbox');
  });
});
