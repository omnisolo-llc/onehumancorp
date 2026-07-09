import { test, expect } from '@playwright/test';

test.describe('Real-Time Multi-Tenant Edge Notifications & Sync', () => {
  test('should receive real-time notifications via sync gateway when an event is triggered', async ({ page, request }) => {
    const tenantId = 'test_tenant_e2e_notifications_' + Date.now();

    // Create a mock token
    const token = await request.post('/api/auth/token', {
      data: {
        organization_id: tenantId,
        agent_id: 'test_agent',
      }
    }).then(r => r.ok() ? r.json() : null); // We mock the auth headers or use a mock JWT, depending on your auth setup

    // For E2E tests, we often use test mode headers
    await page.goto('/dashboard');
    await page.evaluate((tid) => {
      localStorage.setItem('tenant_id', tid);
    }, tenantId);
    await page.reload();

    // Trigger an offline sync that causes a ToggleSoldOut which triggers the backend notification
    const res = await request.post('/api/v1/sync/events', {
      headers: {
        'x-tenant-id': tenantId,
        'x-mock-auth': 'true',
        'x-spiffe-id': `spiffe://onehumancorp.io/org/${tenantId}/agent/test_agent`,
      },
      data: {
        events: [
          {
            id: 'sync-evt-1',
            entity_type: 'product',
            entity_id: 'prod-test-1',
            action_type: 'ToggleSoldOut',
            payload: { is_sold_out: true },
            base_version: 0
          }
        ]
      }
    });

    expect(res.ok()).toBeTruthy();

    const notification = page.locator('div[role="alert"]', { hasText: 'Synced 1 updates' });
    await expect(notification).toBeVisible({ timeout: 10000 });
  });
});
