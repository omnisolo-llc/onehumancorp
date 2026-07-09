import { test, expect } from '@playwright/test';

test.describe('Negotiator Agent Quote and Deposit E2E', () => {
  const tenantId = 'ohc_e2e_tenant_' + Date.now();

  test.beforeEach(async ({ page }) => {
    // 1. Simulate user login
    await page.goto('/login');
    await page.fill('input[type="email"]', `e2e_owner_${Date.now()}@example.com`);
    await page.fill('input[type="password"]', 'testpassword123');
    await page.click('button[type="submit"]');
    await page.waitForURL('/dashboard');
  });

  test('Negotiator Agent generates a quote and creates a feed item', async ({ request, page }) => {
    // 2. Simulate an incoming message via webhook
    const incomingMessage = {
      event_type: 'message.received',
      tenant_id: tenantId,
      payload: {
        message: 'I need a ceiling fan installed.',
        sender_id: '1234567890',
        customer_id: 'cust_001',
        inbox_message_id: 'msg_001'
      }
    };

    const response = await request.post('/api/v1/webhooks/omni', {
      data: incomingMessage
    });

    expect(response.ok()).toBeTruthy();

    // 3. Wait for agent to process and generate feed item on the dashboard
    await page.goto('/dashboard');

    // We expect the negotiator agent to create an auto execute action, which creates a feed card showing the booked status.
    await expect(page.locator('text="Draft quote and propose schedule for Service"').first()).toBeVisible({ timeout: 15000 });

    // 4. Verify the details in the feed card
    const card = page.locator('.agent-feed-card').filter({ hasText: 'Draft quote and propose schedule for Service' }).first();
    await expect(card).toBeVisible();
    await expect(card.locator('text="$120"')).toBeVisible(); // default heuristic price
  });
});
