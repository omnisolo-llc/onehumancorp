import { test, expect } from '@playwright/test';

test.describe('Unified Triage Workflow', () => {
  test('ingests webhook and allows UI triage approval', async ({ page, request }) => {
    // 1. Send simulated webhook to populate feed
    const webhookRes = await request.post('http://127.0.0.1:18789/api/v1/webhooks/triage_ingest?tenant_id=ohc', {
      data: {
        customer_id: 'test-user',
        channel: 'instagram',
        content: 'How much for a cake?',
      },
      headers: {
        'Content-Type': 'application/json'
      }
    });

    expect(webhookRes.ok()).toBeTruthy();

    // 2. Navigate to UI
    await page.goto('/login');
    await page.fill('input[type="email"]', 'owner@example.com');
    await page.fill('input[type="password"]', 'password');
    await page.click('button[type="submit"]');

    // Make sure we are logged in (wait for dashboard or similar)
    await page.waitForURL('**/dashboard**');

    // 3. Go to Triage page
    await page.goto('/triage');

    // 4. Verify Triage Item appears
    await expect(page.locator('text=How much for a cake?').first()).toBeVisible({ timeout: 10000 });

    // 5. Click Approve
    const approveBtn = page.locator('[data-testid="approve-btn"]').first();
    await expect(approveBtn).toBeVisible();
    await approveBtn.click();

    // 6. Verify success toast
    await expect(page.locator('#action-status')).toHaveText(/Approved/i);
  });
});
