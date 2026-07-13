import { test, expect } from '../../../../e2e/fixtures';

test.describe('Ambassador Auto-Responder CUJ', () => {
  test('Owner sees AI Handled auto-replied message in inbox', async ({ page, request }) => {
    // 1. Connect Instagram via Integrations
    // Start from login to satisfy the rules
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill('test@example.com');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Log In' }).click();
    await expect(page.getByRole('heading', { name: 'Dashboard' }).first()).toBeVisible();

    // Set configuration for auto-reply in backend if possible, or trigger auto-reply
    const tenantId = 'test-tenant';

    // 2. Trigger the Ambassador's draft reply via a real API call (no mocks)
    // The CustomerSuccess agent listens for tenant.message.received, which is triggered via the webhook endpoint
    const webhookPayload = {
      tenant_id: tenantId,
      sender_id: 'testuser',
      message: 'I would like to place an order.',
      source: 'instagram'
    };

    const apiBase = process.env.OHC_API_URL || process.env.BACKEND_URL || process.env.BASE_URL || '';
    const response = await request.post(`${apiBase}/api/inbox/webhook`, {
      data: webhookPayload,
    });

    expect(response.ok()).toBeTruthy();

    // 3. Wait for background task to execute
    // In our test environment, we wait for a moment so the worker pool handles it
    await page.waitForTimeout(2000);

    // 4. Check Inbox Page
    await page.goto('/inbox');
    await expect(page.getByRole('heading', { name: 'Inbox' })).toBeVisible();

    // Verify "AI Handled" badge shows up
    const messageLocator = page.locator('.app-list-item', { hasText: 'I would like to place an order.' }).first();
    await expect(messageLocator).toBeVisible({ timeout: 5000 });

    // Click it
    await messageLocator.click();

    // Verify detail shows AI Handled
    await expect(page.locator('.app-panel-body .app-badge', { hasText: 'AI Handled' })).toBeVisible();
  });
});
