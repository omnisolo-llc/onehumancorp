import { test as base, expect } from './fixtures';
import { e2eDbQuery } from './db_utils';

const test = base.extend({
  page: async ({ page }, use) => {
    // Mobile viewport
    await page.setViewportSize({ width: 375, height: 812 });
    await use(page);
  }
});

test.describe('Automated Cart Recovery via Agents', () => {
  test('should process abandoned cart and generate feed item', async ({ adminUser, loginAs, page }) => {
    test.setTimeout(75_000);
    const tenantId = adminUser.organizationId;
    const sessionId = 'session-' + Date.now();
    const customerId = 'cust-' + Date.now();

    // Setup abandoned cart in DB directly
    await e2eDbQuery(`
        INSERT INTO customers (id, tenant_id, name, email, phone, created_at, updated_at)
        VALUES ($1, $2, 'Alice Recovered', 'alice.test@example.com', NULL, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
        ON CONFLICT DO NOTHING
    `, [customerId, tenantId]);

    await e2eDbQuery(`
        INSERT INTO conversational_checkout_sessions (id, tenant_id, customer_id, type, amount, status, created_at, updated_at)
        VALUES ($1, $2, $3, 'full', 4500, 'pending', CURRENT_TIMESTAMP - INTERVAL '2 hours', CURRENT_TIMESTAMP - INTERVAL '2 hours')
    `, [sessionId, tenantId, customerId]);

    // The owned native harness uses a 30-second scan and 5-second dispatch
    // cadence. Assert the actual persisted outcome, not an unrelated sales row
    // or a warning followed by a less informative UI timeout.
    const recoveredItems = () => e2eDbQuery(`
        SELECT * FROM agent_feed_items
        WHERE tenant_id = $1 AND event_source = 'sales'
          AND context_payload ->> 'checkout_session_id' = $2
    `, [tenantId, sessionId]);
    await expect.poll(recoveredItems, {
      message: 'The real cart worker must persist exactly one review item for this checkout',
      timeout: 45_000,
      intervals: [500],
    }).toHaveLength(1);
    const [item] = await recoveredItems();
    expect(item.lifecycle_state).toBe('PENDING_APPROVAL');
    expect(item.proposed_action.description).toContain('abandoned cart');

    // Verify it in the UI
    await loginAs(page, adminUser);
    await page.goto('/unified-feed');

    // Wait for the feed item to load
    await expect(page.getByText('abandoned cart').first()).toBeVisible({ timeout: 15000 });
  });
});
