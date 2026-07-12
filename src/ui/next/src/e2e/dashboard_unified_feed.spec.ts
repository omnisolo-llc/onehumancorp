import { test, expect } from '../../../../e2e/fixtures';

test.describe('Real-Time Multi-Tenant Edge Notifications & Sync via WebSocket', () => {
  test('Dashboard Unified Feed receives real-time approval_request event', async ({ page }) => {
    // Navigate to login
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill('test@example.com');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Log In' }).click();

    // Ensure dashboard loads
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });

    const tenantId = await page.evaluate(() => localStorage.getItem('tenant_id') || 'e2e-tenant');

    const uniqueId = `e2e-ws-test-${Date.now()}`;

    const resCreate = await page.request.post(`/api/agent-feed?tenant_id=${tenantId}`, {
      headers: {
        'x-tenant-id': tenantId,
        'x-user-id': 'default'
      },
      data: {
        event_source: 'e2e-test',
        context_payload: { description: 'WS Real-Time Test Item' },
        proposed_action: { draft_reply: 'Yes, this is real-time!', action_type: uniqueId }
      }
    });

    expect(resCreate.ok()).toBeTruthy();
    const createdItem = await resCreate.json();

    const resUpdate = await page.request.put(`/api/agent-feed/${createdItem.id}/state`, {
      headers: {
        'x-tenant-id': tenantId,
        'x-user-id': 'default'
      },
      data: {
        state: 'APPROVED'
      }
    });

    expect(resUpdate.ok()).toBeTruthy();

    await expect(page.locator(`text=${uniqueId}`)).toBeVisible({ timeout: 10000 });
  });
});
