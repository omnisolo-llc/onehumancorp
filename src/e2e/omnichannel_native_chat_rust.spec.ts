import { test, expect } from './fixtures';

test.describe('Native Rust Omnichannel Chat System', () => {
  test('creates conversation, routes message, and displays AI draft in Unified Inbox', async ({ page }) => {
    // 1. Simulate incoming message (via webhook mock)
    const tenantId = 'default';
    const customerMessage = 'Hello, can I order a custom vegan cake for this Saturday?';
    const payload = {
        tenant_id: tenantId,
        source: 'instagram',
        sender_id: 'maya_customer_01',
        message: customerMessage,
    };

    const webhookResponse = await page.request.post('/api/v1/inbox/webhook', {
        data: payload
    });

    // We expect 200 OK or similar
    expect(webhookResponse.ok()).toBeTruthy();

    // 2. Business owner navigates to unified inbox
    await page.goto('/inbox');
    await expect(page.getByRole('heading', { name: 'Inbox' })).toBeVisible();

    // 3. Verify conversation card is present in list
    const conversationCard = page.locator('.conversation-card', { hasText: 'maya_customer_01' }).first();
    await expect(conversationCard).toBeVisible();
    await expect(conversationCard).toContainText('instagram');

    // 4. Click conversation to view thread
    await conversationCard.click();

    // 5. Verify unified timeline displays the customer message
    await expect(page.getByText(customerMessage)).toBeVisible();

    // 6. Verify The Ambassador (AI Agent) has generated a draft
    const aiDraftArea = page.locator('.ai-draft-area');
    await expect(aiDraftArea).toBeVisible();
    await expect(aiDraftArea).toContainText('The Ambassador suggests');

    // The draft should contain "vegan cake" context
    await expect(aiDraftArea.getByText(/vegan cake/i)).toBeVisible();

    // 7. Verify action buttons are available
    await expect(aiDraftArea.getByRole('button', { name: 'Approve' })).toBeVisible();
    await expect(aiDraftArea.getByRole('button', { name: 'Edit' })).toBeVisible();
    await expect(aiDraftArea.getByRole('button', { name: 'Discard' })).toBeVisible();

    // 8. Test the 'Approve' flow
    await aiDraftArea.getByRole('button', { name: 'Approve' }).click();

    // 9. The draft should move to the timeline as a sent message
    // Note: implementation depends on how Sent items are rendered.
    await expect(aiDraftArea).toBeHidden();

    // Verify status transition
    // Conversation status should move from Open to BotHandled or Resolved (based on backend)
    // Testing specific UI indicators if present
  });
});
