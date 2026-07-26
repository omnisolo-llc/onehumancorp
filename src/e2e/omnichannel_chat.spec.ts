import { test, expect } from '@playwright/test';
import { v4 as uuidv4 } from 'uuid';
import { currentAppSmoke } from './smoke';
import { e2eDbQuery } from './e2e_db_util';

test.describe('Native Rust Omnichannel Chat UI', () => {
  const tenantId = `e2e-omni-${uuidv4()}`;
  const inboxId = uuidv4();
  const contactId = uuidv4();
  const conversationId = uuidv4();
  const messageId = uuidv4();

  test.beforeAll(async ({ request }) => {
    // We will bypass DB seeding with raw SQL via the webhook that mimics it.
    // Instead of dealing with DB credentials we can post to the API.
    const webhookRes = await request.post('/api/v1/inbox/webhook', {
      data: {
        tenant_id: tenantId,
        source: "WhatsApp",
        message: "Can you fix my sink?",
        sender_id: "+1234567890",
        target_language: "English"
      }
    });

    // We don't assert 200 immediately here because we might need to handle the DB directly
    // since the webhook creates its own IDs.

    // Fallback: seed database with e2e_db_util
    await e2eDbQuery(`INSERT INTO tenants (id, name, ceo_name) VALUES ('${tenantId}', 'Carlos Handyman', 'Carlos') ON CONFLICT DO NOTHING;`);
    await e2eDbQuery(`INSERT INTO inboxes (id, tenant_id, name, channel_type) VALUES ('${inboxId}', '${tenantId}', 'Main', 'WhatsApp');`);
    await e2eDbQuery(`INSERT INTO contacts (id, tenant_id, name, phone) VALUES ('${contactId}', '${tenantId}', 'Jane Customer', '+1234567890');`);
    await e2eDbQuery(`INSERT INTO conversations (id, tenant_id, inbox_id, contact_id, status) VALUES ('${conversationId}', '${tenantId}', '${inboxId}', '${contactId}', 'open');`);
    await e2eDbQuery(`INSERT INTO messages (id, tenant_id, conversation_id, content, sender_type) VALUES ('${messageId}', '${tenantId}', '${conversationId}', 'Can you fix my sink?', '+1234567890');`);

  });

  test('Unified Inbox Desktop View Loads Conversations', async ({ page, request, loginAs }) => {
    await loginAs(tenantId, 'owner@ohc.local', 'password');

    // Set viewport to desktop
    await page.setViewportSize({ width: 1440, height: 900 });

    await page.goto('/inbox');
    await expect(page.locator('text=Unified Inbox')).toBeVisible();

    // Verify conversation list is visible
    await expect(page.locator('text=Active Conversations')).toBeVisible();
    await expect(page.locator('text=WhatsApp')).toBeVisible();

    // Click conversation
    await page.locator('text=WhatsApp').click();

    // Verify detail pane
    await expect(page.locator('text=Can you fix my sink?')).toBeVisible();
    await expect(page.locator('text=Known Customer')).toBeVisible();
  });

  test('Mobile View 375px Has No Horizontal Scroll and Works Properly', async ({ page, loginAs }) => {
    await loginAs(tenantId, 'owner@ohc.local', 'password');

    // 1. Mobile-First Non-Negotiable: Set viewport to 375px
    await page.setViewportSize({ width: 375, height: 667 });

    await page.goto('/inbox');

    // On mobile, the list should be visible, and detail hidden initially
    await expect(page.locator('text=Active Conversations')).toBeVisible();

    // 2. No Horizontal Scroll Check
    const scrollWidth = await page.evaluate(() => document.documentElement.scrollWidth);
    const clientWidth = await page.evaluate(() => document.documentElement.clientWidth);
    expect(scrollWidth).toBeLessThanOrEqual(clientWidth + 5); // Allow minor sub-pixel variations

    // 3. Click conversation
    await page.locator('text=WhatsApp').first().click();

    // Detail view opens
    await expect(page.locator('text=Can you fix my sink?')).toBeVisible();
    await expect(page.locator('text=Known Customer')).toBeVisible();

    // We should see a Back button in the header
    const backBtn = page.locator('button', { hasText: 'Back' });
    await expect(backBtn).toBeVisible();

    // The Active Conversations list is now hidden (mobile layout)
    await expect(page.locator('text=Active Conversations')).toBeHidden();

    // Go back
    await backBtn.click();
    await expect(page.locator('text=Active Conversations')).toBeVisible();
  });
});
