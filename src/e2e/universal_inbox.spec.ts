import { test as base, expect } from './fixtures';

const test = base.extend({
  page: async ({ page }, use) => {
    await page.setViewportSize({ width: 375, height: 812 });
    await use(page);
  }
});

test.describe('Universal Agentic Inbox CUJ', () => {
  test('Maya receives an inquiry via Universal Webhook, processes draft and approves it in Inbox', async ({ adminUser, loginAs, page, request }) => {
    await loginAs(page, adminUser);

    // 1. Simulate Maya receiving an inbound message via webhook
    const tenantId = adminUser.tenantId;
    const webhookResponse = await request.post('/api/v1/webhooks/universal', {
      data: {
        tenant_id: tenantId,
        channel: 'instagram_dm',
        message: 'Do you have availability for a custom 2-tier vegan cake next Saturday?',
        sender_id: 'maya_customer_123',
        target_language: 'English'
      }
    });
    expect(webhookResponse.status()).toBe(200);

    // Wait briefly for the orchestrator and agents to complete their asynchronous tasks
    await page.waitForTimeout(2500);

    // 2. Maya navigates to her Universal Inbox
    await page.goto('/inbox');
    await expect(page.getByRole('heading', { name: 'Inbox' })).toBeVisible();

    // 3. Maya verifies the new thread and message is listed
    await expect(page.getByText('instagram_dm').first()).toBeVisible({ timeout: 15000 });
    await expect(page.getByText('Do you have availability').first()).toBeVisible();

    // Click the item to view details
    await page.getByText('Do you have availability').first().click();

    // 4. Maya sees the AI-drafted reply and approves it
    // Note: Our TranslationAgent mock in the tests usually generates a canned response or passes it through
    await expect(page.getByText('Customer Message')).toBeVisible();
    await expect(page.getByText('Draft Reply')).toBeVisible();

    const approveBtn = page.getByRole('button', { name: '✨ Approve & Send Draft' });
    await expect(approveBtn).toBeVisible();

    await approveBtn.click();

    // 5. Verify optimistic update or state change confirming it was sent
    await expect(page.getByText('Draft approved and sent.')).toBeVisible({ timeout: 10000 });
    // Verify badge turns to sent/good tone
    const sentBadge = page.locator('span.app-badge.good', { hasText: 'sent' }).first();
    await expect(sentBadge).toBeVisible();
  });
});
