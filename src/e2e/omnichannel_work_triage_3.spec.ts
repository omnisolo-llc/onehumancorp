import { test, expect } from '@playwright/test';

test.describe('Omnichannel Work Triage CUJ - SMS', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('Maya can see SMS messages', async ({ page }) => {
    test.setTimeout(180000);

    // 1. Log in
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill('test@example.com');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Log In' }).click();
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });

    const tenantId = await page.evaluate(() => localStorage.getItem('tenant_id') || 'e2e-tenant');

    const seedData = [
      {
        source: 'SMS',
        priority: 'high',
        context: 'Message: Customer wants to cancel order.',
        action_type: 'Cancel Order',
        action_payload: 'Canceling order #123.',
        customer_id: 'maya_sms_cust'
      }
    ];

    for (const data of seedData) {
      await page.request.post(`/api/v1/triage/create?tenant_id=${encodeURIComponent(tenantId)}`, {
        data
      });
    }

    await page.goto('/triage');
    await expect(page.locator('body')).toContainText(/Work Triage/, { timeout: 15000 });

    const card = page.locator('div[data-testid^="triage-card-"]', { hasText: 'SMS' }).first();
    await expect(card).toBeVisible();
  });
});
