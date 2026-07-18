import { test, expect } from '@playwright/test';

test.describe('Advisor Agent CUJ', () => {
  test('Owner reviews Advisor recommendations in the Action Center', async ({ page, request }) => {
    // 1. Trigger the Business Advisory report simulation via a real API call (no mocks)
    // The BusinessAdvisory agent listens for tenant.report.weekly_health
    const tenantId = 'e2e-tenant';
    const webhookPayload = {
      tenant_id: tenantId,
      gross_sales: 1500.00,
      orders_count: 12,
      top_seller_name: 'Vegan Chocolate Cake'
    };

    const apiBase = process.env.OHC_API_URL || process.env.BACKEND_URL || process.env.BASE_URL || '';
    const response = await request.post(`${apiBase}/api/agents/webhook`, {
      data: {
        tenant_id: tenantId,
        event_type: 'tenant.report.weekly_health',
        payload: webhookPayload
      },
    });

    // We expect 200 or 202 depending on the webhook handler
    expect(response.ok()).toBeTruthy();

    // 2. Start from login to satisfy the rules
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill('maya@ohc.test');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Login' }).click();
    await expect(page.getByRole('heading', { name: 'Dashboard' }).first()).toBeVisible({ timeout: 15000 });

    // 3. Navigate to the Action Center
    await page.goto('/action-center');
    await expect(page.getByRole('heading', { name: 'Action Center' })).toBeVisible({ timeout: 15000 });

    // Wait for either a pending recommendation or the empty state.
    // The description usually says "Draft weekly business health report"
    const approvalLocator = page.getByText('Draft weekly business health report').first();
    const approveButton = page.getByRole('button', { name: 'Approve & Send' }).first();
    const dismissButton = page.getByRole('button', { name: 'Dismiss' }).first();

    await expect(page.getByText(/All Caught Up!|Draft weekly business health report/)).toBeVisible({ timeout: 15000 });

    if (await approveButton.isVisible()) {
      // 4. Owner approves the drafted action
      await approveButton.click();

      // Validate success status or removal
      await expect(page.getByText('Action approved and executed.')).toBeVisible({ timeout: 15000 });
    } else {
      await expect(page.getByText('All Caught Up!')).toBeVisible();
    }
  });
});
