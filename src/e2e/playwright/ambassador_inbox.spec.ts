import { test, expect } from '@playwright/test';

test.describe('Ambassador Agent Workflow in Unified Inbox', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('simulate ambassador draft, verify in inbox, and approve', async ({ page, request }) => {
    const tenantId = 'test-ambassador-tenant';
    const identifier = '+15555551234';
    const messageContent = 'Do you have vegan chocolate cake available for Saturday?';

    // 1. Seed the database
    await request.post('/api/v1/builder/seeder/exec', {
      data: {
        sql: `
          INSERT INTO users (id, email, full_name, is_superadmin)
          VALUES ('ambassador_user_id', 'ambassador@example.com', 'Ambassador User', false)
          ON CONFLICT DO NOTHING;

          INSERT INTO tenants (id, name, owner_email)
          VALUES ('${tenantId}', 'Ambassador Store', 'ambassador@example.com')
          ON CONFLICT DO NOTHING;

          INSERT INTO customers (id, tenant_id, name, email, phone)
          VALUES ('test_cust_ambassador', '${tenantId}', 'Test Ambassador Customer', 'ambassador_test@example.com', '${identifier}')
          ON CONFLICT DO NOTHING;
        `
      }
    });

    // 2. Post the webhook payload directly to the API
    const response = await request.post('/api/v1/omnichannel/webhook', {
      data: {
        tenant_id: tenantId,
        channel: 'instagram_dm',
        sender_id: identifier,
        message: messageContent
      }
    });

    expect(response.ok()).toBeTruthy();

    // 3. Wait for background processing (message_triage job)
    await page.waitForTimeout(4000);

    // 4. Login and navigate to inbox
    await page.goto(`/login?test_email=ambassador@example.com`);
    await page.evaluate((t) => localStorage.setItem('tenant', t), tenantId);

    await page.goto('/inbox');

    // Ensure we are in the mobile view
    const bodyBox = await page.locator('body').boundingBox();
    expect(bodyBox?.width).toBeLessThanOrEqual(375);

    // 5. Verify the conversation appears in the unified inbox queue
    const messageItem = page.locator('.app-list-item', { hasText: messageContent }).first();
    await expect(messageItem).toBeVisible({ timeout: 10000 });

    // 6. Click on the conversation to view details
    await messageItem.click();

    // 7. Verify Conversation Details Panel
    const detailPanel = page.locator('.app-panel').nth(1); // The second panel
    await expect(detailPanel).toContainText('Conversation Detail');
    await expect(detailPanel).toContainText(messageContent);
    await expect(detailPanel).toContainText('Draft Reply');

    // 8. Find and interact with the Approve & Send button
    const approveButton = detailPanel.getByTestId('feed-approve-btn');
    await expect(approveButton).toBeVisible();

    // Ensure the button has a min 44x44 bounding box (mobile touch target standard)
    const box = await approveButton.boundingBox();
    expect(box).not.toBeNull();
    if (box) {
      expect(box.width).toBeGreaterThanOrEqual(44);
      expect(box.height).toBeGreaterThanOrEqual(44);
    }

    // 9. Tap the button
    await approveButton.click();

    // 10. Wait for the status to show success and UI to settle
    await expect(page.locator('div[role="status"]')).toContainText('Draft approved and sent.', { timeout: 5000 });
  });
});
