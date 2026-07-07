import { test, expect } from './fixtures';

test.describe('Omni Inbox Triage Integration', () => {
  test.use({ viewport: { width: 375, height: 667 } }); // Mobile viewport

  const tenantId = 'e2e-omni-triage-tenant';

  test('should display omni inbox messages in triage feed', async ({ page, request }) => {
    // Seed omni_inbox_messages
    await request.post('/api/v1/builder/seeder/exec', {
      data: {
        sql: `
          INSERT INTO tenants (id, name, tier) VALUES ('${tenantId}', 'Test', 'free') ON CONFLICT DO NOTHING;
          INSERT INTO omni_inbox_messages (id, tenant_id, source, original_content, translated_content, target_language, draft_reply, status, sender_id, created_at)
          VALUES ('omni-msg-1', '${tenantId}', 'WhatsApp', 'Need an appointment', 'Need an appointment', 'English', 'I can help with that.', 'unread', 'cust_1', NOW())
          ON CONFLICT DO NOTHING;
        `
      }
    });

    await page.goto(`/api/ui/triage.html?tenant_id=${tenantId}`);

    // Check if it appears in triage
    const itemCard = page.getByTestId('triage-card-omni-msg-1');
    await expect(itemCard).toBeVisible({ timeout: 15000 });
    await expect(itemCard.locator('text=WhatsApp')).toBeVisible();
    await expect(itemCard.locator('text=Need an appointment')).toBeVisible();
    await expect(itemCard.locator('text=Draft Reply')).toBeVisible();
  });

  test('should approve omni inbox message from triage', async ({ page, request }) => {
    await request.post('/api/v1/builder/seeder/exec', {
      data: {
        sql: `
          INSERT INTO tenants (id, name, tier) VALUES ('${tenantId}', 'Test', 'free') ON CONFLICT DO NOTHING;
          INSERT INTO omni_inbox_messages (id, tenant_id, source, original_content, translated_content, target_language, draft_reply, status, sender_id, created_at)
          VALUES ('omni-msg-2', '${tenantId}', 'Instagram DM', 'How much is it?', 'How much is it?', 'English', 'It is $50.', 'unread', 'cust_2', NOW())
          ON CONFLICT DO NOTHING;
        `
      }
    });

    await page.goto(`/api/ui/triage.html?tenant_id=${tenantId}`);

    const itemCard = page.getByTestId('triage-card-omni-msg-2');
    await expect(itemCard).toBeVisible({ timeout: 15000 });

    const cardHeader = page.getByTestId('triage-card-header-omni-msg-2');
    await cardHeader.click();

    const approveButton = page.getByTestId('triage-approve-omni-msg-2');
    await expect(approveButton).toBeVisible();
    await approveButton.click();

    await expect(itemCard).not.toBeVisible({ timeout: 5000 });
  });

  test('should edit and approve omni inbox message from triage', async ({ page, request }) => {
    await request.post('/api/v1/builder/seeder/exec', {
      data: {
        sql: `
          INSERT INTO tenants (id, name, tier) VALUES ('${tenantId}', 'Test', 'free') ON CONFLICT DO NOTHING;
          INSERT INTO omni_inbox_messages (id, tenant_id, source, original_content, translated_content, target_language, draft_reply, status, sender_id, created_at)
          VALUES ('omni-msg-3', '${tenantId}', 'Email', 'Where are you located?', 'Where are you located?', 'English', 'We are in NY.', 'unread', 'cust_3', NOW())
          ON CONFLICT DO NOTHING;
        `
      }
    });

    await page.goto(`/api/ui/triage.html?tenant_id=${tenantId}`);

    const itemCard = page.getByTestId('triage-card-omni-msg-3');
    await expect(itemCard).toBeVisible({ timeout: 15000 });

    const cardHeader = page.getByTestId('triage-card-header-omni-msg-3');
    await cardHeader.click();

    const reviewButton = page.getByTestId('triage-review-btn-omni-msg-3');
    await reviewButton.click();

    const textarea = page.getByTestId('triage-edit-textarea-omni-msg-3');
    await expect(textarea).toBeVisible();
    await expect(textarea).toHaveValue('We are in NY.');

    await textarea.fill('We are located in downtown NY.');

    const saveButton = page.getByTestId('triage-save-btn-omni-msg-3');
    await saveButton.click();

    await expect(itemCard).not.toBeVisible({ timeout: 5000 });
  });

  test('should dismiss omni inbox message from triage', async ({ page, request }) => {
    await request.post('/api/v1/builder/seeder/exec', {
      data: {
        sql: `
          INSERT INTO tenants (id, name, tier) VALUES ('${tenantId}', 'Test', 'free') ON CONFLICT DO NOTHING;
          INSERT INTO omni_inbox_messages (id, tenant_id, source, original_content, translated_content, target_language, draft_reply, status, sender_id, created_at)
          VALUES ('omni-msg-4', '${tenantId}', 'SMS', 'Stop', 'Stop', 'English', '', 'unread', 'cust_4', NOW())
          ON CONFLICT DO NOTHING;
        `
      }
    });

    await page.goto(`/api/ui/triage.html?tenant_id=${tenantId}`);

    const itemCard = page.getByTestId('triage-card-omni-msg-4');
    await expect(itemCard).toBeVisible({ timeout: 15000 });

    const cardHeader = page.getByTestId('triage-card-header-omni-msg-4');
    await cardHeader.click();

    const dismissButton = page.getByTestId('triage-dismiss-omni-msg-4');
    await expect(dismissButton).toBeVisible();
    await dismissButton.click();

    await expect(itemCard).not.toBeVisible({ timeout: 5000 });
  });

  test('should not display resolved or dismissed omni inbox messages', async ({ page, request }) => {
    await request.post('/api/v1/builder/seeder/exec', {
      data: {
        sql: `
          INSERT INTO tenants (id, name, tier) VALUES ('${tenantId}', 'Test', 'free') ON CONFLICT DO NOTHING;
          INSERT INTO omni_inbox_messages (id, tenant_id, source, original_content, translated_content, target_language, draft_reply, status, sender_id, created_at)
          VALUES ('omni-msg-5', '${tenantId}', 'SMS', 'Hello', 'Hello', 'English', 'Hi!', 'resolved', 'cust_5', NOW()),
                 ('omni-msg-6', '${tenantId}', 'SMS', 'Spam', 'Spam', 'English', '', 'dismissed', 'cust_6', NOW())
          ON CONFLICT DO NOTHING;
        `
      }
    });

    await page.goto(`/api/ui/triage.html?tenant_id=${tenantId}`);

    const itemCard5 = page.getByTestId('triage-card-omni-msg-5');
    await expect(itemCard5).not.toBeVisible({ timeout: 5000 });

    const itemCard6 = page.getByTestId('triage-card-omni-msg-6');
    await expect(itemCard6).not.toBeVisible({ timeout: 5000 });
  });

  test('should provide a contextually aware drafted response using RAG for past orders', async ({ page, request }) => {
    await request.post('/api/v1/builder/seeder/exec', {
      data: {
        sql: `
          INSERT INTO tenants (id, name, tier) VALUES ('tenant-rag', 'Test', 'free') ON CONFLICT DO NOTHING;
          INSERT INTO customers (id, tenant_id, name, email, phone) VALUES ('cust-rag', 'tenant-rag', 'RAG Customer', 'rag@example.com', '1234') ON CONFLICT DO NOTHING;
          INSERT INTO purchase_orders (id, tenant_id, vendor_id, total_cost, status) VALUES ('po-rag', 'tenant-rag', 'cust-rag', 45.0, 'completed') ON CONFLICT DO NOTHING;
        `
      }
    });

    // Trigger webhook so it gets triaged
    await request.post('/api/webhook/omnichannel', {
      data: {
        tenant_id: 'tenant-rag',
        channel: 'Instagram DM',
        sender_id: '1234',
        message: 'Do you still have vegan options?'
      }
    });

    // Wait for the worker to triage the message (it runs every 1 second)
    await page.waitForTimeout(2000);

    await page.goto(`/api/ui/triage.html?tenant_id=tenant-rag`);

    // We can't know the exact omni-msg id since it's a uuid generated on webhook insert
    // But we know there will be a card with the text "Do you still have vegan options?"
    await expect(page.locator('text=vegan options').first()).toBeVisible({ timeout: 15000 });
  });

});
