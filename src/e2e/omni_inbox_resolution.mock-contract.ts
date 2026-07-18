import { test, expect } from './fixtures';
import { e2eDbQuery } from './db_utils';

test.describe('Omnichannel Inbox Identity Resolution', () => {
  test('receives webhook, resolves identity, and shows in feed', async ({ page }) => {
    const tenantId = 'omni-test-tenant-' + Date.now();
    const customerPhone = '+15555551234';

    // 1. Set up a dummy customer to match against
    await e2eDbQuery(`
      INSERT INTO customers (id, tenant_id, name, email, phone)
      VALUES ('test-customer-1', '${tenantId}', 'Test Omnichannel Customer', 'omni@example.com', '${customerPhone}')
    `);

    // Setup an initial user session to match the tenant
    await e2eDbQuery(`
        INSERT INTO users (id, email, full_name, is_superadmin)
        VALUES ('omni-user-1', 'omni-user@example.com', 'Omni User', false)
    `);

    await e2eDbQuery(`
        INSERT INTO tenants (id, name, owner_email)
        VALUES ('${tenantId}', 'Omni Store', 'omni-user@example.com')
    `);

    // We'll use the browser login to switch contexts.
    // Simulate login by setting localStorage (depending on the auth flow in E2E setup).
    // Or just fetch the webhook directly.

    // 2. Post the webhook payload directly to the API
    const response = await page.request.post('/api/v1/webhooks/omnichannel', {
      data: {
        tenant_id: tenantId,
        channel: 'whatsapp',
        sender_id: customerPhone,
        message: 'Hello, what is the status of my order?'
      }
    });

    expect(response.status()).toBe(200);
    const result = await response.json();
    expect(result.success).toBe(true);

    // 3. Verify in database that identity was cached
    const identities = await e2eDbQuery(`SELECT * FROM customer_identities WHERE tenant_id = '${tenantId}'`);
    expect(identities.length).toBeGreaterThan(0);
    expect(identities[0].customer_id).toBe('test-customer-1');

    // 4. Verify message in database
    const messages = await e2eDbQuery(`SELECT * FROM inbox_messages WHERE tenant_id = '${tenantId}'`);
    expect(messages.length).toBeGreaterThan(0);
    expect(messages[0].source).toBe('whatsapp');
    expect(messages[0].original_content).toBe('Hello, what is the status of my order?');

    // 5. Navigate to the inbox page and verify UI
    // We will simulate logging in as this user for the UI test
    await page.goto(`/login?test_email=omni-user@example.com`);

    await page.goto('/inbox');

    // Wait for feed to load
    await expect(page.getByText('Hello, what is the status of my order?')).toBeVisible({ timeout: 10000 });
  });
});
