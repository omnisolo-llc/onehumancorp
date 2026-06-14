import { test, expect } from '@playwright/test';

test.describe('OHC Unified Inbox & AI Communication Auto-Drafting', () => {
  const tenantId = `tenant-omni-inbox-${Date.now()}`;
  const senderId = '@cake_lover';
  const source = 'instagram';
  const messageContent = 'Do you have vegan chocolate cakes available for Saturday?';

  test('should ingest message via webhook, triage, and allow approval', async ({ request, page }) => {
    // 1. Simulate webhook payload to backend
    const webhookResponse = await request.post('/api/v1/omnichannel/webhook', {
      data: {
        tenant_id: tenantId,
        source: source,
        sender_id: senderId,
        message: messageContent
      },
      headers: {
        'Content-Type': 'application/json'
      }
    });

    expect(webhookResponse.ok()).toBeTruthy();
    const result = await webhookResponse.json();
    expect(result.success).toBeTruthy();

    // Wait for async workers (triage, LLM draft) to process the queue
    await page.waitForTimeout(5000);

    // 2. Set tenant in UI and navigate to Unified Inbox
    await page.goto('/');
    await page.evaluate((id) => {
      localStorage.setItem('tenant', id);
      localStorage.setItem('tenant_id', id);
      // Simulate auth token for API calls
      localStorage.setItem('token', 'e2e-test-token');
    }, tenantId);

    await page.goto('/inbox');

    // 3. Verify the message is loaded in the PowerSync/frontend queue
    // The Inbox UI shows the Message Queue list
    const messageLocator = page.locator('#messages-list').filter({ hasText: messageContent });
    await expect(messageLocator).toBeVisible({ timeout: 10000 });

    // 4. Click the message to view details
    await messageLocator.click();

    // Verify details panel shows the source, sender, message, and an AI drafted reply
    await expect(page.locator('.app-panel-body').filter({ hasText: senderId })).toBeVisible();
    await expect(page.locator('.app-panel-body').filter({ hasText: source })).toBeVisible();
    await expect(page.locator('.app-panel-body').filter({ hasText: messageContent })).toBeVisible();

    // Check that a draft reply was generated (not empty fallback)
    const draftReplyContainer = page.locator('.app-panel-body').filter({ hasText: 'Draft Reply' });
    await expect(draftReplyContainer.filter({ hasText: 'No draft reply stored' })).not.toBeVisible();

    // 5. Click the Approve & Send button
    const approveButton = page.locator('button', { hasText: 'Approve & Send Draft' });
    await expect(approveButton).toBeVisible();

    await approveButton.click();

    // Verify successful approval feedback
    await expect(page.locator('.app-badge', { hasText: 'Draft approved and sent.' })).toBeVisible({ timeout: 5000 });
  });
});
