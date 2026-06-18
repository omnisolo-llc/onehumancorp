import { test, expect } from '@playwright/test';

test.describe('Omnichannel Intake Agent feed card', () => {
  test.use({ viewport: { width: 375, height: 667 } }); // strictly mobile viewport

  test('should process a webhook, triage, and allow approval on mobile feed', async ({ page, request }) => {
    const tenantId = 'omni_test_tenant_' + Date.now();
    const customerPhone = '+15555551234';

    // Seed the database
    await request.post('/api/v1/builder/seeder/exec', {
      data: {
        sql: `
          INSERT INTO users (id, email, full_name, is_superadmin)
          VALUES ('omni_user_id', 'omni_user@example.com', 'Omni User', false)
          ON CONFLICT DO NOTHING;

          INSERT INTO tenants (id, name, owner_email)
          VALUES ('${tenantId}', 'Omni Store', 'omni_user@example.com')
          ON CONFLICT DO NOTHING;

          INSERT INTO customers (id, tenant_id, name, email, phone)
          VALUES ('test_cust_1', '${tenantId}', 'Test Omnichannel Customer', 'omni@example.com', '${customerPhone}')
          ON CONFLICT DO NOTHING;
        `
      }
    });

    // 2. Post the webhook payload directly to the API
    const response = await page.request.post('/api/v1/omnichannel/webhook', {
      data: {
        tenant_id: tenantId,
        channel: 'instagram',
        sender_id: customerPhone,
        message: 'Hello, what is the status of my order?'
      }
    });

    expect(response.status()).toBe(200);

    // Wait a brief moment for the background worker to process triage
    await page.waitForTimeout(3000);

    // Navigate to dashboard where feed is shown
    await page.goto(`/login?test_email=omni_user@example.com`);
    await page.evaluate((t) => localStorage.setItem('tenant', t), tenantId);
    await page.goto('/dashboard');

    // Wait for Action Required section to be visible
    const actionPanel = page.locator('.app-panel', { hasText: 'Action Required' });
    await expect(actionPanel).toBeVisible({ timeout: 15000 });

    const approvalCard = actionPanel.locator('.app-list-item', { hasText: 'Action Required: Approve Reply' });
    await expect(approvalCard).toBeVisible({ timeout: 15000 });

    // Verify mobile constraints
    const bodyBox = await page.locator('body').boundingBox();
    expect(bodyBox?.width).toBeLessThanOrEqual(375);

    // Check if the drafted reply is visible
    await expect(approvalCard.getByText('Hello, what is the status of my order?')).toBeVisible({ timeout: 15000 });
    await expect(approvalCard.getByText('AI Draft')).toBeVisible();

    // Approve the response
    const approveButton = approvalCard.getByRole('button', { name: /Send Draft/ }).first();
    await expect(approveButton).toBeVisible();

    // Ensure the button has a min 44x44 bounding box
    const box = await approveButton.boundingBox();
    expect(box).not.toBeNull();
    if (box) {
      expect(box.width).toBeGreaterThanOrEqual(44);
      expect(box.height).toBeGreaterThanOrEqual(44);
    }

    await approveButton.click();

    // Verify it disappears
    await expect(approvalCard).not.toBeVisible({ timeout: 10000 });
  });
});
