import { test, expect } from '@playwright/test';

test.describe('Intelligent Customer Auto-Responder', () => {
  let uniqueTenantId: string;
  let testInboxMessageId: string;

  test.beforeAll(async () => {
    uniqueTenantId = `autoresponder_tenant_${Date.now()}`;
  });

  test('should enqueue background job, process message, and show AI Handled in UI', async ({ request, page }) => {
    // 1. Simulate a webhook event which creates the inbox message and enqueues the job.
    // In our implementation, the twilio webhook handles this:
    const twilioPayload = new URLSearchParams();
    twilioPayload.append('From', 'whatsapp:+1234567890');
    twilioPayload.append('To', 'whatsapp:+0987654321');
    twilioPayload.append('Body', 'Where is my order?');

    // Actually we'll hit the meta webhook because it accepts JSON, wait... no, the twilio webhook parses URL encoded!
    // But since the twilio webhook currently hardcodes "test_tenant", we should probably use the meta webhook which allows some control over tenant id via recipient, wait, meta webhook hardcodes "test_tenant" too!
    // Let's use the local API if possible or just use the UI /api/v1/webhooks/twilio.
    // Wait, the webhooks are mounted at /api/v1/webhooks/... which the Playwright test can hit.

    // Instead of fighting the hardcoded tenant, let's login as "test_tenant" in the UI.
    const loginRes = await request.post('/api/auth/login', {
      data: {
        email: 'owner@test_tenant.com',
        password: 'password123',
      }
    });
    // For OHC, usually we just set the local storage tenant_id

    await page.goto('/login');
    // Set tenant manually since auth might be mocked
    await page.evaluate(() => {
      localStorage.setItem('tenant_id', 'test_tenant');
    });

    const response = await request.post('/api/v1/webhooks/twilio', {
      headers: {
        'Content-Type': 'application/x-www-form-urlencoded',
      },
      data: twilioPayload.toString(),
    });
    expect(response.status()).toBe(200);

    // 2. Wait for background task to process the job.
    // In real time, the background worker polls and updates the `inbox_messages` table status to 'ai_handled' and sets `draft_reply`.
    // We navigate to Inbox UI.
    await page.goto('/inbox');

    // 3. Verify the message is shown as AI Handled.
    // We might need to wait for the UI to fetch or for the background job to finish.
    // Click refresh a few times or use polling.
    await expect(page.locator('.app-list-item').filter({ hasText: 'Where is my order?' })).toBeVisible({ timeout: 15000 });

    // Click the item
    await page.locator('.app-list-item').filter({ hasText: 'Where is my order?' }).click();

    // Check if it got AI Handled.
    await expect(page.locator('.app-badge.magic', { hasText: '✨ AI Handled' })).toBeVisible({ timeout: 15000 });

    // Check if the drafted reply is rendered
    await expect(page.locator('.app-panel-body')).toContainText('Thanks for reaching out! We will review this and get back to you soon.');
  });
});
