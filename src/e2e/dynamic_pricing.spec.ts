import { test, expect } from '@playwright/test';
import { adminPage } from './fixtures';
import { Client } from 'pg';

test.describe('Autonomous AI-Driven Dynamic Pricing Engine', () => {
  let db: Client;

  test.beforeAll(async () => {
    db = new Client({ connectionString: process.env.DATABASE_URL || 'postgres://ohc:ohc@localhost:5432/ohc' });
    await db.connect();
  });

  test.afterAll(async () => {
    await db.end();
  });

  test.beforeEach(async () => {
    // Insert a mock dynamic pricing recommendation feed item
    await db.query(`
      INSERT INTO agent_feed_items (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state, created_at, updated_at)
      VALUES (
        'dp-mock-1234',
        'e2e-tenant',
        'Pricing Agent',
        '{"type": "pricing_analysis"}'::jsonb,
        '{"feature_type": "dynamic_pricing", "type": "dynamic_pricing_recommendation", "target_id": "22222222-2222-2222-2222-222222222222", "target_title": "Summer Hats", "base_price_cents": 1000, "recommendation": "Summer Hats are moving slow. Suggest a 15% discount to clear out inventory.", "action": "create_rule", "rule_config": {"name": "Clearance", "type": "InventoryThreshold", "config": {"threshold": 10, "adjustment_percent": -15.0}}}'::jsonb,
        'PENDING_APPROVAL',
        CURRENT_TIMESTAMP,
        CURRENT_TIMESTAMP
      )
      ON CONFLICT (id) DO UPDATE SET lifecycle_state = 'PENDING_APPROVAL';
    `);
  });

  test('Owner can approve a dynamic pricing recommendation from the feed', async ({ browser }) => {
    const context = await browser.newContext();
    const page = await adminPage({ context, page: await context.newPage() });

    await page.goto('/');

    // Wait for the feed card to appear
    await expect(page.locator('text=✨ Pricing Suggestion')).toBeVisible();
    await expect(page.locator('text=Summer Hats')).toBeVisible();
    await expect(page.locator('text=Base price: $10.00')).toBeVisible();
    await expect(page.locator('text=Summer Hats are moving slow.')).toBeVisible();

    // Click the "Approve & Run Sale" button
    const approveBtn = page.locator('button:has-text("Approve & Run Sale")');
    await expect(approveBtn).toBeVisible();

    // In our legacy e2e flow or mock feed UI, sometimes we use the action_required endpoint
    // We can also click it directly.
    await approveBtn.click();

    // Wait for the card to disappear optimistically
    await expect(page.locator('text=✨ Pricing Suggestion')).toBeHidden();

    // Also verify it was inserted in the DB
    // wait a moment for the backend background worker to process it
    await page.waitForTimeout(2000);

    const rules = await db.query(`
      SELECT name, is_active FROM pricing_rules WHERE target_id = '22222222-2222-2222-2222-222222222222' AND tenant_id = 'e2e-tenant'
    `);
    expect(rules.rows.length).toBe(1);
    expect(rules.rows[0].name).toBe('Clearance');
    expect(rules.rows[0].is_active).toBe(true);

    await context.close();
  });
});
