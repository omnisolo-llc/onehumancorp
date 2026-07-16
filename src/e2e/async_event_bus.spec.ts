import { test, expect } from './fixtures';

test.describe('Universal Event Bus and Async Queue', () => {
  test('handles concurrent incoming webhooks via Event Router', async ({ page, request }) => {
    // 1. Merchant views their dashboard to confirm baseline
    await page.goto('/dashboard');
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 15000 });

    // 2. We trigger the server-side action for concurrent incoming webhooks
    const triggerPromises = [];
    for (let i = 0; i < 3; i++) {
        triggerPromises.push(request.post('/api/v1/omnichannel/webhook', {
            data: {
                message_id: `test-message-${i}`,
                source: "instagram",
                sender_id: `test-sender-${i}`,
                customer_id: `test-customer-e2e`,
                message: `Can I order a custom cake ${i}?`
            }
        }));
    }

    const responses = await Promise.all(triggerPromises);
    for (const res of responses) {
        expect(res.ok()).toBeTruthy();
    }

    // Wait for the action to log and be processed by the queue/router/agent
    await page.waitForTimeout(2000);

    // 3. Navigate to triage or unified feed where the merchant can see the action log
    await page.goto('/triage');

    // The feed should mention the 3 cake inquiries
    for (let i = 0; i < 3; i++) {
        await expect(page.locator('body')).toContainText(`Can I order a custom cake ${i}?`, { timeout: 15000 });
    }
  });
});
