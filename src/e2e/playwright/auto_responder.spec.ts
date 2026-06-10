import { test, expect } from '@playwright/test';
import { v4 as uuidv4 } from 'uuid';

test.describe('Intelligent Customer Auto-Responder CUJ', () => {
  const tenantId = 'e2e-test-tenant-' + uuidv4();

  test.beforeEach(async ({ page }) => {
    // 1. Setup tenant context in localStorage (mock login/session)
    await page.goto('/');
    await page.evaluate((id) => {
      localStorage.setItem('tenant_id', id);
      localStorage.setItem('user_name', 'E2E Owner');
    }, tenantId);
  });

  test('should handle a new message autonomously and show AI Handled badge', async ({ page, request }) => {
    // 2. Simulate Incoming Webhook from Meta (Instagram)
    const webhookResponse = await request.post('/api/v1/webhooks/meta', {
      data: {
        entry: [{
          messaging: [{
            sender: { id: 'customer-123' },
            recipient: { id: tenantId },
            message: { text: 'Do you have sourdough bread today?' }
          }]
        }]
      }
    });
    expect(webhookResponse.ok()).toBeTruthy();

    // 3. Navigate to Dashboard and verify "Auto-Replied" metric
    await page.goto('/dashboard');
    // Poll for the metric to update (AI worker takes a second)
    await expect(page.locator('.app-metric-label:has-text("Auto-Replied") + .app-metric-value')).not.toHaveText('0', { timeout: 10000 });

    // 4. Navigate to Inbox and verify "AI Handled" badge
    await page.goto('/inbox');
    const aiBadge = page.locator('.app-badge:has-text("AI Handled")');
    await expect(aiBadge).toBeVisible({ timeout: 5000 });

    // 5. Verify conversation detail shows the draft/reply
    await page.click('#messages-list button');
    await expect(page.locator('.app-metric-label:has-text("Draft Reply") + div')).not.toContainText('No draft reply stored');
  });
});
