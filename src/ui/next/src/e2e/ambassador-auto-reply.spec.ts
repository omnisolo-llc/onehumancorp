import { test, expect } from '../../../../e2e/fixtures';

test.describe('Ambassador Auto-Responder CUJ', () => {
  test('Owner sees drafted message in inbox and approves it', async ({ page, request, loginAs, adminUser }) => {
    // 1. Log in
    await loginAs(page, adminUser);
    await expect(page.getByRole('heading', { name: 'Dashboard' }).first()).toBeVisible({ timeout: 15000 });

    const tenantId = 'e2e-tenant';

    // 2. Trigger the Ambassador's draft reply via a real API call (no mocks)
    const webhookPayload = {
      tenant_id: tenantId,
      sender_id: 'testuser',
      message: 'I would like to place an order.',
      source: 'instagram'
    };

    const apiBase = process.env.OHC_API_URL || process.env.BACKEND_URL || process.env.BASE_URL || '';
    const response = await request.post(`${apiBase}/api/v1/inbox/webhook`, {
      data: webhookPayload,
    });

    expect(response.ok()).toBeTruthy();

    // 3. Wait for background task to execute
    await page.waitForTimeout(3000);

    // 4. Check Inbox Page
    await page.goto('/inbox');
    await expect(page.getByRole('heading', { name: 'Unified Inbox' })).toBeVisible({ timeout: 15000 });

    // Find the message in the queue
    const messageLocator = page.locator('.app-list-item', { hasText: 'I would like to place an order.' }).first();
    await expect(messageLocator).toBeVisible({ timeout: 10000 });

    // Click it to view details
    await messageLocator.click();

    // Verify detail shows Draft Ready or a Send button
    const approveBtn = page.getByRole('button', { name: /.*Approve.*Send.*Draft.*/ });
    await expect(approveBtn).toBeVisible({ timeout: 10000 });

    // Approve the draft
    await approveBtn.click();

    // Verify success message
    await expect(page.getByText('Draft approved and sent.')).toBeVisible({ timeout: 5000 });
  });
});
