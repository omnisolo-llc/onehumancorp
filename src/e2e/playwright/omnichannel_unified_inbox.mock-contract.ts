import { test, expect } from '@playwright/test';

test.describe('Unified Inbox & Agentic Triage', () => {
  const tenantId = 'test-unified-inbox-tenant';

  test('should display actionable cards for incoming messages and allow approval', async ({ request, page }) => {
    // 1. Simulate incoming message via webhook
    const identifier = 'maya_insta';
    const messageContent = 'Do you have vegan cupcakes for this Saturday?';

    const webhookRes = await request.post('/api/v1/omnichannel/webhook', {
      data: {
        tenant_id: tenantId,
        channel: 'instagram_dm',
        sender_id: identifier,
        message: messageContent
      }
    });

    expect(webhookRes.ok()).toBeTruthy();

    // 2. Wait for background processing
    await page.waitForTimeout(4000);

    // 3. Login and navigate to dashboard
    await page.goto(`/login`);
    await page.fill('input[name="tenant_id"]', tenantId);
    await page.fill('input[name="password"]', 'admin');
    await page.click('button[type="submit"]');

    await page.waitForURL('**/dashboard**');

    // 4. Verify feed item exists
    const messageContext = page.locator(`text="${messageContent}"`).first();
    await expect(messageContext).toBeVisible({ timeout: 10000 });

    const approveButton = page.locator('button:has-text("Resolve Message"), button:has-text("Approve")').first();
    await expect(approveButton).toBeVisible();

    // 5. Approve the action
    await approveButton.click();

    // Wait for the action to complete
    await page.waitForTimeout(1000);
  });
});
