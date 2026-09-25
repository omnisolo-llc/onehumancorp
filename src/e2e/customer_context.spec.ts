import { test, expect } from './fixtures';
import { db } from './db_utils';

test.describe('Omnichannel Unified Customer Memory Graph UI', () => {
  // Use loginAs fixture which handles authentication
  test('displays customer context in the Ambassador Reply Card', async ({ page, loginAs, adminUser }) => {
    await db.query(`
      INSERT INTO agent_feed_items (
        id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state, created_at, updated_at
      )
      VALUES (
        'e2e-feed-ambassador-reply',
        'e2e-tenant',
        'Ambassador',
        '{"feature_type":"ambassador_reply","source":"Instagram","past_orders":"Returning Customer (2 past orders).","context_used":"Customer prefers vegan options.","original_message":"Do you have vegan options?"}'::jsonb,
        '{"feature_type":"ambassador_reply","action_type":"DraftForReview","source":"Instagram","past_orders":"Returning Customer (2 past orders).","context_used":"Customer prefers vegan options.","original_message":"Do you have vegan options?","generated_response":"Yes! We have a full vegan pastry selection."}'::jsonb,
        'PENDING_APPROVAL',
        CURRENT_TIMESTAMP,
        CURRENT_TIMESTAMP
      )
      ON CONFLICT (id) DO UPDATE SET
        lifecycle_state = 'PENDING_APPROVAL',
        context_payload = EXCLUDED.context_payload,
        proposed_action = EXCLUDED.proposed_action,
        created_at = CURRENT_TIMESTAMP,
        updated_at = CURRENT_TIMESTAMP;
    `);

    await loginAs(page, adminUser);
    // Navigate to dashboard
    await page.goto('/dashboard');

    // Wait for the unified agent feed to load
    await expect(page.locator('#unified-agent-feed-section')).toBeVisible();

    // Verify the ambassador reply card is rendered (seeded in e2e-seed.sql)
    const card = page.locator('[data-testid="ambassador-reply-card"]').first();
    await expect(card).toBeVisible({ timeout: 15000 });

    // Check if the context section is rendered
    await expect(card.locator('text=Customer Context')).toBeVisible();

    // Check past orders is rendered properly
    await expect(card.locator('text=Returning Customer (2 past orders).')).toBeVisible();

    // Check context is rendered properly
    await expect(card.locator('text=Customer prefers vegan options.')).toBeVisible();
  });
});
