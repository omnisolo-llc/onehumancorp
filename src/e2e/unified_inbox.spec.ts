import { test, expect } from '@playwright/test';

test.describe('Unified Multimodal Autonomous Customer Support Engine', () => {
  test('should display messages in the unified inbox UI', async ({ page, request }) => {
    // We would normally set up DB state or call the webhook directly here
    // For this e2e test, let's inject a message via the webhook endpoint
    const tenantId = 'test-tenant-' + Date.now();
    const webhookResponse = await request.post('/api/support/webhook', {
      data: {
        channel: 'ig',
        sender_id: 'customer_123',
        content: 'Do you have vegan cakes?',
        tenant_id: tenantId,
      }
    });

    expect(webhookResponse.status()).toBe(200);
    const body = await webhookResponse.json();
    expect(body.status).toBe('needs-review');
    expect(body.draft_reply).toContain('vegan');

    // In a real scenario we'd navigate to the unified inbox page in the Next.js app
    await page.goto('/inbox/unified');
    await expect(page.locator('text=customer_123')).toBeVisible();
    await expect(page.locator('text=Do you have vegan cakes?')).toBeVisible();
  });
});
