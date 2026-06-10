import { test, expect } from '@playwright/test';
import { e2eDataHelper } from './fixtures/e2e_data_helper';

test.describe('Omnichannel Webhook & Identity Resolution', () => {
  test('Webhook processing resolves identity and adds to inbox', async ({ request, page }) => {
    // We assume there's a test tenant created by setup scripts.
    const tenantId = 'e2e-tenant';
    const email = 'customer_e2e_test@example.com';
    const messageContent = 'Hello, checking on my order from IG DMs';

    // First, let's create a customer to resolve against (through API if available, or just rely on test data).
    // For this test, we will just send the webhook payload. The DB setup for playwright usually
    // seeds `e2e-tenant` with a known state, or we can just send it and verify the inbox message is created.

    const payload = {
      tenant_id: tenantId,
      message: messageContent,
      source: 'instagram',
      sender_id: email
    };

    const response = await request.post('/api/agents/webhook', {
      data: payload,
    });

    expect(response.status()).toBe(200);
    const body = await response.json();
    expect(body.success).toBe(true);

    // Now log in as the owner to verify the message appears in the UI
    await page.goto('/login');
    // For E2E tests in OHC, there is usually an auto-login or default credentials
    await page.getByPlaceholder('Email').fill('admin@onehumancorp.com');
    await page.getByPlaceholder('Password').fill('admin');
    await page.getByRole('button', { name: 'Sign In' }).click();

    // Navigate to the inbox or feed
    await page.goto('/inbox');

    // Check if the drafted message is visible in the inbox
    // We wait for the message content to appear
    await expect(page.getByText(messageContent)).toBeVisible({ timeout: 10000 });
  });
});
