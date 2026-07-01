import { test, expect } from './fixtures';

test.describe('Autonomous Inventory, CRM, and Pricing Synchronization', () => {
  test('Priya views and approves a stockout reorder and price action', async ({ page, request }) => {
    // 1. Log in via magic test route
    await page.goto('/api/auth/test-login?tenant=tenant-priya&user=user-priya&role=owner');

    // 2. Clear existing approvals
    await request.post('/api/dev/reset-approvals?tenant_id=tenant-priya').catch(() => {});

    // 3. Simulate the stockout
    await request.post('/api/agents/approvals/simulate-stockout-reorder', {
      headers: {
        'x-tenant-id': 'tenant-priya',
        'x-user-id': 'user-priya',
      }
    });

    // 4. Navigate to dashboard feed
    await page.goto('/dashboard');
    await page.waitForLoadState('networkidle');

    // 5. Verify the card appears
    await expect(page.getByText('Urgent: Red Dress Stockout & Price Action')).toBeVisible({ timeout: 10000 });
    await expect(page.getByText('Red Dress sold out in 2 days')).toBeVisible();
    await expect(page.getByTestId('stockout-new-price')).toContainText('$46.00');
    await expect(page.getByTestId('stockout-reorder')).toContainText('50 Units');

    // 6. Click Approve
    const approveBtn = page.getByTestId('approve-stockout').first();
    await expect(approveBtn).toBeVisible();
    await approveBtn.click();

    // 7. Verify the card is removed from pending feed
    await expect(page.getByText('Urgent: Red Dress Stockout & Price Action')).not.toBeVisible();

    // 8. Go to activity tab and check it was approved
    const activityTab = page.getByTestId('tab-activity');
    if (await activityTab.isVisible()) {
        await activityTab.click();
        await page.waitForTimeout(1000);
        await expect(page.getByText('Urgent: Red Dress Stockout & Price Action')).toBeVisible();
        await expect(page.getByText('APPROVED')).toBeVisible();
    }
  });

  test('Priya views and approves an inventory reconciliation action', async ({ page, request }) => {
    await page.goto('/api/auth/test-login?tenant=tenant-priya&user=user-priya&role=owner');

    await request.post('/api/dev/reset-approvals?tenant_id=tenant-priya').catch(() => {});

    await request.post('/api/agents/approvals/simulate-inventory-reconciliation', {
      headers: {
        'x-tenant-id': 'tenant-priya',
        'x-user-id': 'user-priya',
      }
    });

    await page.goto('/dashboard');
    await page.waitForLoadState('networkidle');

    await expect(page.getByText('Inventory Reconciliation: Shopify Sync Issue')).toBeVisible({ timeout: 10000 });
    await expect(page.getByText('Inventory discrepancy detected on Shopify')).toBeVisible();

    const approveSyncBtn = page.getByTestId('approve-sync').first();
    await expect(approveSyncBtn).toBeVisible();
    await approveSyncBtn.click();

    await expect(page.getByText('Inventory Reconciliation: Shopify Sync Issue')).not.toBeVisible();

    const activityTab = page.getByTestId('tab-activity');
    if (await activityTab.isVisible()) {
        await activityTab.click();
        await page.waitForTimeout(1000);
        await expect(page.getByText('Inventory Reconciliation: Shopify Sync Issue')).toBeVisible();
        await expect(page.getByText('APPROVED')).toBeVisible();
    }
  });
});
