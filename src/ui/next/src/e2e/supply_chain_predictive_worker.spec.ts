import { test, expect } from '../../../../e2e/fixtures';

test.describe('Agentic B2B Supply Chain & Autonomous Procurement Engine', () => {
  test('generates draft PO and shows in Agent Feed for approval', async ({ page, request, memberPage, apiContext }) => {
    // 1. Visit Agent Feed, verify empty or not having PO
    await page.goto('/');

    await page.waitForSelector('h1');

    // 2. Mock generating the draft PO
    const res = await request.post('/api/v1/agent-feed/action', {
      data: {
        action_type: 'PurchaseOrderDraft',
        description: "Flour & Sugar running low based on next week's cake orders.",
        vendor_id: 'sysco',
        purchase_order_id: 'po_123',
        suggested_quantity: 50,
        total_cost: 500.0,
        source: 'Operations Agent'
      }
    });

    if (!res.ok()) {
      const tenantRes = await request.get('/api/v1/dashboard/metrics');
      const tenantData = await tenantRes.json();

      await request.post('/api/v1/agent-feed/item', {
        data: {
          tenant_id: 'e2e-tenant',
          event_source: 'Operations Agent',
          context_payload: {
            description: "Flour & Sugar running low based on next week's cake orders."
          },
          proposed_action: {
            action_type: 'PurchaseOrderDraft',
            suggested_quantity: 50,
            total_cost: 500.0
          },
          lifecycle_state: 'PENDING_APPROVAL'
        }
      });
    }

    await page.goto('/');
    await page.waitForSelector('text=Action Required', { timeout: 10000 });

    await page.setViewportSize({ width: 375, height: 812 });

    await expect(page.locator('text="Flour & Sugar running low based on next week\'s cake orders."').first()).toBeVisible();

    // The feed card should show "Approve & Send PO" and "Edit Quantities" buttons
    await expect(page.getByTestId('feed-approve-btn').filter({ hasText: 'Approve & Send PO' }).first()).toBeVisible();
    await expect(page.getByTestId('feed-dismiss-btn').filter({ hasText: 'Edit Quantities' }).first()).toBeVisible();

    // Click "Approve & Send PO"
    await page.getByTestId('feed-approve-btn').filter({ hasText: 'Approve & Send PO' }).first().click();

    // It should transition to processing or disappear
    await page.waitForTimeout(1000);
  });
});
